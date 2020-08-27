use actix_files as fs;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use std::io;

struct Class {
    name: String,
    description: String,
}

impl Class {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {}

#[derive(Template)]
#[template(path = "members.html")]
struct MembersTemplate {}

#[derive(Template)]
#[template(path = "join.html")]
struct JoinTemplate {}

#[derive(Template)]
#[template(path = "alumni.html")]
struct AlumniTemplate {}

#[derive(Template)]
#[template(path = "contact.html")]
struct ContactTemplate {}

#[derive(Template)]
#[template(path = "classes.html")]
struct ClassesTemplate {
    classes: Vec<Class>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(IndexTemplate {}.render().unwrap())
}

#[get("/about")]
async fn about() -> impl Responder {
    HttpResponse::Ok().body(AboutTemplate {}.render().unwrap())
}

#[get("/members")]
async fn members() -> impl Responder {
    HttpResponse::Ok().body(MembersTemplate {}.render().unwrap())
}

#[get("/alumni")]
async fn alumni() -> impl Responder {
    HttpResponse::Ok().body(AlumniTemplate {}.render().unwrap())
}

#[get("/contact")]
async fn contact() -> impl Responder {
    HttpResponse::Ok().body(ContactTemplate {}.render().unwrap())
}

#[get("/classes")]
async fn classes() -> impl Responder {
    HttpResponse::Ok().body(
        ClassesTemplate {
            classes: vec![Class::new(
                "Example class".to_owned(),
                "This is the description of the example class".to_owned(),
            )],
        }
        .render()
        .unwrap(),
    )
}

#[get("/join")]
async fn join() -> impl Responder {
    HttpResponse::Ok().body(JoinTemplate {}.render().unwrap())
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./web/static"))
            .service(index)
            .service(about)
            .service(members)
            .service(alumni)
            .service(join)
            .service(contact)
            .service(classes)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
