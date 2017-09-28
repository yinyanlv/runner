use router::Router;
use controllers::*;

pub fn gen_router() -> Router {

    let mut router = Router::new();

    router.get("/", home::render_home, "render_home");

    router.get("/login", login::render_login, "render_login");
    router.post("/login", login::login, "login");

    router.get("/register", register::render_register, "render_register");
    router.post("/register", register::register, "register");

    router.get("/github/auth", login::github_auth_callback, "github_auth_callback");
    router.post("/bind-user", register::bind_user, "bind_user");

    router.get("/logout", logout::logout, "logout");

    router.get("/topic/:topic_id", topic::render_topic, "render_topic");
    router.get("/create-topic", topic::render_create_topic, "render_create_topic");
    router.post("/create-topic", topic::create_topic, "create_topic");
    router.get("/edit-topic", topic::render_edit_topic, "render_edit_topic");
    router.post("/edit-topic/:topic_id", topic::edit_topic, "edit_topic");

    router.get("/user/:username", user::render_user, "render_user");
    router.post("/user/update", user::update_user_info, "update_user_info");
    router.post("/user/change-password", user::change_password, "change_password");

    router.get("/resource", simple_render::render_resource, "resource");

    router.get("/about-site", simple_render::render_about_site, "about_site");

    router.post("/upload", upload::upload_file, "upload");

    router.get("/*", error::render_not_found, "render_not_found");

    router
}

