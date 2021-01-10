use actix_web::{
    dev::HttpResponseBuilder, get, http::header, http::StatusCode, post, web, App, HttpResponse,
    HttpServer, Responder,
};
use std::sync::Mutex;

// This struct represents (not shared) state
// struct AppState {
//     app_name: String,
// }

// This struct is shared app state
struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

// #[get("/")]
// async fn index(data: web::Data<AppState>) -> String {
//     let app_name = &data.app_name; // <- get app_name

//     format!("Hello {}!", app_name) // <- response with app_name
// }

#[get("/")]
async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    println!("Running function `healthcheck`");
    HttpResponse::Ok().body("Status: Up!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("running `echo`");
    HttpResponse::Ok().body(req_body)
}

#[get("/error")]
async fn error_test() -> impl Responder {
    println!("Running function `error_test`");

    HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
        .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(format!(
            "Failed on purpose! showing you status code: {}",
            StatusCode::BAD_REQUEST
        ))

    // HttpResponse::Ok(error::ErrorBadRequest("Failing now!"))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server!");

    // setup shared state
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    // move counter into the closure
    HttpServer::new(move || {
        // HttpServer::new(|| { // NOT shared state
        App::new()
            // Note: using app_data instead of data
            .app_data(counter.clone()) // <- register the created data
            // .data(AppState { // NOT Shared State
            //     app_name: String::from("Actix-web"),
            // })
            // web::scope("/") // scopes apply to a path and all sub paths
            // .guard(guard::Header("Host", "www.rust-lang.org")) and guards can restrict access to the server via a boolean
            .service(index)
            .service(healthcheck)
            .service(echo)
            .service(error_test)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
