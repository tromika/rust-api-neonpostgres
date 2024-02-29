use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_person_handler, delete_person_handler, edit_person_handler, get_person_handler,
        health_checker_handler, person_list_handler,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/people/", post(create_person_handler))
        .route("/api/people", get(person_list_handler))
        .route(
            "/api/people/:id",
            get(get_person_handler)
                .patch(edit_person_handler)
                .delete(delete_person_handler),
        )
        .with_state(app_state)
}
