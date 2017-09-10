use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware};

pub struct GlobalControl;

impl BeforeMiddleware for GlobalControl {

    fn before(&self, req: &mut Request) -> IronResult<()> {

        Ok(())
    }
}

impl AfterMiddleware for GlobalControl {

    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {

        Ok(res)
    }
}
