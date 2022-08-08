use actix_web::{get, post, middleware, web, App, HttpRequest, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct JsonData{
  id: isize,
  name: String,
  arr: Vec<isize>,
}


#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
  println!("Request: {req:?}");
  "Hello, World!"
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
  format!("Hello, {name}!")
}

#[post("/post")]
async fn post(info: web::Json<JsonData>) -> impl Responder {
  let ret = serde_json::to_string(&info).unwrap();
  println!("id:{}, name:{}, all:\r\n{}", info.id, info.name, ret);
  ret
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
      .service(index)
      .service(hello)
      .service(post)
  })
  .bind((address, 8080))?
  .run()
  .await
}
