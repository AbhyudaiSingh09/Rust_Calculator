
#[macro_use] extern crate rocket;
use rocket::{launch, routes, Rocket, Build};
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket_cors::{CorsOptions, AllowedOrigins};


#[derive(Deserialize)]
struct CalcParams {
    num1: f32,
    num2: f32,
    operator: String,
}

#[derive(Deserialize)]
struct BitwiseParams {
    num1: i64,
    num2: i64,
    operator: String,
}

#[derive(Deserialize)]
struct NegateParams {
    num1: i8,
}

#[derive(Serialize)]
struct CalcResult<T> {
    result: T,
}

#[post("/arithmetic", format = "json", data = "<params>")]
fn arithmetic_calculator(params: Json<CalcParams>) -> Json<CalcResult<f32>> {
    let result = match params.operator.as_str() {
        "+" => params.num1 + params.num2,
        "-" => params.num1 - params.num2,
        "*" => params.num1 * params.num2,
        "/" => if params.num2 != 0.0 { params.num1 / params.num2 } else { 0.0 }, // Consider proper error handling
        "%" => params.num1 % params.num2,
        _ => 0.0, // Consider proper error handling
    };

    Json(CalcResult { result })
}

#[post("/bitwise", format = "json", data = "<params>")]
fn bitwise_calculator(params: Json<BitwiseParams>) -> Json<CalcResult<i64>> {
    let result = match params.operator.as_str() {
        "&" => params.num1 & params.num2,
        "|" => params.num1 | params.num2,
        "^" => params.num1 ^ params.num2,
        _ => 0, // Consider proper error handling
    };

    Json(CalcResult { result })
}

#[post("/negate", format = "json", data = "<params>")]
fn negate_calculator(params: Json<NegateParams>) -> Json<CalcResult<i8>> {
    Json(CalcResult { result: !params.num1 })
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the calculator API!"
}




fn rocket() -> Rocket<Build> {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::All)
        .allow_credentials(true)
        .to_cors().expect("error creating CORS fairing");

    rocket::build()
        .attach(cors)
        .mount("/", routes![index, arithmetic_calculator, bitwise_calculator, negate_calculator])
}