use crate::{
    id::GuildId,
    voice::VoiceState,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceStateUpdate {
    pub guild_id: Option<GuildId>,
    pub voice_state: VoiceState,
}