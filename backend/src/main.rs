extern crate iron;
extern crate router;
extern crate backend;

#[macro_use]
extern crate trackable;


use iron::prelude::*;
use router::Router;
use backend::*;

use rustracing::tag::Tag;
use rustracing_jaeger::reporter::JaegerCompactReporter;
use rustracing_jaeger::Tracer;


fn main() -> trackable::result::MainResult {
    let (tracer, span_rx) = Tracer::new(rustracing::sampler::AllSampler);
    {
        let mut span0 = tracer.span("main")
                .tag(Tag::new("Version", "v0.1.0"))
                .start();
            span0.log(|log| {
                log.std().message("Starting App");
            });
    }   
    let mut reporter = track!(JaegerCompactReporter::new("Demo-app-backend"))?;
    reporter.add_service_tag(Tag::new("App", "Demo-app-backend"));
    track!(reporter.report(&span_rx.try_iter().collect::<Vec<_>>()))?;

                
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/user", post_form, "user");
    println!("Serving on http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();

    Ok(())
}
