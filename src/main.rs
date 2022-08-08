use actix_web::{middleware, web, App, HttpRequest, HttpServer};

async fn index(req: HttpRequest) -> &'static str {
  println!("Request: {req:?}");
  "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let address = match std::env::var("NARO3_ADDRESS"){
    Ok(val) => val,
    Err(_e) => "localhost".to_string()
  };
  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .service(web::resource("/").to(index))
  })
  .bind((address, 8080))?
  .run()
  .await
}
