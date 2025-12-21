use axum::{
    Router,
    routing::{delete, get, post, put},
};
use registry::AppRegistry;

use crate::handler::user::{
    change_password, change_role, delete_user, get_current_user, list_users, register_user,
};

pub fn build_user_router() -> Router<AppRegistry> {
    let router = Router::new()
        .route("/me", get(get_current_user))
        .route("/me/password", put(change_password))
        .route("/", get(list_users))
        .route("/", post(register_user))
        .route("/:user_id", delete(delete_user))
        .route("/:user_id/role", put(change_role));

    Router::new().nest("/users", router)
}
