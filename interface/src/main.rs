use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::Hasher;
use std::path::{Path, PathBuf};

use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};

use handlebars::Handlebars;
use sass_rs::{compile_file, Options};

use serde::{Deserialize, Serialize};


#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

// thanks rust-lang git repo for the sass compile stuff

#[derive(Clone, Serialize)]
struct CSSFiles {
    app: String,
    fonts: String,
    //vendor: String,
}
#[derive(Clone, Serialize)]
struct JSFiles {
    app: String,
}
#[derive(Clone, Serialize)]
struct AssetFiles {
    css: CSSFiles,
    //js: JSFiles,
}

lazy_static! {
    static ref ASSETS: AssetFiles = {
        let app_css_file = compile_sass("app");
        let fonts_css_file = compile_sass("fonts");
        //let vendor_css_file = concat_vendor_css(vec!["tachyons"]);
        //let app_js_file = concat_app_js(vec!["tools-install"]);

        AssetFiles {
            css: CSSFiles {
                app: app_css_file,
                fonts: fonts_css_file,
            //    vendor: vendor_css_file,
            },
            //js: JSFiles { app: app_js_file },
        }
    };
}

#[get("/static/{filename}")]
async fn get_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({
        "name": "Handlebars"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn hash_css(css: &str) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(css.as_bytes());
    hasher.finish().to_string()
}

fn compile_sass(filename: &str) -> String {
    let scss_file = format!("./src/styles/{}.scss", filename);

    let css = compile_file(&scss_file, Options::default())
        .unwrap_or_else(|_| panic!("couldn't compile sass: {}", &scss_file));

    let css_sha = format!("{}_{}", filename, hash_css(&css));
    let css_file = format!("./static/styles/{}.css", css_sha);

    fs::write(&css_file, css.into_bytes())
        .unwrap_or_else(|_| panic!("couldn't write css file: {}", &css_file));

    String::from(&css_file[1..])
}

fn concat_vendor_css(files: Vec<&str>) -> String {
    let mut concatted = String::new();
    for filestem in files {
        let vendor_path = format!("./static/styles/{}.css", filestem);
        let contents = fs::read_to_string(vendor_path).expect("couldn't read vendor css");
        concatted.push_str(&contents);
    }

    let css_sha = format!("vendor_{}", hash_css(&concatted));
    let css_path = format!("./static/styles/{}.css", &css_sha);

    fs::write(&css_path, &concatted).expect("couldn't write vendor css");

    String::from(&css_path[1..])
}
