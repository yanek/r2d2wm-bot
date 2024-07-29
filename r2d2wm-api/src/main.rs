use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};
use routes::{delete_task, get_guilds_tasks, get_task_by_id, post_task};
use tracing_subscriber::EnvFilter;

mod data;
mod routes;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/tasks", post(post_task))
        .route("/tasks/:id", get(get_task_by_id))
        .route("/tasks/:id", delete(delete_task))
        .route("/tasks/guilds/:guild_id", get(get_guilds_tasks));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
