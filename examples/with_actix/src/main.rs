use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use partiql::lang::Lang;
use partiql::lang::LangType;
use partiql::parser;

struct AppState {
    lang: Lang,
}

#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello World"))
}

#[get("/greet/{name}")]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", &name)
}

#[get("/pokemon/en/partiql/{query}")]
async fn pokemon(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let q = if let Some(q) = req.match_info().get("query") {
        q
    } else {
        return Ok(HttpResponse::Ok().content_type("text/plain").body("error"));
    };
    let sql = if let Some(sql) = parser::sql(q).ok() {
        sql
    } else {
        return Ok(HttpResponse::Ok().content_type("text/plain").body("error"));
    };

    let result = partiql::sql::evaluate(&sql, &data.lang.data);

    let mut lang = Lang {
        from: LangType::Json,
        to: LangType::Toml,
        data: result,
        colnames: vec![],
        text: String::default(),
    };

    Ok(HttpResponse::Ok().json(lang.data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let json = include_str!("../pokemon.json/en/pokemon.json");

    println!("hello");

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                lang: Lang::from_as(&json, partiql::lang::LangType::Json).expect("parse json"),
            })
            .service(index)
            .service(greet)
            .service(pokemon)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
