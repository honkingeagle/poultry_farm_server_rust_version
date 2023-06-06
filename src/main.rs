use poultry_farm_server::AppState;
use shuttle_secrets::SecretStore;
use sqlx::postgres::PgPoolOptions;

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&secrets.get("DATABASE_URL").unwrap())
        .await
        .expect("Unable to load database_url");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Unable to migrate sql files");

    let state = AppState::new(pool);

    let router = poultry_farm_server::create_router(state);

    Ok(router.into())
}
