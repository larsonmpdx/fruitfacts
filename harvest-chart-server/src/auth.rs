use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use anyhow::Result;
use oauth2::basic::{BasicErrorResponseType, BasicTokenType};
use serde::{Deserialize};
use std::io::Read;

use actix_session::{Session};
use rand::Rng;

use oauth2::reqwest::http_client;
use oauth2::{basic::BasicClient, revocation::StandardRevocableToken, TokenResponse};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope,
    StandardTokenIntrospectionResponse, TokenUrl,
};
use std::env;

// todo - session cookie first, then csrf storage and retreival in some kind of cache with expiration
// https://security.stackexchange.com/questions/20187/oauth2-cross-site-request-forgery-and-state-parameter

use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex;

use expiring_map::ExpiringMap;
use std::time::Duration;

#[derive(Debug)]
struct OAuthVerificationInfo {
    pkce_code_verifier: Option<PkceCodeVerifier>, // Option<> because the library author is our mom and doesn't want us reusing this
    csrf_state: CsrfToken,
}

fn get_google_client() -> oauth2::Client<
    oauth2::StandardErrorResponse<BasicErrorResponseType>,
    oauth2::StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
> {
    let google_client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable"),
    );
    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET")
            .expect("Missing the GOOGLE_CLIENT_SECRET environment variable"),
    );

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    // Set up the config for the Google OAuth2 process
    BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://fruitfacts.xyz:8080/authRedirect".to_string())
            .expect("Invalid redirect URL"),
    )
    // Google supports OAuth 2.0 Token Revocation (RFC-7009)
    .set_revocation_uri(
        RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .expect("Invalid revocation endpoint URL"),
    )
}

static OAUTH_INFO: Lazy<Mutex<ExpiringMap<String, OAuthVerificationInfo>>> =
    Lazy::new(|| Mutex::new(ExpiringMap::new(Duration::from_secs(60))));

#[get("/authURLs")]
async fn get_auth_URLs(
    session: Session, //  pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let session_value;
    if let Some(value) = session.get::<String>("key")? {
        // todo - could we just use the session key that actix-session creates? all we need is a random number.  I couldn't find an API to access this key though
        println!("existing session value: {}", value);
        session_value = value;
    } else {
        // set a random session
        session_value = base64::encode(rand::thread_rng().gen::<[u8; 32]>());
        println!("setting new session value: {}", session_value);
        session.set::<String>("key", session_value.clone()).unwrap();
    }

    let client = get_google_client();

    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // see https://developers.google.com/identity/protocols/oauth2/scopes#oauth2
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    // save oauth info to verify the redirect query string that comes back
    OAUTH_INFO.lock().unwrap().insert(
        session_value,
        OAuthVerificationInfo {
            pkce_code_verifier: Some(pkce_code_verifier),
            csrf_state,
        },
    );

    // todo - put google url under some json or something
    Ok(HttpResponse::Ok().json(authorize_url))
}

#[derive(Debug, Deserialize)]
struct GoogleAuthQuery {
    state: Option<String>,
    code: Option<String>,
    scope: Option<String>,
    authuser: Option<String>,
    prompt: Option<String>,
    //  session_state: Option<String>,
    //  hd: Option<String>,
}

#[get("/authRedirect")]
async fn receive_oauth_redirect(
    query: web::Query<GoogleAuthQuery>,
    session: Session,
    //  pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    // let conn = pool.get().expect("couldn't get db connection from pool");

    // we receive:
    // AuthQuery {
    // state: Some("0eacf4d808124f15a4e127aba2e8b017"),
    // code: Some("4/0AX4XfWjOjzN8G7iHcwUym44ARWcTJiOlU6UKrgrfVvhIUw9lJCRe3PYx_wDu2nN9wOJ4Dg"),
    // scope: Some("email profile openid https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email"),
    // authuser: Some("0"),
    // prompt: Some("consent")
    // }
    println!("{:?}", query);

    // todo -
    // verify token
    // look up user in database
    // create or refresh user info
    // issue cookie (and make sure cookie security is good)
    // redirect to a post-login page

    if let Some(session_value) = session.get::<String>("key")? {
        println!("SESSION value: {}", session_value);

        if let Some(oauth_info) = OAUTH_INFO.lock().unwrap().get_mut(&session_value) {
            if let (Some(code), Some(state)) = (&query.code, &query.state) {
                let code = AuthorizationCode::new(code.clone());
                let state = CsrfToken::new(state.clone());

                println!("Google returned the following code:\n{}\n", code.secret());
                println!(
                    "Google returned the following state:\n{} (expected `{}`)\n",
                    state.secret(),
                    oauth_info.csrf_state.secret()
                );

                // todo - make sure we drop whatever we need to unlock OAUTH_INFO

                let client = get_google_client();

                // get ownership out of the Option<>
                let verifier = std::mem::replace(&mut oauth_info.pkce_code_verifier, None);

                // todo - test whether the extracted verifier is Some()

                let token_response = client
                    .exchange_code(code)
                    .set_pkce_verifier(verifier.unwrap())
                    .request(http_client);

                println!("token response: {:#?}", token_response);
                let token_secret = token_response.unwrap().access_token().secret().clone();

                println!("token: {:?}", token_secret);

                // todo - get user's info using this token

                let mut resp = reqwest::blocking::get(format!(
                    "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
                    token_secret
                ))
                .unwrap();

                let mut body = String::new();
                resp.read_to_string(&mut body)?;

                println!("{:#?} body: {}", resp, body);

                //  {
                //  "id": "numbers...", -> won't change, primary key
                //  "email": "email@gmail.com", -> could change, still store it
                //  "verified_email": true, -> require this to be true
                //  "picture": "https://lh3.googleusercontent.com/a/default-user=s96-c"
                //  }

                // todo - move all of this sync stuff out of the handler here so we don't hold up actix
            } else {
                println!("query string didn't have code and state");
            }
        } else {
            println!(
                "didn't find oauth value with session value {}",
                session_value
            );
        }
    } else {
        println!("didn't find session value");
    }

    // https://stackoverflow.com/questions/60060323/google-oauth-2-0-refresh-access-token-and-new-refresh-token
    // curl "https://www.googleapis.com/oauth2/v4/token" \
    // --request POST \
    // --silent \
    // --data 'grant_type=authorization_code
    //   &code=[** AUTH CODE **]
    //   &client_id=[** CLIENT_ID **]
    //   &client_secret=[** CLIENT_SECRET **]
    //   &redirect_uri=http://fruitfacts.xyz
    //   &state=[** STATE **]'

    // there are further steps to an oauth flow but we don't care because we just needed to verify the account and then transition
    // to our own server's account plus a cookie

    Ok(HttpResponse::Ok().json(""))
}
