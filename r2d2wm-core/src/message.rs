use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Message {
    pub id: NonZeroU64,
    pub content: String,
    pub guild_id: NonZeroU64,
    pub channel_id: NonZeroU64,
}
