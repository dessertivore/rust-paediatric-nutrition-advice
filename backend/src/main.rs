use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{get, post},
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool};
use std::net::SocketAddr;

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

#[derive(Serialize)]
struct UserResponse {
    id: i32,
    name: String,
    birthday: String,
    gender: String,
}

async fn create_user(Json(payload): Json<CreateUser>) -> Result<Json<UserResponse>, StatusCode> {
    let database_url = "postgres://test_user:test_password@localhost:5432/rust-nutrition";
    let pool = PgPool::connect(database_url).await;
    let birthday: NaiveDate = NaiveDate::parse_from_str(&payload.birthday, "%Y-%m-%d")
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let name: String = payload.name;
    let gender: String = payload.gender;
    // // Insert the user into the database
    // let row = sqlx::query!(
    //     r#"
    //     INSERT INTO users (name, birthday, gender)
    //     VALUES ($1, $2, $3)
    //     RETURNING id, name, birthday, gender
    //     "#,
    //     name,
    //     birthday,
    //     gender
    // )
    // .fetch_one(&pool)
    // .await
    // .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return the created user as a response
    Ok(Json(UserResponse {
        id: 1,
        name: "test".to_string(),
        birthday: "1/1/10".to_string(),
        gender: "test".to_string(),
    }))
}
