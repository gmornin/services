use actix_web::{http::header::ContentType, HttpResponse};
use goodmorning_bindings::{
    structs::{ServicesTriggerTypes, ServicesTriggers},
    traits::SerdeAny,
};
use serde::{Deserialize, Serialize};

use crate::traits::Peekable;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceTriggerWrapper(pub ServicesTriggers);

#[typetag::serde(name = "service triggers")]
impl SerdeAny for ServiceTriggerWrapper {
    fn exit_status(&self) -> u16 {
        self.0.exit_status()
    }
}

impl Peekable for ServiceTriggerWrapper {
    fn to_html(&self) -> Option<actix_web::HttpResponse> {
        let html = match &self.0.value {
            ServicesTriggerTypes::EmailVerification {
                email,
                username,
                id,
            } => {
                let email = html_escape::encode_safe(&email);
                let username = html_escape::encode_safe(&username);
                format!(
                    r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="shortcut icon" href="/static/services/images/favicon-dark.svg" />
    <link rel="stylesheet" href="/static/services/css/triggernotfound.css" />
    <link rel="stylesheet" href="/static/services/css/dark/triggernotfound.css" />
    <script src="/static/services/scripts/trigger.js"></script>
    <title>Verify email</title>
  </head>
  <body>
    <center id="dialog">
      <img src="/static/services/images/favicon-dark.svg" width="100" id="icon" />
      <br />
      <h1>Is this your email?</h1>
      Email: {email}<br />
      Username: {username}<br />
      User ID: {id}
      <div id="buts">
        <button class="greenbut" onclick="trigger()">Yes, verify this account.</button>
        <button class="redbut" onclick="revoke()">No, he's an imposter.</button>
      </div>
      <p id="output"></p>
    </center>
    <center id="output"></center>
  </body>
</html>"#
                )
            }
        };

        Some(
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html),
        )
    }
}
