use crate::core::{ConvertConfig, VmessConfig};
use reqwest;
use rocket::request::LenientForm;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/v2rayN2quan?<url>&<config..>")]
fn v2rayN2quan(url: String, config: Option<LenientForm<ConvertConfig>>) {
    unimplemented!()
}
