extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {

    fn hello_iron(req: &mut Request) -> IronResult<Response> {

        Ok(Response::with((status::Ok, "hello rust iron")))

    }

    Iron::new(hello_iron).http("localhost:3000").unwrap();

    println!("rust server is listening on port 3000 !");
}