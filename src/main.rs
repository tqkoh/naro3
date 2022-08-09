use actix_web::{get, middleware, post, web, App, HttpRequest, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[derive(Serialize, Deserialize)]
struct JsonData {
    id: isize,
    name: String,
    arr: Vec<isize>,
}

#[derive(Deserialize)]
struct AddJsonData {
    left: isize,
    right: isize,
}

#[derive(Serialize)]
struct AnswerJsonData {
    answer: isize,
}

#[derive(Deserialize)]
struct FizzbuzzQuery {
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

#[get("dbtest")]
async fn dbtest() -> impl Responder {
    "test"
}

#[get("fizzbuzz")]
async fn fizzbuzz(info: web::Query<FizzbuzzQuery>) -> impl Responder {
    println!("{}", info.count);

    if info.count < 0 {
        return ":ayase_eye2.ex-large:".to_string();
    }

    let mut ret = "".to_string();
    for i in 1..info.count + 1 {
        if i % 15 == 0 {
            ret.push_str("fizzbuzz\n");
        } else if i % 3 == 0 {
            ret.push_str("fizz\n");
        } else if i % 5 == 0 {
            ret.push_str("buzz\n");
        } else {
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
    let ret = AnswerJsonData {
        answer: info.left + info.right,
    };
    serde_json::to_string(&ret).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let address = match env::var("NARO3_ADDRESS") {
        Ok(val) => val,
        Err(_e) => "localhost".to_string(),
    };

    let database = env::var("DB_DATABASE").expect("DB_DATABASE is not set");
    let user = env::var("DB_USERNAME").expect("DB_USERNAME is not set");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD is not set");
    let port = env::var("DB_PORT").unwrap_or("3306".to_string());
    let host = env::var("DB_HOSTNAME").unwrap_or("localhost".to_string());

    // mysql://user:pass@127.0.0.1:3306/db_name
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        user, password, host, port, database
    );
    println!(
        "{}",
        format!("mysql://{}:(password)@{}:{}/{}", user, host, port, database)
    );

    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let sql = r#"SELECT * FROM city WHERE Name='Tokyo'"#;
    let row: (i64,) = sqlx::query_as(sql)
        .bind(150_i64)
        .fetch_one(&pool)
        .await
        .unwrap_or((-1,));
    println!("{}", row.0);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(ping)
            .service(fizzbuzz)
            .service(hello)
            .service(post)
            .service(add)
            .service(dbtest)
    })
    .bind((address, 8080))?
    .run()
    .await
}
