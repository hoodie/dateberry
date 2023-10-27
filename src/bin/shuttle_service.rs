use dateberry::{setup_router, record_start_time};

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    record_start_time();
    let router = setup_router();

    Ok(router.into())
}
