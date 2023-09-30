#![warn(clippy::all, rust_2018_idioms)]

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Project {
    code: String,
}
pub mod app;
pub use app::VideoEditor;
