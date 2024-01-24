use std::{
    env,
    fs::{self, DirEntry},
    io::Error, os::unix::thread,
};

use chrono::offset::Utc;
use chrono::DateTime;

use axum::{http::StatusCode, response::IntoResponse, Json};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::error::ExternalExecutionError;

#[instrument]
fn get_storage_path() -> String {
    env::var("event_store").unwrap_or("/home/jan/projects/rust-catan/.storage".to_string())
}

#[instrument]
pub async fn list() -> Result<impl IntoResponse, ExternalExecutionError> {
    let dir = fs::read_dir(get_storage_path()).unwrap();
    let files: Vec<Game> = dir.into_iter().map(Game::from).collect();
    Ok((StatusCode::OK, Json(files)))
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub last_modified: String,
}

impl From<Result<DirEntry, Error>> for Game {
    #[instrument]
    fn from(value: Result<DirEntry, Error>) -> Self {
        let rng: u64 =  rand::thread_rng().gen_range(600..1000);
        std::thread::sleep(std::time::Duration::from_millis(rng));
        
        let dir = match value {
            Ok(val) => val,
            Err(_) => {
                return Game {
                    name: "N/A".to_string(),
                    last_modified: "N/A".to_string(),
                }
            }
        };

        let name = dir.file_name().to_str().unwrap().to_string();
        let modified = dir.metadata().unwrap().modified().unwrap();
        let datetime: DateTime<Utc> = modified.into();

        Game {
            name,
            last_modified: datetime.format("%m/%d/%y %H:%M:%S").to_string(),
        }
    }
}

// TODO: Rename
