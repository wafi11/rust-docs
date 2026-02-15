use serde::Serialize;

#[derive(Serialize)]
pub struct RootResponse {
    pub timestamp : String,
    pub message :&'static str,
    pub status : i16
}