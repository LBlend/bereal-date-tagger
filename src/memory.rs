use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    pub front_image: Image,
    pub back_image: Image,
    is_late: bool,
    date: DateTime<Utc>,
    pub taken_time: DateTime<Utc>,
    bereal_moment: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub path: String,
    width: u16,
    bucket: String,
    height: u16,
    mime_type: String,
}
