use super::schema_generated::base_plants;
use actix_web::{get, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct BasePlantsItemForPatents {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub uspp_number: Option<String>,
    pub uspp_expiration: Option<i64>,
}

pub fn get_recent_patents_db(
    conn: &SqliteConnection,
) -> Result<Vec<BasePlantsItemForPatents>, diesel::result::Error> {
    base_plants::dsl::base_plants
        .select((
            base_plants::name,
            base_plants::type_,
            base_plants::uspp_number,
            base_plants::uspp_expiration,
        ))
        .filter(base_plants::uspp_expiration.is_not_null())
        .order(base_plants::uspp_expiration.desc())
        .load::<BasePlantsItemForPatents>(conn)
}

#[get("/patents")]
async fn get_recent_patents(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let patents = web::block(move || get_recent_patents_db(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(patents))
}
