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
            eprintln!("Unable to load database_url: {err}");
            process::exit(1);
        });

    sqlx::migrate!().run(&pool).await.unwrap_or_else(|err| {
        eprintln!("Unable to migrate sql files: {err}");
        process::exit(1);
    });

    let smtp_email = secrets.get("SMTP_EMAIL").unwrap();
    let smtp_password = secrets.get("SMTP_PASSWORD").unwrap();
    // Changed backed to AppState::new()
    let state = AppState::new(pool, smtp_email, smtp_password);

    let router = poultry_farm_server::create_router(state);

    Ok(router.into())
}
