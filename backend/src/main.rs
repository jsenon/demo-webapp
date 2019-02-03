extern crate iron;
#[macro_use] extern crate router;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use iron::Handler;
use iron::mime::Mime;
use std::io::Read;

use rustracing::tag::Tag;
use rustracing_jaeger::reporter::JaegerCompactReporter;
use rustracing_jaeger::Tracer;
use rustracing::sampler::AllSampler;


use std::sync::{Arc, Mutex};


#[derive(Serialize, Deserialize, Debug)]
struct User {
    user: String
}

struct MessageHandler {
    message: String
}
impl Handler for MessageHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, self.message.clone())))
    }
}

struct Root {
    tracer: Arc<Mutex<Tracer>>,
}
impl Handler for Root {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut msg = String::new();

        let tracer = self.tracer.lock().expect("Cannot acquire lock");
        let mut span = tracer
            .span("RootBase")
            .tag(Tag::new("App", "Demo-Webapp"))
            .tag(Tag::new("Fn", "root"))
            .start();
        span.log(|log| {
            log.std().message("Entering Get /");
        });
        {
            let mut span1 = tracer
                .span("CheckVersion")
                .child_of(&span)
                .tag(Tag::new("App", "Demo-Webapp"))
                .tag(Tag::new("Fn", "version"))
                .start();
            span1.log(|log| {
                log.std().message("Retrieve version");
            });
            msg.push_str("Version v0.1.0");
        }
        Ok(Response::with((status::Ok,msg)))
    }
}

struct PostUser {
    tracer: Arc<Mutex<Tracer>>,
}
impl Handler for PostUser {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let tracer = self.tracer.lock().expect("Cannot acquire lock");
        let mut span = tracer
            .span("PostUser")
            .tag(Tag::new("App", "Demo-Webapp"))
            .tag(Tag::new("Fn", "postuser"))
            .start();
        span.log(|log| {
            log.std().message("Entering Get /");
        });
        let mime = "application/json".parse::<Mime>().unwrap();
        let mut payload = String::new();
        
        request.body.read_to_string(&mut payload).unwrap();
        println!("{:?}", payload);
        
        let deserialized: User = serde_json::from_str(&payload).unwrap();
        println!("Deserialized: {:?}", deserialized);

        let result = format!("{} {}", "Succesfully create", deserialized.user);
        Ok(Response::with((mime, status::Ok, result)))
    }
}


fn main() {
    let (tracer, span_rx) = Tracer::new(AllSampler);
    let tracer_post = tracer.clone();  
    let nospan = MessageHandler {
        message: "This is a test!".to_string()
    };
    let root = Root {
        tracer: Arc::new(Mutex::new(tracer)),
    };

    let postuser = PostUser {
        tracer: Arc::new(Mutex::new(tracer_post)),
    };

    let router = router!{
        id_1: get "/" => root,
        id_2: post "/user" => postuser,
        id_3: get "/nospan" => nospan,
    };

    std::thread::spawn(move || {
        let reporter = JaegerCompactReporter::new("Demo-Webapp").unwrap();
        for span in span_rx {
            reporter.report(&[span]).unwrap();
        }
    });
    println!("Serving on http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}
