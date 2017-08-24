use router::Router;
use controllers::*;

pub fn get_router() -> Router {

    let mut router = Router::new();

    router.get("/", home::render_home, "index");

    router
}

