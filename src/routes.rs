use router::Router;

use common::middlewares::authorize;
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
    router.get("/create-topic", authorize(topic::render_create_topic, true, false), "render_create_topic");
    router.post("/create-topic", authorize(topic::create_topic, true, false), "create_topic");
    router.get("/edit-topic/:topic_id", authorize(topic::render_edit_topic, true, false), "render_edit_topic");
    router.put("/edit-topic/:topic_id", authorize(topic::edit_topic, true, false), "edit_topic");
    router.delete("/delete-topic/:topic_id", authorize(topic::delete_topic, true, false), "delete_topic");
    router.post("/topic/collect/:topic_id", authorize(topic::collect_topic, true, false), "collect_topic");
    router.post("/topic/vote/:topic_id", authorize(topic::vote_topic, true, false), "vote_topic");
    router.post("/topic/stick/:topic_id", authorize(topic::stick_topic, true, true), "stick_topic");
    router.post("/topic/essence/:topic_id", authorize(topic::essence_topic, true, true), "essence_topic");

    router.post("/create-comment", authorize(comment::create_comment, true, false), "create_comment");
    router.get("/edit-comment/:comment_id", authorize(comment::render_edit_comment, true, false), "render_edit_comment");
    router.put("/edit-comment/:comment_id", authorize(comment::edit_comment, true, false), "edit_comment");
    router.delete("/delete-comment/:comment_id", authorize(comment::delete_comment, true, false), "delete_comment");
    router.post("/comment/vote/:comment_id", authorize(comment::vote_comment, true, false), "vote_comment");

    router.get("/:username/message/unread", authorize(message::render_unread_message, true, false), "render_unread_message");
    router.get("/read-message/:message_id", authorize(message::read_message, true, false), "read_message");

    router.get("/user/:username", user::render_user, "render_user");
    router.put("/user/update", authorize(user::update_user_info, true, false), "update_user_info");
    router.put("/user/change-password", authorize(user::change_password, true, false), "change_password");

    router.get("/reset-password", reset_password::render_reset_password, "render_find_password");
    router.post("/reset-password", reset_password::send_reset_password_email, "send_reset_password_email");
    router.get("/set-new-password", reset_password::render_set_new_password, "render_set_new_password");
    router.post("/set-new-password", reset_password::set_new_password, "set_new_password");

    router.get("/resource", simple_render::render_resource, "resource");

    router.get("/about-site", simple_render::render_about_site, "about_site");

    router.post("/upload", authorize(upload::upload_file, true, false), "upload");

    router.get("/rss", rss::render_rss, "render_rss");

    router.get("/forbidden", error::render_forbidden, "render_forbidden");

    router.get("/*", error::render_not_found, "render_not_found");

    router
}

