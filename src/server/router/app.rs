use std::fs::File;
use std::io::prelude::*;

use crate::server::ResponseBody;

use super::render::{render_app_data_text, render_app_data_bin};

pub fn get_manifest() -> ResponseBody {
    let mut file = File::open("src/frontend/manifest.json").expect("");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("");
    render_app_data_text("application/manifest+json",contents)
}

pub fn get_logo() -> ResponseBody {
    let mut file = File::open("src/frontend/images/logo.png").expect("logo not found");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("failed to read image");
    render_app_data_bin("image/png", contents)
}