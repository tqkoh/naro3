use actix_web::web::Data;
use actix_web::{get, middleware, post, web, App, HttpRequest, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::sync::*;

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
async fn dbtest(pool_data: web::Data<Arc<Mutex<sqlx::Pool<sqlx::MySql>>>>) -> impl Responder {
    println!("{:?}", pool_data);
    let pool = pool_data.lock().unwrap();
    let sql = r#"SELECT * FROM city WHERE Name='Tokyo'"#;
    let row: (i64,) = sqlx::query_as(sql)
        .bind(150_i64)
        .fetch_one(&*pool)
        .await
        .unwrap_or((-1,));
    let ret = format!("{}", row.0);
    println!("{}", ret);
    ret
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
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let address = match env::var("NARO3_ADDRESS") {
        Ok(val) => val,
        Err(_e) => "localhost".to_string(),
    };

    let database = env::var("MARIADB_DATABASE").expect("MARIADB_DATABASE is not set");
    let user = env::var("MARIADB_USERNAME").expect("MARIADB_USERNAME is not set");
    let password = env::var("MARIADB_PASSWORD").expect("MARIADB_PASSWORD is not set");
    let port = env::var("DB_PORT").unwrap_or("3306".to_string());
    let host = env::var("MARIADB_HOSTNAME").unwrap_or("localhost".to_string());

    // mysql://user:pass@127.0.0.1:3306/db_name
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        user, password, host, port, database
    );

    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let pool_data = Arc::new(Mutex::new(pool));
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool_data.clone()))
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
