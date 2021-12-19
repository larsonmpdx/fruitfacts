use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use actix_session::{CookieSession, Session};
use rand::Rng;

use oauth2::{basic::BasicClient, revocation::StandardRevocableToken, TokenResponse};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope, TokenUrl,
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
    pkce_code_verifier: PkceCodeVerifier,
    csrf_state: CsrfToken,
}

static OAUTH_INFO: Lazy<Mutex<ExpiringMap<String, OAuthVerificationInfo>>> =
    Lazy::new(|| Mutex::new(ExpiringMap::new(Duration::from_secs(60))));

#[get("/authURLs")]
async fn get_auth_URLs(
    session: Session, //  pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let session_value;
    if let Some(value) = session.get::<String>("key")? { // todo - could we just use the session key that actix-session creates? all we need is a random number.  I couldn't find an API to access this key though
        println!("existing session value: {}", value);
        session_value = value;
    } else {
        // set a random session
        session_value = base64::encode(rand::thread_rng().gen::<[u8; 32]>());
        println!("setting new session value: {}", session_value);
        session.set::<String>("key", session_value.clone()).unwrap();
    }

    let google_client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable."),
    );
    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET")
            .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    // Set up the config for the Google OAuth2 process.
    let client = BasicClient::new(
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
    );

    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // see https://developers.google.com/identity/protocols/oauth2/scopes#oauth2
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );

    // save oauth info
    // pkce_code_verifier, csrf_state
    OAUTH_INFO.lock().unwrap().insert(
        session_value,
        OAuthVerificationInfo {
            pkce_code_verifier,
            csrf_state,
        },
    );

    // todo - put google url under some json or something
    Ok(HttpResponse::Ok().json(""))
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

    // todo -
    // verify token
    // look up user in database
    // create or refresh user info
    // issue cookie (and make sure cookie security is good)
    // redirect to a post-login page

    if let Some(session_value) = session.get::<String>("key")? {
        println!("SESSION value: {}", session_value);

        if let Some(oauth_info) = OAUTH_INFO.lock().unwrap().get(&session_value) {
            if let (Some(code), Some(state)) = (&query.code, &query.state) {
                let code = AuthorizationCode::new(code.clone());
                let state = CsrfToken::new(state.clone());

                println!("Google returned the following code:\n{}\n", code.secret());
                println!(
                    "Google returned the following state:\n{} (expected `{}`)\n",
                    state.secret(),
                    oauth_info.csrf_state.secret()
                );

                // todo - etc.
            } else {
                println!("query string didn't have code and state");
            }
        } else {
            println!("didn't find oauth value with session value {}", session_value);
        }
    } else {
        println!("didn't find session value");
    }

    // we receive:
    // AuthQuery {
    // state: Some("0eacf4d808124f15a4e127aba2e8b017"),
    // code: Some("4/0AX4XfWjOjzN8G7iHcwUym44ARWcTJiOlU6UKrgrfVvhIUw9lJCRe3PYx_wDu2nN9wOJ4Dg"),
    // scope: Some("email profile openid https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email"),
    // authuser: Some("0"),
    // prompt: Some("consent")
    // }
    println!("{:?}", query);

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
