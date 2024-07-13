use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;
use env_logger::Env;
use std::path::{Path, PathBuf};

const FRONTEND: &str = "./frontend";

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    let path = Path::new(FRONTEND).join("index.html");
    Ok(NamedFile::open_async(path).await?)
}

#[get("/about")]
async fn about() -> actix_web::Result<NamedFile> {
    let path = Path::new(FRONTEND).join("about.html");
    Ok(NamedFile::open_async(path).await?)
}

#[get("/experience")]
async fn experience() -> actix_web::Result<NamedFile> {
    let path = Path::new(FRONTEND).join("experience.html");
    Ok(NamedFile::open_async(path).await?)
}

#[get("/projects")]
async fn projects() -> actix_web::Result<NamedFile> {
    let path = Path::new(FRONTEND).join("projects.html");
    Ok(NamedFile::open_async(path).await?)
}

#[get("/contact")]
async fn contact() -> actix_web::Result<NamedFile> {
    let path = Path::new(FRONTEND).join("contact.html");
    Ok(NamedFile::open_async(path).await?)
}

#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<NamedFile> {
    let path = Path::new(FRONTEND).join("static").join("favicon.ico");
    Ok(NamedFile::open_async(path).await?)
}

#[get("/robots.txt")]
async fn robots() -> actix_web::Result<NamedFile> {
    let path = Path::new(FRONTEND).join("static").join("robots.txt");
    Ok(NamedFile::open_async(path).await?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| 
        App::new()
            .service(index)
            .service(about)
            .service(experience)
            .service(projects)
            .service(contact)
            .service(favicon)
            .service(robots)
            .service(actix_files::Files::new("/static", Path::new(FRONTEND).join("static")))
            .service(actix_files::Files::new("/js", Path::new(FRONTEND).join("out")))
            .service(actix_files::Files::new("/css", Path::new(FRONTEND).join("css")))
            .wrap(Logger::default())
    )
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
