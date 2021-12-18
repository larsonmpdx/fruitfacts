use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct AuthQuery {
    state: Option<String>,
    code: Option<String>,
    scope: Option<String>,
    authuser: Option<String>,
    prompt: Option<String>,
}

#[get("/auth")]
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

    Ok(HttpResponse::Ok().json(""))
}
