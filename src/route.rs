use router::Router;
use controllers::*;

pub fn gen_router() -> Router {

    let mut router = Router::new();

    router.get("/", home::render_home, "home");

    router
}

