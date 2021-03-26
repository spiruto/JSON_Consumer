extern crate reqwest;
use std::env::args;
use std::fs::write;
fn main() {
    let mut arguments = args().skip(1);
    if args().len()<=1 {
        panic!("URL not provided");
    }
    let key: String = arguments.next().unwrap();

    let value: String = arguments.next().unwrap();
    if key != "--url" && value.is_empty() {
        panic!("URL not provided");
    }
    match reqwest::blocking::get(value) {
        Ok(response) => {
            let parsed = json::parse(&response.text().unwrap());
            match write("data.json", parsed.unwrap().to_string()) {
                Ok(()) => {print!("Document created, written and saved.");}
                Err(_) => {print!("Not able to fetch/create the data");}
            };
        }
        Err(error) => {
            print!("Error, {}", error)
        }
    }
}
