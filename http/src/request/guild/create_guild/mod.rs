use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType},
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, PartialGuild, Permissions,
        VerificationLevel,
    },
    id::{ChannelId, RoleId},
};

mod builder;

pub use self::builder::*;

/// The error returned when the guild can not be created as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CreateGuildError {
    /// The name of the guild is either fewer than 2 UTF-16 characters or more than 100 UTF-16
    /// characters.
    NameInvalid {
        /// Provided name.
        name: String,
    },
    /// The number of channels provided is too many.
    ///
    /// The maximum amount is 500.
    TooManyChannels {
        /// Provided channels.
        channels: Vec<GuildChannelFields>,
    },
    /// The number of roles provided is too many.
    ///
    /// The maximum amount is 250.
    TooManyRoles {
        /// Provided roles.
        roles: Vec<RoleFields>,
    },
}

impl Display for CreateGuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameInvalid { .. } => f.write_str("the guild name is invalid"),
            Self::TooManyChannels { .. } => f.write_str("too many channels were provided"),
            Self::TooManyRoles { .. } => f.write_str("too many roles were provided"),
        }
    }
}

impl Error for CreateGuildError {}

#[derive(Serialize)]
struct CreateGuildFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    channels: Option<Vec<GuildChannelFields>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_message_notifications: Option<DefaultMessageNotificationLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explicit_content_filter: Option<ExplicitContentFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<RoleFields>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_level: Option<VerificationLevel>,
}

/// Role fields sent to Discord.
///
/// Use [`RoleFieldsBuilder`] to build one.
///
/// [`RoleFieldsBuilder`]: struct.RoleFieldsBuilder.html
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RoleFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    pub id: RoleId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
}

impl From<RoleFieldsBuilder> for RoleFields {
    /// Convert a [`RoleFieldsBuilder`] into a [`RoleFields`].
    ///
    /// This is equivalent to calling [`RoleFieldsBuilder::build`].
    ///
    /// [`RoleFieldsBuilder`]: struct.RoleFieldsBuilder.html
    /// [`RoleFields`]: struct.RoleFields.html
    /// [`RoleFieldsBuilder::build`]: struct.RoleFieldsBuilder.html#method.build
    fn from(builder: RoleFieldsBuilder) -> Self {
        builder.build()
    }
}

/// Variants of channel fields sent to Discord.
///
/// Use [`GuildChannelFieldsBuilder`] to build one.
///
/// [`GuildChannelFieldsBuilder`]: struct.GuildChannelFieldsBuilder.html
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum GuildChannelFields {
    Category(CategoryFields),
    Text(TextFields),
    Voice(VoiceFields),
}

impl GuildChannelFields {
    pub fn id(self) -> ChannelId {
        match self {
            Self::Category(c) => c.id,
            Self::Text(t) => t.id,
            Self::Voice(v) => v.id,
        }
    }
}

/// Category channel fields sent to Discord.
///
/// Use [`CategoryFieldsBuilder`] to build one.
///
/// [`CategoryFieldsBuilder`]: struct.CategoryFieldsBuilder.html
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CategoryFields {
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
}

/// Text channel fields sent to Discord.
///
/// Use [`TextFieldsBuilder`] to build one.
///
/// [`TextFieldsBuilder`]: struct.TextFieldsBuilder.html
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TextFields {
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

impl From<TextFieldsBuilder> for TextFields {
    fn from(builder: TextFieldsBuilder) -> TextFields {
        builder.build()
    }
}

/// Voice channel fields sent to Discord.
///
/// Use [`VoiceFieldsBuilder`] to build one.
///
/// [`VoiceFieldsBuilder`]: struct.VoiceFieldsBuilder.html
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VoiceFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u64>,
}

impl From<VoiceFieldsBuilder> for VoiceFields {
    fn from(builder: VoiceFieldsBuilder) -> VoiceFields {
        builder.build()
    }
}

/// Create a new request to create a guild.
///
/// The minimum length of the name is 2 UTF-16 characters and the maximum is 100 UTF-16 characters.
/// This endpoint can only be used by bots in less than 10 guilds.
///
/// # Errors
///
/// Returns [`CreateGuildError::NameInvalid`] if the name length is too short or too long.
///
/// [`CreateGuildError::NameInvalid`]: enum.CreateGuildError.html#variant.NameInvalid
pub struct CreateGuild<'a> {
    fields: CreateGuildFields,
    fut: Option<Pending<'a, PartialGuild>>,
    http: &'a Client,
}

impl<'a> CreateGuild<'a> {
    pub(crate) fn new(http: &'a Client, name: impl Into<String>) -> Result<Self, CreateGuildError> {
        Self::_new(http, name.into())
    }

    fn _new(http: &'a Client, name: String) -> Result<Self, CreateGuildError> {
        if !validate::guild_name(&name) {
            return Err(CreateGuildError::NameInvalid { name });
        }

        Ok(Self {
            fields: CreateGuildFields {
                channels: None,
                default_message_notifications: None,
                explicit_content_filter: None,
                icon: None,
                name,
                region: None,
                roles: None,
                verification_level: None,
            },
            fut: None,
            http,
        })
    }

    /// Add a role to the list of roles.
    pub fn add_role(mut self, role: impl Into<RoleFields>) -> Self {
        if self.fields.roles.is_none() {
            let builder = RoleFieldsBuilder::new("@everyone");
            self.fields.roles.replace(vec![builder.build()]);
        }

        if let Some(roles) = self.fields.roles.as_mut() {
            roles.push(role.into());
        }

        self
    }

    /// Set the channels to create with the guild.
    ///
    /// The maximum number of channels that can be provided is 500.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use twilight_http::{
    ///     Client,
    ///     request::guild::create_guild::{
    ///         GuildChannelFieldsBuilder, CategoryFieldsBuilder, TextFieldsBuilder,
    ///         VoiceFieldsBuilder,
    ///     },
    /// };
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("my token");
    ///
    /// let text = TextFieldsBuilder::new("text channel")?;
    /// let voice = VoiceFieldsBuilder::new("voice channel")?;
    /// let text2 = TextFieldsBuilder::new("other text channel")?
    ///     .topic("posting")?;
    ///
    /// let category = CategoryFieldsBuilder::new("category channel")?
    ///     .add_text(text2)
    ///     .add_voice(voice);
    ///
    /// let channels = GuildChannelFieldsBuilder::new()
    ///     .add_text(text)
    ///     .add_category_builder(category)
    ///     .build();
    ///
    /// let guild = client.create_guild("guild name")?.channels(channels)?.await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildError::TooManyChannels`] if the number of channels is over 500.
    ///
    /// [`CreateGuildError::TooManyChannels`]: enum.CreateGuildError.html#variant.TooManyChannels
    pub fn channels(mut self, channels: Vec<GuildChannelFields>) -> Result<Self, CreateGuildError> {
        // Error 30013
        // <https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#json>
        if channels.len() > 500 {
            return Err(CreateGuildError::TooManyChannels { channels });
        }

        self.fields.channels.replace(channels);

        Ok(self)
    }

    /// Set the default message notification level. Refer to [the discord docs] for more
    /// information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/guild#create-guild
    pub fn default_message_notifications(
        mut self,
        default_message_notifications: DefaultMessageNotificationLevel,
    ) -> Self {
        self.fields
            .default_message_notifications
            .replace(default_message_notifications);

        self
    }

    /// Set the explicit content filter level.
    pub fn explicit_content_filter(
        mut self,
        explicit_content_filter: ExplicitContentFilter,
    ) -> Self {
        self.fields
            .explicit_content_filter
            .replace(explicit_content_filter);

        self
    }

    /// Set the icon.
    ///
    /// This must be a Data URI, in the form of `data:image/{type};base64,{data}` where `{type}` is
    /// the image MIME type and `{data}` is the base64-encoded image. Refer to [the discord docs]
    /// for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/reference#image-data
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.fields.icon.replace(icon.into());

        self
    }

    /// Override the everyone role of the guild.
    ///
    /// If there are not yet roles set with [`roles`], this will create a role override in the
    /// first position. Discord understands the first role in the list to override @everyone.
    /// If there are roles, this replaces the first role in the position.
    ///
    /// [`roles`]: #method.roles
    pub fn override_everyone(mut self, everyone: impl Into<RoleFields>) -> Self {
        if let Some(roles) = self.fields.roles.as_mut() {
            roles.remove(0);
            roles.insert(0, everyone.into());
        } else {
            self.fields.roles.replace(vec![everyone.into()]);
        }

        self
    }

    /// Specify the voice server region for the guild. Refer to [the discord docs] for more
    /// information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/voice#voice-region-object
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.fields.region.replace(region.into());

        self
    }

    /// Set the roles to create with the guild.
    ///
    /// The maximum number of roles that can be provided is 250.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use twilight_http::{Client, request::guild::create_guild::RoleFieldsBuilder};
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("my token");
    ///
    /// let roles = vec![RoleFieldsBuilder::new("role 1").color(0x543923)?.build()];
    /// client.create_guild("guild name")?.roles(roles)?.await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildError::TooManyRoles`] if the number of roles is
    /// over 250.
    ///
    /// [`CreateGuildError::TooManyRoles`]: enum.CreateGuildError.html#variant.TooManyRoles
    pub fn roles(mut self, mut roles: Vec<RoleFields>) -> Result<Self, CreateGuildError> {
        if roles.len() > 250 {
            return Err(CreateGuildError::TooManyRoles { roles });
        }

        if let Some(prev_roles) = self.fields.roles.as_mut() {
            roles.insert(0, prev_roles.remove(0));
        } else {
            let builder = RoleFieldsBuilder::new("@everyone");
            roles.insert(0, builder.build());
        }

        self.fields.roles.replace(roles);

        Ok(self)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            crate::json_to_vec(&self.fields)?,
            Route::CreateGuild,
        )))));

        Ok(())
    }
}

poll_req!(CreateGuild<'_>, PartialGuild);
