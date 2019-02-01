
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    user: String
}

pub fn get_form(_: &mut Request) -> IronResult<Response> {
    let mime = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((mime, status::Ok, "My Version is 0.1")))
}

pub fn post_form(request: &mut Request) -> IronResult<Response> {
    let mime = "application/json".parse::<Mime>().unwrap();
    let mut payload = String::new();
    
    request.body.read_to_string(&mut payload).unwrap();
    println!("{:?}", payload);
    
    let deserialized: User = serde_json::from_str(&payload).unwrap();
    println!("Deserialized: {:?}", deserialized);

    let result = format!("{} {}", "Succesfully create", deserialized.user);

    Ok(Response::with((mime, status::Ok, result)))
}