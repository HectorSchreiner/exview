use uuid::Uuid;

pub struct User {
    username: String,
    password: String,
    id: Uuid
}