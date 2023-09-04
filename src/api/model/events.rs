use sea_orm::prelude::Json;

pub struct CreateEvent {
    pub name: String,
    pub info: Option<Json>,
}
