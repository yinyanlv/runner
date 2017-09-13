use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware};

pub struct FlowControl;

impl BeforeMiddleware for FlowControl {

    fn before(&self, req: &mut Request) -> IronResult<()> {

        Ok(())
    }
}

impl AfterMiddleware for FlowControl {

    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {

        Ok(res)
    }
}
