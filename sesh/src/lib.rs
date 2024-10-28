#[cfg(feature = "zip")]
mod zip;

use std::{net::IpAddr, time::Duration};

use serde::Deserialize;
use time::OffsetDateTime;
#[cfg(feature = "zip")]
pub use zip::*;

mod json;

pub enum Error {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

mod offline_timestamp_serde {
    use serde::Deserialize;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<time::OffsetDateTime>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ms = i128::deserialize(deserializer)?;
        if ms == 0 {
            Ok(None)
        } else {
            time::OffsetDateTime::from_unix_timestamp_nanos(ms * 1_000_000)
                .map_err(serde::de::Error::custom)
                .map(Some)
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stream {
    /// Timestamp of when the stream ended.
    #[serde(with = "time::serde::rfc3339")]
    pub ts: OffsetDateTime,
    pub username: String,
    pub platform: String,
    /// Number of milliseconds the stream was played.
    pub ms_played: u64,
    /// Country code of where the stream was played.
    pub conn_country: String,
    /// IP address of where the stream was played.
    pub ip_addr_decrypted: Option<IpAddr>,
    pub user_agent_decrypted: Option<String>,
    /// The name of the track that was played.
    pub master_metadata_track_name: Option<String>,
    /// The name of the artist, band or podcast.
    pub master_metadata_album_artist_name: Option<String>,
    /// The name of the album.
    pub master_metadata_album_album_name: Option<String>,
    pub spotify_track_uri: Option<String>,
    /// Podcast episode name.
    pub episode_name: Option<String>,
    /// Podcast show name.
    pub episode_show_name: Option<String>,
    pub spotify_episode_uri: Option<String>,
    pub reason_start: Option<String>,
    pub reason_end: Option<String>,
    pub shuffle: Option<bool>,
    pub skipped: Option<bool>,
    pub offline: Option<bool>,
    #[serde(with = "offline_timestamp_serde")]
    pub offline_timestamp: Option<OffsetDateTime>,
    pub incognito_mode: Option<bool>,
}

impl Stream {
    pub const fn playtime(&self) -> Duration {
        Duration::from_millis(self.ms_played)
    }
}
