use shuttle_secrets::SecretStore;
use trial::AppState;
use sqlx::postgres::PgPoolOptions;


#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&secrets.get("DATABASE_URL").unwrap())
        .await
        .expect("Unable to load database_url");

    let state = AppState::new(pool);
    
    let router = trial::create_router(state);

    Ok(router.into())
}
