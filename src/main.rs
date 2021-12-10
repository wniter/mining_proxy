use anyhow::Result;
use clap::crate_name;
use log::{debug, info};
use tokio::sync::mpsc;

mod client;
mod mine;
mod protocol;
mod util;

use util::{config, get_app_command_matches, logger};

use client::{tcp, tls};

use crate::mine::Mine;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = get_app_command_matches().await?;
    let config_file_name = matches.value_of("config").unwrap_or("default.yaml");
    let config = config::Settings::new(config_file_name)?;
    logger::init(crate_name!(), config.log_path.clone(), config.log_level)?;
    info!("✅ config init success!");
    let mine = Mine::new(config.clone()).await?;
    let (tx, mut rx) = mpsc::channel::<String>(50);
    
    let _ = tokio::join!(
        tcp::accept_tcp(config.clone(), tx.clone()),
        tls::accept_tcp_with_tls(config.clone(), tx.clone()),
        mine.accept(tx.clone(), rx),
    );

    Ok(())
}
