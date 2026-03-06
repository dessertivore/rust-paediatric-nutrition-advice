use axum::{
    Router,
    extract::{Json, Path, State},
    routing::{get, post},
};
use backend::schema::child_profiles;
use core::time;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::sync::Arc;
use tokio;
// 0 = male
// 1 = female

#[tokio::main]
async fn main() {
    let app = Router::new()
        // `GET /` goes to `root`
        .route(
            "/nutrient/{age_years}/{age_months}/{nutrient}",
            get(get_nutrient_rec),
        )
        .route("/user", get(get_nutrient_rec));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_nutrient_rec(
    Path((age_years, age_months, nutrient)): Path<(u8, u8, String)>,
) -> impl axum::response::IntoResponse {
    let message: f32 = if age_years == 0 {
        match age_months {
            0..=3 => 1.7,
            4..=6 => 4.3,
            7..=9 => 7.8,
            10..=12 => 7.8,
            _ => 0.0, // Default case for other months
        }
    } else {
        panic!()
    };

    format!("For {}, the recommendation is: {}", nutrient, message)
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    birthday: String, // Use `String` for simplicity; you can parse it into a `chrono::NaiveDate` later
    gender: String,
}

#[derive(Clone, PartialEq, Serialize, Default)]
struct ChildProfileInput {
    name: String,
    birthday: String,
    gender: bool,
    weight: Option<f32>,
    height: Option<f32>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = backend::schema::child_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChildProfile {
    pub id: i32,
    pub name: String,
    pub birthday: chrono::NaiveDate,
    pub gender: bool,
    pub weight: Option<f32>,
    pub height: Option<f32>,
}

pub async fn create_child_profile(
    Json(profile): Json<ChildProfile>,
) -> Result<(), axum::http::StatusCode> {
    let conn = &mut establish_connection();
    diesel::insert_into(child_profiles::table)
        .values(&profile)
        .execute(conn)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
