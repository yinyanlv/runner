use router::Router;
use controllers::*;

pub fn gen_router() -> Router {

    let mut router = Router::new();

    router.get("/", home::render_home, "home_render");

    router.get("/login", login::render_login, "login_render");

    router.get("/register", register::render_register, "register_render");

    router.post("/register", register::register, "register_register");

    router
}

