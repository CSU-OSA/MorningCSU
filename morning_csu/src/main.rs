use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use http::Error;
use serde::{Serialize, Deserialize};
use simplelog::*;
use log::{info, error, warn, debug};

mod sign_in;

#[actix_web::main]
async fn main() -> Result<(),()> {
    let log_file = match std::fs::File::create("morning_csu.log"){
        Ok(file) => file,
        Err(_) => return Err(()),
    };
    simplelog::WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        log_file,
    ).expect("simplelog init error!");
    println!("Hello, world!");
    Ok(())
}

