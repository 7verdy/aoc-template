use anyhow::Result;
use reqwest::{self, Response};
// use reqwest::{cookie::Jar, Url};
use std::io::prelude::*;

use std::fs::{read_to_string, File};
use std::io::copy;

fn step_one(filename: String) {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;
    // =========== END SETUP =============

    // =========== END STEP 1 ============
    println!("The result for step 1 is: {}", total);
}

fn step_two(filename: String) {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;
    // =========== END SETUP =============

    // =========== END STEP 2 ============
    println!("The result for step 2 is: {}", total);
}


fn dl_file_from_url(url_str: String) -> Result<()> {
    if File::open("input.txt").is_ok() {
        return Ok(());
    }
    let jar = reqwest::cookie::Jar::default();
    let url = reqwest::Url::parse(&url_str)?;

    let mut cookie_file = File::open("cookie.txt")?;
    let mut cookie = String::new();
    cookie_file.read_to_string(&mut cookie)?;
    jar.add_cookie_str(format!("session={}", cookie).as_str(), &url);

    let client = reqwest::blocking::Client::builder()
        .cookie_provider(jar.into())
        .build()?;
    let mut response = client.get(url).send()?;
    println!("{:?}", response);
    let mut file = File::create("input.txt")?;
    copy(&mut response, &mut file)?;
    Ok(())
}

fn main() {
    let filename = "input.txt".to_string();
    let day = 6; // TO MODIFY
    let year = 2023; // TO MODIFY
    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let _ = dl_file_from_url(input_url);

    step_one(filename.clone());
    step_two(filename.clone());
}
