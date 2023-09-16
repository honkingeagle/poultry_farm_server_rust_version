use poultry_farm_server::AppState;
use shuttle_secrets::SecretStore;
// use shuttle_shared_db::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::process;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
    // #[shuttle_shared_db::Postgres] _: PgPool,
) -> shuttle_axum::ShuttleAxum {

    // This pool is for development only.

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
    let frontend_url = secrets.get("DEV_FRONTEND_URL").unwrap();
    let state = AppState::new(pool, smtp_email, smtp_password, frontend_url);

    let router = poultry_farm_server::create_router(state);

    Ok(router.into())
}
