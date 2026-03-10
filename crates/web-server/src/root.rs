use crate::errors::CustomError;
use axum::{response::Html, Extension};
use web_pages::root;
use clorinde::deadpool_postgres::Pool;

pub async fn loader(
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = clorinde::queries::users::get_users()
        .bind(&client)
        .all()
        .await?;

    let html = root::index(users);

    Ok(Html(html))
}
