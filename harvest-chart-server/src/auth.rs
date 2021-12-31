use actix_web::{get, web, HttpResponse};
use anyhow::{anyhow, Result};
use oauth2::basic::{BasicErrorResponseType, BasicTokenType};
use serde::Deserialize;
use serde::Serialize;
use std::io::Read;

use expiring_map::ExpiringMap;
use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex;
use std::time::Duration;

use super::schema_generated::*;
use super::schema_types::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use actix_session::Session;
use rand::Rng;

use oauth2::reqwest::http_client;
use oauth2::{basic::BasicClient, revocation::StandardRevocableToken, TokenResponse};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, RedirectUrl, RevocationUrl, Scope, StandardTokenIntrospectionResponse,
    TokenUrl,
};
use std::env;

use crate::session;

// todo - session cookie first, then csrf storage and retreival in some kind of cache with expiration
// https://security.stackexchange.com/questions/20187/oauth2-cross-site-request-forgery-and-state-parameter

type GoogleClientType = oauth2::Client<
    oauth2::StandardErrorResponse<BasicErrorResponseType>,
    oauth2::StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
>;

fn get_google_client() -> GoogleClientType {
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

#[get("/authURLs")]
async fn get_auth_urls(
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
    session::insert_oauth_info(
        session_value,
        session::OAuthVerificationInfo {
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
    // scope: Option<String>,
    // authuser: Option<String>,
    // prompt: Option<String>,
    // session_state: Option<String>,
    // hd: Option<String>,
}

pub fn get_existing_oauth_entry_db(
    db_conn: &SqliteConnection,
    unique_id: String,
) -> Result<UserOauthEntry, diesel::result::Error> {
    user_oauth_entries::dsl::user_oauth_entries
        .filter(user_oauth_entries::unique_id.eq(unique_id))
        .order(user_oauth_entries::id.desc())
        .first::<UserOauthEntry>(db_conn)
}

pub fn get_existing_user_db(
    db_conn: &SqliteConnection,
    user_id: i32,
) -> Result<User, diesel::result::Error> {
    users::dsl::users
        .filter(users::id.eq(user_id))
        .order(users::id.desc())
        .first::<User>(db_conn)
}

#[derive(Clone)]
pub struct AccountOffer {
    used: bool,
    google_account_info: Option<GoogleAccountInfo>,
}

// if a user comes back from an external oauth redirect with a valid external account but no website account yet, offer to create a website account
// cache the offer for N minutes so that the user can hit the "ok, create" API
static ACCOUNT_OFFER_CACHE: Lazy<Mutex<ExpiringMap<String, AccountOffer>>> =
    Lazy::new(|| Mutex::new(ExpiringMap::new(Duration::from_secs(30 * 60))));

pub fn insert_account_offer(session_value: String, account_offer: AccountOffer) {
    ACCOUNT_OFFER_CACHE
        .lock()
        .unwrap()
        .insert(session_value, account_offer);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GoogleAccountInfo {
    id: String,           // won't change, primary key
    email: String,        // could change. store this anyway so we can greet the user correctly
    verified_email: bool, // require true
    picture: String,      // we don't use this
}

#[derive(Default, Serialize)]
pub struct ReceiveRedirectReturn {
    user: Option<User>, // give this back if we already have a matching user
    account_info: Option<GoogleAccountInfo>, // give this back if we don't have a user created yet (with account_offer = true)
    account_offer: bool,
}

fn receive_oauth_redirect_blocking(
    query: web::Query<GoogleAuthQuery>,
    session_value: String,
    db_conn: &SqliteConnection,
) -> Result<ReceiveRedirectReturn> {
    println!("{:?}", query);

    if query.code.is_none() || query.state.is_none() {
        return Err(anyhow!("missing query info"));
    }

    let code = AuthorizationCode::new(query.code.as_ref().unwrap().clone());
    let state = CsrfToken::new(query.state.as_ref().unwrap().clone());

    // will fail if we didn't have an existing oauth request that matches this redirect
    let (pkce_code_verifier, csrf_state) = session::get_oauth_info(&session_value).unwrap();

    println!(
        "oauth redirect returned the following code:\n{}\n",
        code.secret()
    );
    println!(
        "oauth redirect returned the following state:\n{} (expected `{}`)\n",
        state.secret(),
        csrf_state
    );

    let client = get_google_client();

    let token_response = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request(http_client);

    println!("token response: {:#?}", token_response);
    let token_secret = token_response.unwrap().access_token().secret().clone();

    println!("token: {:?}", token_secret);

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let mut resp = client
        .get(format!(
            "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
            token_secret
        ))
        .send()?;

    let mut body = String::new();
    let _ = resp.read_to_string(&mut body); // ignore errors, we'll catch them in json parsing

    let account_info: GoogleAccountInfo = serde_json::from_str(&body)?;

    println!("{:#?} body: {:#?}", resp, account_info);

    // todo -
    // - if we already have a user, associate this session to that user (database)
    // if account_info.id is in user_oauth_entries
    if let Ok(oauth_entry) =
        get_existing_oauth_entry_db(db_conn, format!("google:{}", account_info.id))
    {
        if let Ok(user) = get_existing_user_db(db_conn, oauth_entry.user_id) {
            session::store_session(
                db_conn,
                UserSessionToInsert {
                    user_id: user.id,
                    session_value,
                    created: chrono::Utc::now().timestamp(),
                },
            );
            return Ok(ReceiveRedirectReturn {
                user: Some(user),
                account_info: None,
                account_offer: false,
            });
        } else {
            // todo - have oauth entry but no user, should delete the oauth entry or print something
        }
    }

    // - if we don't have a user, put them into an account offer pool. timeout like 30 minutes (another singleton map)
    insert_account_offer(
        session_value,
        AccountOffer {
            google_account_info: Some(account_info.clone()),
            used: false,
        },
    );
    return Ok(ReceiveRedirectReturn {
        user: None,
        account_info: Some(account_info),
        account_offer: true,
    });
}

fn get_session_value(session: Session) -> Option<String> {
    let session_value = session.get::<String>("key");
    if session_value.is_err() {
        println!("didn't find session value");
        return None;
    }

    let session_value = session_value.unwrap();
    if session_value.is_none() {
        println!("empty session value");
        return None;
    }

    return session_value;
}

#[get("/authRedirect")]
async fn receive_oauth_redirect(
    query: web::Query<GoogleAuthQuery>,
    session: Session,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let session_value = get_session_value(session);
    let db_conn = pool.get().expect("couldn't get db connection from pool");

    let results = web::block(move || {
        receive_oauth_redirect_blocking(query, session_value.unwrap(), &db_conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    // redirect to a post-login page (either account offer or logged-in landing page)
    // todo: encode our account info somewhere, I guess in a query string in the redirect?
    if results.account_offer {
        Ok(HttpResponse::Found()
            .header("Location", "/createAccount")
            .finish())
    } else {
        Ok(HttpResponse::Found().header("Location", "/").finish())
    }
}

#[derive(Default, Serialize)]
pub struct CreateAccountReturn {
    user: Option<User>,
}

pub fn create_account_blocking(
    session_value: String,
    db_conn: &SqliteConnection,
) -> Result<CreateAccountReturn> {
    if let Some(offer) = ACCOUNT_OFFER_CACHE.lock().unwrap().get_mut(&session_value) {
        if offer.used {
            return Err(anyhow!("account offer already used"));
        }
        offer.used = true;
        // if found, create an account in the database

        let google_account = offer.google_account_info.as_ref().unwrap();
        let name = google_account.email.clone(); // todo - allow a customized username
                                                 // todo
        let rows_inserted = diesel::insert_into(users::dsl::users)
            .values((users::name.eq(name.clone()),))
            .execute(db_conn);

        if rows_inserted != Ok(1) {
            return Err(anyhow!("couldn't create account"));
        }

        // get the id of the newly-created user (sqlite can't return this from the creation query)
        // todo
        let new_user = users::dsl::users
            .filter(users::name.eq(name.clone()))
            .order(users::id.desc())
            .first::<User>(db_conn)?;

        let rows_inserted = diesel::insert_into(user_oauth_entries::dsl::user_oauth_entries)
            .values((
                user_oauth_entries::user_id.eq(new_user.id),
                user_oauth_entries::unique_id.eq(format!("google:{}", google_account.id)),
                user_oauth_entries::oauth_info.eq(serde_json::to_string(&google_account)?), // save this for display purposes or whatever
            ))
            .execute(db_conn);

        if rows_inserted != Ok(1) {
            return Err(anyhow!("couldn't create oauth entry")); // todo - a transaction to cover account + oauth entry creation
        }

        // set the user logged in
        session::store_session(
            db_conn,
            UserSessionToInsert {
                user_id: new_user.id,
                session_value,
                created: chrono::Utc::now().timestamp(),
            },
        );
        // return the user
        return Ok(CreateAccountReturn {
            user: Some(new_user),
        });
    } else {
        // no offer in the cache
        return Err(anyhow!("no account offer in cache"));
    }
}

// todo new APIs:
// - create account (after getting an account offer from an oauth redirect)
// - check account (front end calls this to see if the user is already logged in based on an existing session)
// - log out

#[get("/createAccount")]
async fn create_account(
    session: Session,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    // todo

    // todo - user gets to fill in other fields like nickname or whatever, maybe in the query string

    // look at account offer cache for this session
    let session_value = get_session_value(session);
    let db_conn = pool.get().expect("couldn't get db connection from pool");

    let _results = web::block(move || create_account_blocking(session_value.unwrap(), &db_conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    // todo - returns
    Ok(HttpResponse::InternalServerError().finish())
}

#[get("/checkLogin")]
async fn check_login(
    _session: Session,
    _pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    // todo

    // return account if this session is logged in
    Ok(HttpResponse::Ok().json(""))
}

#[get("/logout")]
async fn logout(
    _session: Session,
    _pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    // todo

    // remove session entry
    Ok(HttpResponse::Ok().json(""))
}
