use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;
use env_logger::Env;
use std::path::{Path, PathBuf};

const FRONTEND: &str = "./frontend";

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    let path = Path::new(FRONTEND).join("index.html");
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| 
        App::new()
            .service(index)
            .service(actix_files::Files::new("/static", Path::new(FRONTEND).join("static")))
            .service(actix_files::Files::new("/js", Path::new(FRONTEND).join("out")))
            .service(actix_files::Files::new("/css", Path::new(FRONTEND).join("css")))
            .wrap(Logger::default())
    )
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
