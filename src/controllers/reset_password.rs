use iron::prelude::*;
use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SecurityLevel, SmtpTransport, SmtpTransportBuilder};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::EmailTransport;

use common::http::*;
use common::utils::*;

pub fn render_reset_password(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);

    respond_view("reset-password", &data)
}

pub fn send_reset_password_email(req: &mut Request) -> IronResult<Response> {

    let params = get_request_body(req);
    let email_str = &*params.get("email").unwrap()[0];
    let mut data = JsonData::new();

    let email = EmailBuilder::new()
        .to(email_str)
        .from("rustchina@163.com")
        .subject("重置密码")
        .html(r#"
            <p style="padding:10px;">
                请点击
                <a style="text-decoration:underline;color:#1e90ff;" href="http://localhost:3000" target="_blank">http://localhost:3000</a>
                ，进行密码重置！
            </p>
            "#)
        .build()
        .unwrap();

    let mut mailer = SmtpTransportBuilder::new(("smtp.163.com", 25)).unwrap()
        .credentials("rustchina@163.com", "runner111111")
        .security_level(SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Plain)
        .connection_reuse(true)
        .build();

    let result = mailer.send(email);

    if result.is_ok() {

        data.message = "验证邮件已发送，请注意查收！".to_string();
        data.data = json!("/");
    } else {
        data.success = false;
        data.message = "验证邮件发送失败，请重新尝试！".to_string();
    }

    respond_json(&data)
}