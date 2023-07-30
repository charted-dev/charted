// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    config::Config,
    protos::{
        emails_server::Emails, value::Kind, ListValue, PingRequest, PingResponse, SendEmailRequest, SendEmailResponse,
    },
    registry::TemplateRegistry,
};
use charted_common::{hashmap, COMMIT_HASH, VERSION};
use eyre::{eyre, Result};
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Address, AsyncSmtpTransport, AsyncTransport,
    Message, Tokio1Executor,
};
use mustache::Data;
use std::collections::HashMap;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct Service {
    templates: TemplateRegistry,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl Service {
    pub async fn new() -> Result<Service> {
        info!("creating SMTP mailer service!");

        let config = Config::get();
        let mut mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp.host)?.port(config.smtp.port);

        match (config.smtp.username.clone(), config.smtp.password()?.clone()) {
            (Some(username), Some(password)) => {
                mailer = mailer.clone().credentials(Credentials::new(username, password));
            }

            (Some(_), None) => {
                return Err(eyre!("missing `config.smtp.password` configuration key"));
            }

            (None, Some(_)) => return Err(eyre!("missing `config.smtp.username` configuration key")),
            _ => {}
        }

        let mailer = mailer.build();
        mailer.test_connection().await?;

        let registry = TemplateRegistry::new(config.templates.clone());
        registry.init()?;

        Ok(Service {
            templates: registry,
            mailer,
        })
    }
}

#[async_trait]
impl Emails for Service {
    async fn ping(&self, _request: Request<PingRequest>) -> tonic::Result<Response<PingResponse>, Status> {
        Ok(Response::new(PingResponse { pong: true }))
    }

    async fn send(&self, request: Request<SendEmailRequest>) -> tonic::Result<Response<SendEmailResponse>, Status> {
        let config = Config::get();
        let request = request.get_ref();

        let from = match config.smtp.from.parse::<Address>() {
            Ok(addr) => addr,
            Err(e) => {
                error!("cannot parse from address [{}]: {e}", config.smtp.from);
                sentry::capture_error(&e);

                return Err(Status::internal("Internal Server Error"));
            }
        };

        let to = match request.to.parse::<Address>() {
            Ok(addr) => addr,
            Err(e) => {
                error!("cannot parse to address [{}]: {e}", request.to);
                sentry::capture_error(&e);

                return Err(Status::internal("Internal Server Error"));
            }
        };

        debug!("sending email request to address {}", request.to);
        let content = match (request.content.clone(), request.template.clone()) {
            (Some(_), Some(_)) => {
                return Err(Status::invalid_argument(
                    "cannot use both 'content' and 'template' together",
                ))
            }
            (Some(content), _) => content,
            (_, Some(template)) => {
                debug!("using template {template}");
                match self.templates.find(template.clone()).await {
                    Ok(true) => {}
                    Ok(false) => return Err(Status::invalid_argument(format!("Template {template} was not found"))),
                    Err(e) => {
                        error!("unable to find template {template}: {e}");
                        return Err(Status::internal("Internal Server Error"));
                    }
                }

                let context = match request.context.clone() {
                    Some(s) => prost_value_to_data(Kind::StructValue(s)),
                    None => Data::Map(hashmap!()),
                };

                self.templates.render(template, context).await.map_err(|e| {
                    error!("Unable to compile and render template: {e}");
                    Status::internal("Internal Server Error")
                })?
            }
            _ => {
                return Err(Status::invalid_argument(
                    "missing `request.content` or `request.template` in request",
                ))
            }
        };

        let message = Message::builder()
            .from(Mailbox::new(None, from))
            .to(Mailbox::new(None, to.clone()))
            .subject(request.subject.clone())
            .date_now()
            .user_agent(format!(
                "Noelware/charted-emails (+https://github.com/charted-dev/charted; v{VERSION}+{COMMIT_HASH}"
            ))
            .body(content.clone())
            .map_err(|e| {
                sentry::capture_error(&e);
                Status::internal(format!("Unable to create message: {e}"))
            })?;

        match self.mailer.send(message).await {
            Ok(_) => Ok(Response::new(SendEmailResponse {
                success: true,
                should_retry: false,
                error_message: None,
            })),

            Err(e) => {
                error!("Unable to send email to [{to}]: {e}");
                sentry::capture_error(&e);

                return Err(Status::internal(format!("cannot send email to {to}: {e}")));
            }
        }
    }
}

fn prost_value_to_data(value: Kind) -> Data {
    match value {
        Kind::BoolValue(b) => Data::Bool(b),
        Kind::NullValue(_) => Data::Null,
        Kind::NumberValue(float) => Data::String(float.to_string()),
        Kind::StringValue(s) => Data::String(s),
        Kind::StructValue(s) => {
            let mut res = HashMap::new();
            for (key, value) in s.fields {
                if value.kind.is_none() {
                    warn!(
                        "key [{key}] with value kind {:?} couldn't be determined, skipping!",
                        value.kind
                    );

                    continue;
                }

                res.insert(key, prost_value_to_data(value.kind.clone().unwrap()));
            }

            Data::Map(res)
        }

        Kind::ListValue(ListValue { values }) => {
            let mut res: Vec<Data> = vec![];
            for (index, val) in values.iter().enumerate() {
                if val.kind.is_none() {
                    warn!(
                        "value kind in index #{index} ({:?}) couldn't be determined, will be skipped",
                        val.kind
                    );

                    continue;
                }

                res.push(prost_value_to_data(val.kind.clone().unwrap()));
            }

            Data::Vec(res)
        }
    }
}
