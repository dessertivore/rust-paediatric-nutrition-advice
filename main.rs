use axum::{Router, extract::Path, routing::get};
use tokio;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route(
            "/iron/{age_years}/{age_months}/{nutrient}",
            get(get_nutrient_rec),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_nutrient_rec(
    Path((age_years, age_months, nutrient)): Path<(u8, u8, String)>,
) -> impl axum::response::IntoResponse {
    let message = if age_years == 0 {
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
