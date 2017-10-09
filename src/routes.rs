use router::Router;
use controllers::*;

pub fn gen_router() -> Router {

    let mut router = Router::new();

    router.get("/", topic_list::render_default_topic_list, "render_default_topic_list");
    router.get("/topics/essence", topic_list::render_essence_topic_list, "render_essence_topic_list");
    router.get("/topics/latest", topic_list::render_latest_topic_list, "render_latest_topic_list");
    router.get("/topics/no-reply", topic_list::render_no_reply_topic_list, "render_no_reply_topic_list");
    router.get("/topics/ask", topic_list::render_ask_topic_list, "render_ask_topic_list");
    router.get("/topics/share", topic_list::render_share_topic_list, "render_share_topic_list");
    router.get("/topics/job", topic_list::render_job_topic_list, "render_job_topic_list");
    router.get("/:username/topics", topic_list::render_user_topics, "render_user_topics");
    router.get("/:username/comments", topic_list::render_user_comments, "render_user_comments");
    router.get("/:username/collections", topic_list::render_user_collections, "render_user_collections");

    router.get("/search", topic_list::render_search_result, "render_search_result");

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
    router.get("/edit-topic/:topic_id", topic::render_edit_topic, "render_edit_topic");
    router.put("/edit-topic/:topic_id", topic::edit_topic, "edit_topic");
    router.delete("/delete-topic/:topic_id", topic::delete_topic, "delete_topic");
    router.post("/topic/collect/:topic_id", topic::collect_topic, "collect_topic");
    router.post("/topic/stick/:topic_id", topic::stick_topic, "stick_topic");
    router.post("/topic/essence/:topic_id", topic::essence_topic, "essence_topic");
    router.post("/topic/vote/:topic_id", topic::vote_topic, "vote_topic");

    router.post("/create-comment", comment::create_comment, "create_comment");
    router.get("/edit-comment/:comment_id", comment::render_edit_comment, "render_edit_comment");
    router.put("/edit-comment/:comment_id", comment::edit_comment, "edit_comment");
    router.delete("/delete-comment/:comment_id", comment::delete_comment, "delete_comment");
    router.post("/comment/vote/:comment_id", comment::vote_comment, "vote_comment");

    router.get("/:username/message/unread", message::render_unread_message, "render_unread_message");
    router.get("/read-message/:message_id", message::read_message, "read_message");

    router.get("/user/:username", user::render_user, "render_user");
    router.put("/user/update", user::update_user_info, "update_user_info");
    router.put("/user/change-password", user::change_password, "change_password");

    router.get("/resource", simple_render::render_resource, "resource");

    router.get("/about-site", simple_render::render_about_site, "about_site");

    router.post("/upload", upload::upload_file, "upload");

    router.get("/rss", rss::render_rss, "render_rss");

    router.get("/*", error::render_not_found, "render_not_found");

    router
}

