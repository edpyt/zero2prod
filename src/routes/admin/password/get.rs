use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

use crate::{
    session_state::TypedSession,
    utils::{e500, see_other},
};

pub async fn change_password_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    }

    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
            <title>Change Password</title>
        </head>
        <body>
            {msg_html}
            <form action="/admin/password" method="post">
                <label>Current Password
                    <input
                        type="password"
                        name="current_password"
                        placeholder="Enter current password"
                    >
                </label>
                <br>
                <label>New Password
                    <input
                        type="password"
                        name="new_password"
                        placeholder="Enter new password"
                    >
                </label>
                <br>
                <label>Confirm New Password
                    <input
                        type="password"
                        name="confirm_new_password"
                        placeholder="Type the new password again"
                    >
                </label>
                <br>
                <button type="submit">Change Password</button>
            </form>
            <p><a href="/admin/dashboard">&lt;- Back</a></p>
        </body>
        </html>
        "#,
        )))
}
