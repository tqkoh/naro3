use actix_session::Session;
use actix_web::web::Data;
use actix_web::{
    get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::*;

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    println!("Request: {req:?}");
    HttpResponse::Ok().body("Hello, World!")
}

#[get("ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[derive(Default, Serialize)]
struct City {
    ID: i32,
    Name: String,
    CountryCode: String,
    District: String,
    Population: i32,
}

#[get("dbtest")]
async fn dbtest(pool_data: web::Data<Arc<Mutex<sqlx::Pool<sqlx::MySql>>>>) -> impl Responder {
    println!("{:?}", pool_data);
    let pool = pool_data.lock().unwrap();
    let mut ret: Vec<String> = vec![];

    let row_tokyo = sqlx::query_as!(City, r#"SELECT * FROM city WHERE Name='Tokyo'"#)
        .fetch_one(&*pool)
        .await
        .unwrap_or(Default::default());
    ret.push(format!("東京都のid: {}\n", row_tokyo.ID));
    println!("{}", row_tokyo.Name);

    let rows_city_in_japan = sqlx::query_as!(City, r#"SELECT * FROM city WHERE CountryCode='JPN'"#)
        .fetch_all(&*pool)
        .await
        .unwrap_or(Default::default());
    ret.push("日本の都市一覧:".to_string());
    for row in rows_city_in_japan.iter() {
        ret.push(format!("名前: {}, 人口: {}", row.Name, row.Population));
    }

    HttpResponse::Ok().body(ret.join("\n"))
}

#[get("cities/{name}")]
async fn cities(
    name: web::Path<String>,
    pool_data: web::Data<Arc<Mutex<sqlx::Pool<sqlx::MySql>>>>,
) -> impl Responder {
    let pool = pool_data.lock().unwrap();
    let ret = sqlx::query_as!(City, r#"SELECT * FROM city WHERE Name=?"#, name.to_string())
        .fetch_one(&*pool)
        .await
        .unwrap_or(Default::default());
    if ret.ID == 0 {
        HttpResponse::NotFound().body(format!("city {name} not found"))
    } else {
        HttpResponse::Ok().json(ret)
    }
}

#[derive(Default, Deserialize)]
struct PostCity {
    name: String,
    countryCode: String,
    district: String,
    population: i32,
}

#[post("/postcity")]
async fn postcity(
    city: web::Json<PostCity>,
    pool_data: web::Data<Arc<Mutex<sqlx::Pool<sqlx::MySql>>>>,
) -> impl Responder {
    let pool = pool_data.lock().unwrap();
    sqlx::query!(
        "INSERT INTO city (Name, CountryCode, District, Population) VALUES (?, ?, ?, ?);",
        city.name,
        city.countryCode,
        city.district,
        city.population,
    )
    .execute(&*pool)
    .await
    .unwrap();
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct FizzbuzzQuery {
    count: isize,
}

#[get("fizzbuzz")]
async fn fizzbuzz(info: web::Query<FizzbuzzQuery>) -> impl Responder {
    println!("{}", info.count);

    if info.count < 0 {
        return HttpResponse::Ok().body(":ayase_eye2.ex-large:");
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
    HttpResponse::Ok().body(ret)
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {name}!"))
}

#[derive(Serialize, Deserialize)]
struct JsonData {
    id: isize,
    name: String,
    arr: Vec<isize>,
}

#[post("/post")]
async fn post(info: web::Json<JsonData>) -> impl Responder {
    HttpResponse::Ok().json(info)
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

#[post("/add")]
async fn add(info: web::Json<AddJsonData>) -> impl Responder {
    let ret = AnswerJsonData {
        answer: info.left + info.right,
    };
    HttpResponse::Ok().json(ret)
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Default)]
struct User {
    Username: String,
    HashedPass: String,
}

#[post("signup")]
async fn signup(
    req: web::Json<LoginRequest>,
    pool_data: web::Data<Arc<Mutex<sqlx::Pool<sqlx::MySql>>>>,
) -> impl Responder {
    let pool = pool_data.lock().unwrap();
    if req.username == "" || req.password == "" {
        return HttpResponse::BadRequest().body("username or password cannot be empty");
    }
    let hashed_pass = hash(req.password, DEFAULT_COST).unwrap();
    let count = sqlx::query!(
        r#"SELECT COUNT(*) as value FROM users WHERE Username=?"#,
        req.username
    )
    .fetch_one(&*pool)
    .await
    .unwrap()
    .value;

    if count > 0 {
        return HttpResponse::Conflict().body("user already exists");
    }

    sqlx::query!(
        "INSERT INTO users (Username, HashedPass) VALUES (?, ?);",
        req.username,
        hashed_pass
    )
    .execute(&*pool)
    .await
    .unwrap();

    HttpResponse::Created().finish()
}

#[post("login")]
async fn login(
    req: web::Json<LoginRequest>,
    pool_data: web::Data<Arc<Mutex<sqlx::Pool<sqlx::MySql>>>>,
    session: Session,
) -> impl Responder {
    let pool = pool_data.lock().unwrap();
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE Username = ?", req.username)
        .fetch_one(&*pool)
        .await
        .unwrap();
    let hashed_pass = user.HashedPass;
    let valid = verify(&req.password, &hashed_pass).unwrap();
    if !valid {
        return HttpResponse::Forbidden().body("password does not match");
    }

    session.insert("username", &req.username);
    HttpResponse::Ok().body("login successful")
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
            .service(postcity)
            .service(cities)
            .service(signup)
            .service(login)
    })
    .bind((address, 8080))?
    .run()
    .await
}
