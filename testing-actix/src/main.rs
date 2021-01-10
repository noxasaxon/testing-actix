use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    println!("Running function `hello`");
    HttpResponse::Ok().body("Hello world!")
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

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server!");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(healthcheck)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
