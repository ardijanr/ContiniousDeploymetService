use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use std::{env, fs};

use std::process::Command;

use execute::Execute;



#[get("/deploy/{project}")]
async fn index(req: HttpRequest, project: web::Path<String>) -> String {
    get_folders(project.as_str().to_string()).await;
    format!("Hello: {}!\r\n", project)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(
                web::resource("/")
                    .wrap(middleware::DefaultHeaders::new().header("X-Version-R2", "0.3"))
                    .default_service(web::route().to(HttpResponse::MethodNotAllowed))
            )
            .service(web::resource("/test1.html").to(|| async { "Test\r\n" }))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}

async fn get_folders(folder: String){
    let folders = fs::read_dir("/home/ardijan/repos").unwrap().filter(|x| x.as_ref().unwrap().path().ends_with(&folder));

    for folder in folders{
        // println!("FOLDER WAS FOUND  {:?}",&folder.unwrap().path());

        println!("{:?}", output);
    }
}



