use std::{env, num::ParseIntError};

use crate::{
    constants::config::server::{default_value, var_name},
    scheduler::{Scheduler, tasks::CronTask},
};
use macros::error_panic;

use axum::Router;
use tokio::net::TcpListener;
use tracing::{info, warn};

pub mod blog;

// todo: merge `get_server_router` & `get_mount_point`, make Option
pub trait Module {
    fn get_server_router(&self) -> Router;

    fn get_mount_point(&self) -> &'static str;

    fn get_cron_tasks(&self) -> Vec<(&str, Box<dyn CronTask>)>;
}

pub struct ModuleTree {
    pub modules: Vec<Box<dyn Module>>,
}

impl ModuleTree {
    pub fn new() -> Self {
        ModuleTree { modules: vec![] }
    }

    pub fn add_module(&mut self, module: impl Module + 'static) -> &mut Self {
        self.modules.push(Box::new(module));
        self
    }

    pub async fn blocking_serve(&self) {
        let mut scheduler = Scheduler::new();
        let mut app = Router::new();

        let port: u32 = match env::var(var_name::PORT) {
            Ok(value) => value.parse().unwrap_or_else(|e: ParseIntError| {
                warn!(
                    "failed to parse `{}`, using default port: {}",
                    var_name::PORT,
                    e.to_string()
                );
                default_value::PORT
            }),
            Err(_) => default_value::PORT,
        };

        let listener = match TcpListener::bind(format!("{}:{}", default_value::HOST, port)).await {
            Ok(listener) => {
                info!("server starting on {}:{}", default_value::HOST, port);
                listener
            }
            Err(e) => {
                error_panic!(
                    "failed to start server on {}:{}: {}",
                    default_value::HOST,
                    port,
                    e.to_string()
                );
            }
        };

        for module in &self.modules {
            let _ = scheduler.insert_list(module.get_cron_tasks());
            app = app.nest(module.get_mount_point(), module.get_server_router());
        }

        tokio::spawn(async move {
            scheduler.start().await;
        });

        axum::serve(listener, app).await.unwrap_or_else(|e| {
            error_panic!("failed to start axum: {}", e.to_string());
        });
    }
}
