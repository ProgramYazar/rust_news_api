use std::fs;
use std::io::{Read, Write};
use std::time::SystemTime;

use gnews::SearchArea;
use rocket::response::content;

#[macro_use]
extern crate rocket;
mod gnews;

const MAX_CACHE_TIME_SEC: u64 = 300; // 5 minute

fn url_to_filename(url: &str) -> String {
    let url = url.to_ascii_lowercase();
    let url = url
        .replace("https://", "")
        .replace("http://", "")
        .replace("=", "")
        .replace("::", "_")
        .replace("?", "_")
        .replace("&", "_")
        .replace(":", "_")
        .replace("/", "_")
        .replace(".", "_")
        .replace("from", "")
        + ".cache";

    format!("cache/{url}")
}

fn in_cache(url: &str) -> Option<String> {
    let filename = url_to_filename(url);
    let f = fs::File::options().read(true).write(false).open(&filename);

    if f.is_err() {
        return None;
    }

    let mut f = f.unwrap();
    let ctime_duration = SystemTime::now()
        .duration_since(f.metadata().unwrap().created().unwrap())
        .unwrap()
        .as_secs();
    println!("sec: {ctime_duration}");

    if ctime_duration > MAX_CACHE_TIME_SEC {
        fs::remove_file(filename).expect("cache file remove error");
        return None;
    }

    let mut ostring = String::new();
    f.read_to_string(&mut ostring).expect("read cache error");

    // println!("coming from cache: {filename}");
    return Some(ostring);
}

fn write_cache(url: &str, content: &str) -> std::io::Result<()> {
    let filename = url_to_filename(url);
    fs::File::options()
        .create(true)
        .write(true)
        .append(false)
        .open(filename)?
        .write(content.as_bytes())?;
    Ok(())
}

#[get("/searchByTitle/<title>")]
async fn search_by_title(title: &str) -> content::RawJson<String> {
    let search_req = gnews::create_search_req(title, &[SearchArea::Title], Some(20), None);

    match in_cache(&search_req) {
        Some(fcontent) => return content::RawJson(fcontent),
        None => {
            let api_repsonse = reqwest::get(&search_req).await.unwrap();
            if let Ok(sresult) = api_repsonse.text().await {
                write_cache(&search_req, &sresult).expect("cache write error");
                return content::RawJson(sresult);
            }
        }
    }
    content::RawJson("{ \"error\": \"Unknown error\" }".to_string())
}

#[get("/searchByDescription/<description>")]
async fn search_by_description(description: &str) -> content::RawJson<String> {
    let search_req =
        gnews::create_search_req(description, &[SearchArea::Description], Some(20), None);

    match in_cache(&search_req) {
        Some(fcontent) => return content::RawJson(fcontent),
        None => {
            let api_repsonse = reqwest::get(&search_req).await.unwrap();
            if let Ok(sresult) = api_repsonse.text().await {
                write_cache(&search_req, &sresult).expect("cache write error");
                return content::RawJson(sresult);
            }
        }
    }
    content::RawJson("{ \"error\": \"Unknown error\" }".to_string())
}

#[get("/searchByContent/<content>")]
async fn search_by_content(content: &str) -> content::RawJson<String> {
    let search_req = gnews::create_search_req(content, &[SearchArea::Content], Some(20), None);

    match in_cache(&search_req) {
        Some(fcontent) => return content::RawJson(fcontent),
        None => {
            let api_repsonse = reqwest::get(&search_req).await.unwrap();
            if let Ok(sresult) = api_repsonse.text().await {
                write_cache(&search_req, &sresult).expect("cache write error");
                return content::RawJson(sresult);
            }
        }
    }
    content::RawJson("{ \"error\": \"Unknown error\" }".to_string())
}

#[get("/")]
async fn index() -> String {
    let search_req = gnews::create_search_req("history", &[SearchArea::Title], Some(20), None);
    let sresp = reqwest::get(search_req)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    sresp
}

#[launch]
fn rocket() -> _ {
    std::env::set_var(
        "ROCKET_SECRET_KEY",
        "asdasd12321asdasd12321asdasd12321asdasd12321",
    );

    for (k, v) in std::env::vars() {
        eprintln!("{}={}", k, v);
    }
    rocket::build().mount(
        "/",
        routes![
            index,
            search_by_title,
            search_by_content,
            search_by_description
        ],
    )
}
