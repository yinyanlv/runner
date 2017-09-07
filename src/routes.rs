use router::Router;
use controllers::*;

pub fn gen_router() -> Router {

    let mut router = Router::new();

    router.get("/", home::render_home, "render_home");

    router.get("/login", login::render_login, "render_login");

    router.post("/login", login::login, "do_login");

    router.get("/register", register::render_register, "render_register");

    router.post("/register", register::register, "do_register");

    router
}

