use iron::prelude::*;
use iron_sessionstorage::traits::SessionRequestExt;
use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::EmailTransport;
use uuid::Uuid;

use common::http::*;
use common::utils::*;
use common::lazy_static::{CONFIG_TABLE, PATH};
use services::user::{get_username_by_email, update_retrieve, get_retrieve_time, update_password};

pub fn render_reset_password(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);

    respond_view("reset-password", &data)
}

pub fn send_reset_password_email(req: &mut Request) -> IronResult<Response> {

    let params = get_request_body(req);
    let email_str = &*params.get("email").unwrap()[0];
    let smtp_config = CONFIG_TABLE.get("smtp").unwrap().as_table().unwrap();
    let smtp_host = smtp_config.get("host").unwrap().as_str().unwrap();
    let smtp_port = smtp_config.get("port").unwrap().as_integer().unwrap() as u16;
    let smtp_username = smtp_config.get("username").unwrap().as_str().unwrap();
    let smtp_password = smtp_config.get("password").unwrap().as_str().unwrap();

    let mut data = JsonData::new();

    let username_wrapper = get_username_by_email(email_str);

    if username_wrapper.is_none() {
        data.success = false;
        data.message = "该邮箱并未在本站注册账号，请检查邮箱地址！".to_string();

        return respond_json(&data);
    }

    let username = username_wrapper.unwrap();
    let retrieve_token = Uuid::new_v4().to_string();

    let email = EmailBuilder::new()
        .to(email_str)
        .from(smtp_username)
        .subject("重置密码")
        .html(&*format!(r#"
            <p style="padding:10px;">
                请点击
                <a style="text-decoration:underline;color:#1e90ff;" href="{0}/set-new-password?username={1}&token={2}" target="_blank">{0}/set-new-password?username={1}&token={2}</a>
                ，进行密码重置！该链接的有效时间为 <span style="color:red;">24</span> 小时！
            </p>
            "#, PATH.to_owned(), username, retrieve_token))
        .build()
        .unwrap();

    let mut mailer = SmtpTransportBuilder::new((smtp_host, smtp_port)).unwrap()
        .credentials(smtp_username, smtp_password)
        .security_level(SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Plain)
        .connection_reuse(true)
        .build();

    let result = mailer.send(email);

    if result.is_ok() {

        update_retrieve(&*username, &*retrieve_token);

        data.message = "验证邮件已发送，该邮件的有效时间为24小时，请注意查收！".to_string();
        data.data = json!("/");
    } else {
        data.success = false;
        data.message = "验证邮件发送失败，请重新尝试！".to_string();
    }

    respond_json(&data)
}

pub fn render_set_new_password(req: &mut Request) -> IronResult<Response> {

    let params = get_request_query(req);
    let username = &*params.get("username").unwrap()[0];
    let token = &*params.get("token").unwrap()[0];

    let mut data = ViewData::new(req);

    if token == "" {

        data.insert("retrieve_message", json!("该验证地址已失效！"));
        return respond_view("new-password", &data);
    }

    let retrieve_time_wrapper = get_retrieve_time(username, token);


    if retrieve_time_wrapper.is_none() {

        data.insert("retrieve_message", json!("该验证地址已失效！"));
    } else {

        let now = gen_datetime().timestamp();
        let one_day = 60 * 60 * 24;
        let retrieve_time = retrieve_time_wrapper.unwrap().timestamp();

        if now - retrieve_time > one_day {  // 该地址未验证时间超过24小时
            data.insert("retrieve_message", json!("该验证地址已失效！"));
        } else {
            data.insert("username", json!(username));
        }
    }

    respond_view("new-password", &data)
}

pub fn set_new_password(req: &mut Request) -> IronResult<Response> {

    let params = get_request_body(req);
    let username = &params.get("username").unwrap()[0];
    let new_password = &params.get("newPassword").unwrap()[0];

    let mut data = JsonData::new();

    let result = update_password(username, new_password);

    if result.is_none() {

        data.success = false;
        data.message = "新密码设置失败，请重新尝试！".to_owned();
    } else {

        update_retrieve(username, "");  // 清空retrieve_token，使得验证地址失效
        data.message = "新密码设置成功，即将前往登录页！".to_owned();
        data.data = json!("/login");
    }

    req.session().clear().unwrap();

    respond_json(&data)
}
