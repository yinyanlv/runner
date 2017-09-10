use router::Router;
use controllers::*;

pub fn gen_router() -> Router {

    let mut router = Router::new();

    router.get("/", home::render_home, "render_home");

    router.get("/login", login::render_login, "render_login");

    router.post("/login", login::login, "login");

    router.get("/github/auth", login::github_auth_callback, "github_auth_callback");

    router.get("/register", register::render_register, "render_register");

    router.post("/register", register::register, "register");

    router.get("/logout", logout::logout, "logout");

    router.get("/*", error::render_not_found, "render_not_found");

    router
}

