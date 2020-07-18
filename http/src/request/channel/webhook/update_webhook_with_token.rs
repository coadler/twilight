use crate::json_to_vec;
use crate::request::prelude::*;
use std::borrow::Cow;
use twilight_model::{channel::Webhook, id::WebhookId};

#[derive(Default, Serialize)]
struct UpdateWebhookWithTokenFields<'a> {
    avatar: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
}

/// Update a webhook, with a token, by ID.
pub struct UpdateWebhookWithToken<'a> {
    fields: UpdateWebhookWithTokenFields<'a>,
    fut: Option<Pending<'a, Webhook>>,
    http: &'a Client,
    token: Cow<'a, str>,
    webhook_id: WebhookId,
}

impl<'a> UpdateWebhookWithToken<'a> {
    pub(crate) fn new(
        http: &'a Client,
        webhook_id: WebhookId,
        token: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            fields: UpdateWebhookWithTokenFields::default(),
            fut: None,
            http,
            token: token.into(),
            webhook_id,
        }
    }

    /// Set the avatar of the webhook.
    ///
    /// See [Discord Docs/Image Data] for more information. This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type and `{data}` is the
    /// base64-encoded image.
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub fn avatar(mut self, avatar: impl Into<Cow<'a, str>>) -> Self {
        self.fields.avatar.replace(avatar.into());

        self
    }

    /// Change the name of the webhook.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self {
        self.fields.name.replace(name.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            json_to_vec(&self.fields)?,
            Route::UpdateWebhook {
                token: Some(&self.token),
                webhook_id: self.webhook_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateWebhookWithToken<'_>, Webhook);