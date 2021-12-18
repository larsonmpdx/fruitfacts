use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use anyhow::Result;
use serde::{Deserialize, Serialize};




use oauth2::{basic::BasicClient, revocation::StandardRevocableToken, TokenResponse};
// Alternatively, this can be oauth2::curl::http_client or a custom.
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
//use url::Url;







#[derive(Debug, Deserialize)]
struct AuthQuery {
    state: Option<String>,
    code: Option<String>,
    scope: Option<String>,
    authuser: Option<String>,
    prompt: Option<String>,
  //  session_state: Option<String>,
  //  hd: Option<String>,
}


#[get("/authURLs")]
async fn get_auth_URLs(
    query: web::Query<AuthQuery>,
  //  pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
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
    // This example will be running its own server at localhost:8080.
    // See below for the server implementation.
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
    )
    // Google supports OAuth 2.0 Token Revocation (RFC-7009)
    .set_revocation_uri(
        RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .expect("Invalid revocation endpoint URL"),
    );

    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the "calendar" features and the user's profile.
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/plus.me".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );


    // todo - put google url under some json or something
    Ok(HttpResponse::Ok().json(""))

}






#[get("/authRedirect")]
async fn receive_oauth_redirect(
    query: web::Query<AuthQuery>,
  //  pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    // let conn = pool.get().expect("couldn't get db connection from pool");

    // todo -
    // verify token
    // look up user in database
    // create or refresh user info
    // issue cookie (and make sure cookie security is good)
    // redirect to a post-login page


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
    //   &redirect_uri=http://localhost
    //   &state=[** STATE **]'

    // there are further steps to an oauth flow but we don't care because we just needed to verify the account and then transition
    // to our own server's account plus a cookie

    Ok(HttpResponse::Ok().json(""))
}
