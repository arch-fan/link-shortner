use serde::Serialize;

#[derive(Serialize)]
pub struct OkResponse<T: Serialize> {
    pub body: T,
}

#[derive(Serialize)]
pub struct ErrResponse {
    pub message: String,
}
