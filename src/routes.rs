use crate::core::*;
use rocket::request::LenientForm;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/v2rayN2quan?<url>&<config..>")]
pub fn v2rayn_to_quan(url: String, config: Option<LenientForm<ConvertConfig>>) -> String {
    let cfg_str = read_cfg_url(&url).unwrap();
    match config {
        Some(cfg) => convert_cfg_str(&cfg_str, &cfg),
        None => convert_cfg_str(&cfg_str, &DEFAULT_CONVERT_CFG),
    }
}
