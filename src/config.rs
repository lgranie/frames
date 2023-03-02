use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub movies: Movies,
    pub tv_shows: TvShows,
    pub downloaders: HashMap<String, DownloaderConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Movies {
    dir: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TvShows {
    dir: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DownloaderConfig {
    type_id: DownloaderType,
    pub api_url: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum DownloaderType {
    Transmission,
    Deluge,
}
