use actix_web::Scope;

mod access;
mod accessto;
mod allow;
mod change_email;
mod change_password;
mod create;
mod delete;
mod disallow;
mod invite;
mod login;
mod regeneratetoken;
mod rename;
mod resend_verify;
mod set_status;

pub fn scope() -> Scope {
    Scope::new("/v1")
        .service(create::create)
        .service(delete::delete)
        .service(login::login)
        .service(regeneratetoken::regenerate_token)
        .service(rename::rename)
        .service(set_status::set_status)
        .service(change_password::changepw)
        .service(change_email::change_email)
        .service(resend_verify::resend_verify)
        .service(allow::allow)
        .service(accessto::accessto)
        .service(access::access)
        .service(disallow::disallow)
        .service(invite::invite)
}
