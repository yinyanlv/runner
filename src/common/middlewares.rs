use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, AroundMiddleware, Handler};

pub struct FlowControl;

impl BeforeMiddleware for FlowControl {

    fn before(&self, req: &mut Request) -> IronResult<()> {

        println!("{:?}", req.url);

        Ok(())
    }
}

impl AfterMiddleware for FlowControl {

    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {

        Ok(res)
    }
}

impl AroundMiddleware for FlowControl {

    fn around(self, handler: Box<Handler>) -> Box<Handler> {

        Box::new(move |req: &mut Request| -> IronResult<Response> {

            println!("flow");

            handler.handle(req)
        })
    }
}

pub struct AuthorizeControl;

impl AroundMiddleware for AuthorizeControl {

    fn around(self, handler: Box<Handler>) -> Box<Handler> {

        Box::new(move |req: &mut Request| -> IronResult<Response> {

            println!("authorize");

            handler.handle(req)
        })
    }
}
