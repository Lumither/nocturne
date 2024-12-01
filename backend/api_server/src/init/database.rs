use std::env;
use std::error::Error;
use std::str::FromStr;

use constants::config::database::{default_value, var_name};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, Pool, Postgres};
use tracing::info;

pub async fn init() -> Result<Pool<Postgres>, Box<dyn Error>> {
    let db_connect_option = PgConnectOptions::from_str(&parse_db_uri())
        .unwrap()
        .disable_statement_logging();
    Ok(PgPoolOptions::new()
        .max_connections(1) // todo: temporal solution, to be fixed
        .connect_with(db_connect_option)
        .await?)
}

fn parse_db_uri() -> String {
    env::var(var_name::URI).unwrap_or_else(|_| {
        info!(
            "`{}` env var is not set, \
            using `{}`, `{}`, `{}`, `{}` and `{}`",
            var_name::URI,
            var_name::USERNAME,
            var_name::PASSWORD,
            var_name::HOST,
            var_name::PORT,
            var_name::DBNAME
        );
        let postgres_user = env::var(var_name::USERNAME).unwrap_or(default_value::USERNAME.into());
        let postgres_password =
            env::var(var_name::PASSWORD).unwrap_or(default_value::PASSWORD.into());
        let db_host = env::var(var_name::HOST).unwrap_or(default_value::HOST.into());
        let db_port = env::var(var_name::PORT).unwrap_or(default_value::PORT.into());
        let postgres_db = env::var(var_name::DBNAME).unwrap_or(default_value::DBNAME.into());
        let db_type = default_value::DATABASE_TYPE;
        format!("{db_type}://{postgres_user}:{postgres_password}@{db_host}:{db_port}/{postgres_db}")
    })
}
