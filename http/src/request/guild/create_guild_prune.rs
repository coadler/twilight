use crate::request::prelude::*;
use std::{
    borrow::{Borrow, Cow},
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    guild::GuildPrune,
    id::{GuildId, RoleId},
};

/// The error created when the guild prune can not be created as configured.
#[derive(Clone, Debug)]
pub enum CreateGuildPruneError {
    /// The number of days is 0.
    DaysInvalid,
}

impl Display for CreateGuildPruneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::DaysInvalid => f.write_str("the number of days is invalid"),
        }
    }
}

impl Error for CreateGuildPruneError {}

#[derive(Default)]
struct CreateGuildPruneFields<'a> {
    compute_prune_count: Option<bool>,
    days: Option<u64>,
    include_roles: Option<Cow<'a, [RoleId]>>,
}

/// Begin a guild prune.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/guild#begin-guild-prune
pub struct CreateGuildPrune<'a> {
    fields: CreateGuildPruneFields<'a>,
    guild_id: GuildId,
    fut: Option<Pending<'a, Option<GuildPrune>>>,
    http: &'a Client,
    reason: Option<Cow<'a, str>>,
}

impl<'a> CreateGuildPrune<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: CreateGuildPruneFields::default(),
            fut: None,
            guild_id,
            http,
            reason: None,
        }
    }

    /// List of roles to include when pruning.
    pub fn include_roles(mut self, roles: impl Into<Cow<'a, [RoleId]>>) -> Self {
        self.fields.include_roles.replace(roles.into());

        self
    }

    /// Return the amount of pruned members. Discouraged for large guilds.
    pub fn compute_prune_count(mut self, compute_prune_count: bool) -> Self {
        self.fields.compute_prune_count.replace(compute_prune_count);

        self
    }

    /// Set the number of days that a user must be inactive before being pruned.
    ///
    /// The number of days must be greater than 0.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildPruneError::DaysInvalid`] if the number of days is 0.
    ///
    /// [`CreateGuildPruneError::DaysInvalid`]: enum.CreateGuildPruneError.html#variant.DaysInvalid
    pub fn days(mut self, days: u64) -> Result<Self, CreateGuildPruneError> {
        if !validate::guild_prune_days(days) {
            return Err(CreateGuildPruneError::DaysInvalid);
        }

        self.fields.days.replace(days);

        Ok(self)
    }

    /// Attach an audit log reason to this request.
    pub fn reason(mut self, reason: impl Into<Cow<'a, str>>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let include_roles = self.fields.include_roles.take();
        let include_roles = include_roles.as_deref();

        let request = if let Some(reason) = self.reason.take() {
            let headers = audit_header(reason.borrow())?;

            Request::from((
                headers,
                Route::CreateGuildPrune {
                    compute_prune_count: self.fields.compute_prune_count,
                    days: self.fields.days,
                    guild_id: self.guild_id.0,
                    include_roles,
                },
            ))
        } else {
            Request::from(Route::CreateGuildPrune {
                compute_prune_count: self.fields.compute_prune_count,
                days: self.fields.days,
                guild_id: self.guild_id.0,
                include_roles,
            })
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreateGuildPrune<'_>, Option<GuildPrune>);