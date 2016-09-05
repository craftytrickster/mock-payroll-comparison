use iron::{Iron, IronResult, Request, Response, status};
use iron::headers::AccessControlAllowOrigin;
use iron::modifiers::Header;
use router::Router;
use serde_json;
use std::io::Read;
use submission::{process_payment, PaymentInfo};

pub fn start() {
    let mut router = Router::new();
    router.get("/", index_handler, "hello_world");
    router.post("/payment", payment_handler, "payment");

    println!("Rust Application Started");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn index_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Rust Payment System Online.")))
}

fn payment_handler(req: &mut Request) -> IronResult<Response> {
    let mut bytes = Vec::new();
    req.body.read_to_end(&mut bytes).unwrap();

    println!("Received payment request");
    let info: PaymentInfo = serde_json::de::from_slice(&bytes).unwrap();

    process_payment(&info);

    Ok(Response::with((status::Ok, Header(AccessControlAllowOrigin::Any))))
}

