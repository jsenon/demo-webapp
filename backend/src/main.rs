extern crate iron;
extern crate router;

extern crate backend;


use iron::prelude::*;
use router::Router;
use backend::*;



fn main() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/user", post_form, "user");
    println!("Serving on http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}
