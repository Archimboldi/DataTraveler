use actix_web::{web, App, HttpResponse, HttpServer, Error};
use actix_files as fs;
use actix_multipart::Multipart;
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};
use local_ipaddress;
use std::fs::read_dir;

async fn list() -> HttpResponse {
    let fmp = ["film","music","picture"];
    let mut list: Vec<Vec<String>> = Vec::new();
    for item in fmp.iter() {
        let films = read_dir(format!("./9090/{}",item)).unwrap();
        let mut film_n: Vec<String> = Vec::new();
        for film in films {
            if let Ok(film) = film {
                let title = film.file_name();
                let suf: Vec<&str> = title.to_str().unwrap().rsplit(".").collect();
                if suf.len() > 1 {
                    match *item {
                        "film" => {
                            if suf[0] == "mp4" || suf[0] == "ogg" || suf[0] == "webm" {
                                film_n.push(title.to_str().unwrap().to_string());
                            }
                        },
                        "music" => {
                            if suf[0] == "mp3" || suf[0] == "ogg" || suf[0] == "wav"{
                                film_n.push(title.to_str().unwrap().to_string());
                            }
                        },
                        "picture" => {
                            film_n.push(title.to_str().unwrap().to_string());
                        },
                        _ => {}
                    }
                }
            }
        }
        list.push(film_n);
    }
    
    HttpResponse::Ok().json(list)
}

async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let suf: Vec<&str> = filename.rsplit(".").collect();
        if suf.len() > 0 {
            let mut filepath = format!("./9090/picture/{}", filename);
            if suf[0] == "mp3" || suf[0] == "ogg" || suf[0] == "wav" {
                filepath = format!("./9090/music/{}", filename);
            }
            let mut f = async_std::fs::File::create(filepath).await?;
            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f.write_all(&data).await?;
            }
        }
    }
    Ok(HttpResponse::Ok().into())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let ip = local_ipaddress::get().expect("none");
    println!("DataTraveler http://{}:9090", ip);
    HttpServer::new(|| {
        App::new()
            .route("/list", web::get().to(list))
            .route("/upload", web::post().to(upload))
            .service(fs::Files::new("/", "./9090").index_file("index.html"))
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}