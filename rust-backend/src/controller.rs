use iron::{Iron, IronResult, Request, Response, status};
use iron::method::Method;
use iron::middleware::Chain;
use iron_cors::CORS;
use router::Router;
use serde_json;
use std::io::Read;
use submission::{process_payment, PaymentInfo};

pub fn start() {
    let mut router = Router::new();
    router.get("/", index_handler, "hello_world");
    router.post("/payment", payment_handler, "payment");

    let cors = CORS::new(vec![(vec![Method::Post], "payment".to_string())]);

    let mut chain = Chain::new(router);
    chain.link_after(cors);

    println!("Rust Application Started");
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn index_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(("Rust Payment System Online.")))
}

fn payment_handler(req: &mut Request) -> IronResult<Response> {
    let mut bytes = Vec::new();
    req.body.read_to_end(&mut bytes).unwrap();

    println!("Received payment request");
    let info: PaymentInfo = serde_json::de::from_slice(&bytes).unwrap();

    let message = process_payment(&info);

    Ok(Response::with((status::Ok, message)))
}

