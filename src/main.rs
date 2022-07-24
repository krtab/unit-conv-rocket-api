use rocket::{get, launch, post, routes, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
enum Unit {
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "kg")]
    Kg,
    #[serde(rename = "metric ton")]
    MTon,
}

impl Unit {
    const fn rate_to_grams(self) -> f64 {
        match self {
            Unit::Lb => 453.59237,
            Unit::G => 1.,
            Unit::Kg => 1_000.,
            Unit::MTon => 1_000_000.,
        }
    }
}

#[derive(Deserialize)]
struct ApiRequest {
    quantity: f64,
    unit_from: Unit,
    unit_to: Unit,
}

#[derive(Serialize)]
struct Response {
    quantity: f64,
    unit: Unit,
}

#[post("/", data = "<request>")]
fn api_convert(request: Json<ApiRequest>) -> Json<Response> {
    let rate = request.unit_from.rate_to_grams() / request.unit_to.rate_to_grams();
    Json(Response {
        quantity: request.quantity * rate,
        unit: request.unit_to,
    })
}

#[get("/")]
fn landing_page() -> &'static str {
    include_str!("../Readme.md")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![api_convert, landing_page])
}
