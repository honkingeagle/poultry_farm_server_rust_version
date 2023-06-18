use poultry_farm_server::AppState;
use shuttle_secrets::SecretStore;
use sqlx::postgres::PgPoolOptions;
use std::process;

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&secrets.get("DATABASE_URL").unwrap())
        .await
        .unwrap_or_else(|err| {
            println!("Unable to load database_url: {err}");
            process::exit(1);
        });

    sqlx::migrate!()
        .run(&pool)
        .await
        .unwrap_or_else(|err| {
            println!("Unable to migrate sql files: {err}");
            process::exit(1);
        });

    let state = AppState::new(pool);

    let router = poultry_farm_server::create_router(state);

    Ok(router.into())
}
