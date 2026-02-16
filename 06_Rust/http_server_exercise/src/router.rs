use axum::{Router, routing::get};
use crate::api::{get_students, get_student, add_student, update_student, delete_student};
use crate::SharedState;

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/students", get(get_students).post(add_student))
        .route("/students/{id}", get(get_student).put(update_student).delete(delete_student))
        .with_state(state)
}
