use actix_files as fs;
use actix_web::dev::Server;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::env;
use std::path::{Path, PathBuf};

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
    HttpResponse::Ok().body(create_root())
}

#[get("/about")]
async fn about() -> impl Responder {
    HttpResponse::Ok().body(create_about())
}

#[get("/members")]
async fn members() -> impl Responder {
    HttpResponse::Ok().body(create_members())
}

#[get("/alumni")]
async fn alumni() -> impl Responder {
    HttpResponse::Ok().body(create_alumni())
}

#[get("/contact")]
async fn contact() -> impl Responder {
    HttpResponse::Ok().body(create_contact())
}

#[get("/classes")]
async fn classes() -> impl Responder {
    HttpResponse::Ok().body(create_classes())
}

#[get("/join")]
async fn join() -> impl Responder {
    HttpResponse::Ok().body(create_join())
}

fn create_root() -> String {
    IndexTemplate {}.render().unwrap()
}

fn create_about() -> String {
    AboutTemplate {}.render().unwrap()
}

fn create_members() -> String {
    MembersTemplate {}.render().unwrap()
}

fn create_alumni() -> String {
    AlumniTemplate {}.render().unwrap()
}

fn create_contact() -> String {
    ContactTemplate {}.render().unwrap()
}

fn create_classes() -> String {
    ClassesTemplate {
        classes: vec![Class::new(
            "Example class".to_owned(),
            "This is the description of the example class".to_owned(),
        )],
    }
    .render()
    .unwrap()
}

fn create_join() -> String {
    JoinTemplate {}.render().unwrap()
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let mode = match args.next() {
        Some(mode_str) => match mode_str.as_str() {
            "static" => Mode::Static,
            "serve" => Mode::Serve,
            _ => panic!("Unknown mode {}", mode_str),
        },
        None => Mode::Serve,
    };
    if args.next().is_some() {
        panic!("Too many arguments");
    }

    match mode {
        Mode::Static => {
            let pages = [
                ("", create_root as fn() -> String),
                ("about", create_about as fn() -> String),
                ("members", create_members as fn() -> String),
                ("alumni", create_alumni as fn() -> String),
                ("contact", create_contact as fn() -> String),
                ("classes", create_classes as fn() -> String),
                ("join", create_join as fn() -> String),
            ];

            // Create pages
            for (page, create_page) in pages.iter() {
                let directory = format!("static_output/{}", page);
                std::fs::create_dir_all(&directory)?;
                let mut file = File::create(format!("{}/index.html", &directory))?;
                file.write_all(&create_page().bytes().collect::<Vec<u8>>())?;
            }

            // Copy static
            copy("web/static", "static_output/static")?;
        }
        Mode::Serve => {
            serve().await?;
        }
    }

    Ok(())
}

fn serve() -> Server {
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
    .bind("127.0.0.1:8088")
    .expect("Could not bind to port 8088")
    .run()
}

enum Mode {
    Static,
    Serve,
}

// From https://stackoverflow.com/a/60406693/7432915
pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if std::fs::metadata(&dest).is_err() {
            std::fs::create_dir_all(&dest)?;
        }

        for entry in std::fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        std::fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        eprintln!("failed to copy: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}
