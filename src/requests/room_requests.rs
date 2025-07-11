use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub is_private: bool,
}