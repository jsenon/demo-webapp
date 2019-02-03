extern crate iron;
#[macro_use] extern crate router;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use std::io::Read;

use rustracing::tag::Tag;
use rustracing_jaeger::reporter::JaegerCompactReporter;
use rustracing_jaeger::Tracer;
use rustracing::sampler::AllSampler;




#[derive(Serialize, Deserialize, Debug)]
struct User {
    user: String
}

fn main() {
    let mut test = String::new();

    let router = router!{
        id_1: get "/" => get_form,
        id_2: post "/user" => post_form
    };
    let (tracer, span_rx) = Tracer::new(AllSampler);

    {
        let mut span = tracer
                .span("Main")
                .tag(Tag::new("version", "0.1.1"))
                .start();
        span.log(|log| {
            log.std().message("Entering in main fn");
        });
        {
            let mut span1 = tracer
                .span("Test")
                .child_of(&span)
                .tag(Tag::new("version", "0.1.1"))
                .start();
            span1.log(|log| {
                log.std().message("Set Test");
            });
             test.push_str("Version v0.1.2");  
        }
    }

    std::thread::spawn(move || {
        let reporter = JaegerCompactReporter::new("Demo-Webapp").unwrap();
        for span in span_rx {
            reporter.report(&[span]).unwrap();
        }
    });

    println!("Serving on http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();


    fn post_form(request: &mut Request) -> IronResult<Response> {
        let mime = "application/json".parse::<Mime>().unwrap();
        let mut payload = String::new();
        
        request.body.read_to_string(&mut payload).unwrap();
        println!("{:?}", payload);
        
        let deserialized: User = serde_json::from_str(&payload).unwrap();
        println!("Deserialized: {:?}", deserialized);

        let result = format!("{} {}", "Succesfully create", deserialized.user);

        Ok(Response::with((mime, status::Ok, result)))
    }

    fn get_form(req: &mut Request) -> IronResult<Response> {
    
        let mut ver = String::new();
        let raw_content_type = req.headers.get_raw("");
        println!("Headers: {:?}", raw_content_type);
        ver.push_str("Version v0.1.0");
        let mime = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((mime, status::Ok, ver)))
    }


}
