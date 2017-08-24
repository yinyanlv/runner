extern crate iron;
extern crate router;

mod route;
mod controllers;

use iron::Chain;

fn main() {

    let port: &str = "3000";
    let mut chain = Chain::new(route::get_router());

    println!("rust is listening on port {} !", port);
    
    iron::Iron::new(chain)
        .http("localhost:".to_string() + port)
        .unwrap();
}