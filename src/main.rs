use actix_web::{get, post, middleware, web, App, HttpRequest, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct JsonData{
  id: isize,
  name: String,
  arr: Vec<isize>,
}

#[derive(Deserialize)]
struct AddJsonData{
  left: isize,
  right: isize,
}

#[derive(Serialize)]
struct AnswerJsonData{
  answer: isize,
}

#[derive(Deserialize)]
struct FizzbuzzQuery{
  #[serde(default = "default_fizzbuzzquery_count")]
  count: isize,
}
fn default_fizzbuzzquery_count() -> isize {
  10
}


#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
  println!("Request: {req:?}");
  "Hello, World!"
}


#[get("ping")]
async fn ping() -> impl Responder {
  "pong"
}

#[get("fizzbuzz")]
async fn fizzbuzz(info: web::Query<FizzbuzzQuery>) -> impl Responder {
  println!("{}", info.count);

  if info.count<0{
    return ":ayase_eye2.ex-large:".to_string();
  }

  let mut ret = "".to_string();
  for i in 1..info.count+1{
    if i%15==0{
      ret.push_str("fizzbuzz\n");
    }
    else if i%3==0{
      ret.push_str("fizz\n");
    }
    else if i%5==0{
      ret.push_str("buzz\n");
    }
    else{
      ret.push_str(&format!("{}\n", i));
    }
  }
  ret
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
  format!("Hello, {name}!")
}

#[post("/post")]
async fn post(info: web::Json<JsonData>) -> impl Responder {
  let ret = serde_json::to_string(&info).unwrap();
  println!("id:{}, name:{}, all:\n{}", info.id, info.name, ret);
  ret
}

#[post("/add")]
async fn add(info: web::Json<AddJsonData>) -> impl Responder {
  let ret = AnswerJsonData{
    answer: info.left+info.right
  };
  serde_json::to_string(&ret).unwrap()
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
      .service(ping)
      .service(fizzbuzz)
      .service(hello)
      .service(post)
      .service(add)
  })
  .bind((address, 8080))?
  .run()
  .await
}
