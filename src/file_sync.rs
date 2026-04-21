extern crate core;
extern crate rocket;
use core::custom_styles::custom_style::CustomStyle;
use core::custom_styles::serial_custom_style::SerialCustomStyle;
use core::parsec::parsec_controls::ParsecControls;
use core::pvp_settings::ParsecConfig;
use core::rest::syncher::{SyncResponse, SynchRequest};
use http::Status;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{options::ClientOptions, Client};
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::State;
use rocket::*;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn find_first_zip(version: String) -> Option<String> {
    let dir = format!("/files/{}", version);
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();

        if path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("zip"))
            == Some(true)
        {
            return Some(path.to_string_lossy().to_string());
        }
    }

    None
}
fn filename(path: &str) -> Option<String> {
    Path::new(path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
}
#[post("/sync/check/<id>", data = "<req>")]
pub async fn sync_check(req: Json<SynchRequest>, id: &str) -> Result<Json<SyncResponse>, Status> {
    let server_file_path = find_first_zip(id.to_string()).ok_or(Status::NotFound)?;
    let server_filename = filename(&server_file_path).ok_or(Status::NotFound)?;

    Ok(Json(SyncResponse {
        needs_update: req.local_zip_file != server_filename,
        filename: server_filename,
    }))
}
#[get("/sync/download/<version>")]
pub async fn download(version: &str) -> Result<NamedFile, Status> {
    let path = find_first_zip(version.to_string()).ok_or(Status::NotFound)?;
    NamedFile::open(path).await.map_err(|_| Status::NotFound)
}

