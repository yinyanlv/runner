use iron::prelude::*;
use lettre::transport::smtp::{SmtpTransport, SmtpTransportBuilder};
use lettre::email::EmailBuilder;
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

    println!("{:?}", email_str);

    let email = EmailBuilder::new()
        .to(email_str)
        .from("user@localhost")
        .subject("Hello")
        .body("请点击此处<a href=\"http://localhost:3000\" target=\"_blank\">重置密码</a>")
        .build()
        .unwrap();

    let mut mailer =
        SmtpTransportBuilder::localhost().unwrap().build();

    let result = mailer.send(email);

    println!("{:?}", result);

    if result.is_ok() {

        data.message = "验证邮件已发送，请注意查收！".to_string();
    } else {
        data.success = false;
        data.message = "验证邮件发送失败，请重新尝试！".to_string();
    }

    respond_json(&data)
}