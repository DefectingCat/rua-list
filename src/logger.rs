use std::path::PathBuf;
use std::process::exit;
use std::{io::Write, path::Path};

use anyhow::Result;
use chrono::Local;
use env_logger::{Builder, Env};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};

use crate::config::RConfig;

pub async fn create_folder(file_path: &Path) -> Result<()> {
    if file_path.exists() {
        return Ok(());
    } else {
        fs::create_dir_all(
            file_path
                .parent()
                .expect("Can not access log parent folder"),
        )
        .await?;
    }
    Ok(())
}

pub async fn init_logger(config: RConfig) -> Result<()> {
    let config = config.lock().await;
    let log_path = if let Some(path) = config.log_path.to_owned() {
        path
    } else {
        eprintln!("Can not read log path from config");
        exit(1);
    };
    let log_level = if let Some(level) = config.log_level.to_owned() {
        level
    } else {
        eprintln!("Can not read log level from config");
        exit(1);
    };

    let now = Local::now();
    let formatted = format!("{}.log", now.format("%Y-%m-%d"));
    let file_path = PathBuf::from(&log_path).join(formatted);
    create_folder(&file_path).await?;

    let env = Env::default().filter_or("RUA_LOG_LEVEL", &log_level);
    let mut builder = Builder::from_env(env);

    builder
        .format(move |buf, record| {
            let formatted = format!("{}", now.format("%Y-%m-%d %H:%M:%S"));
            let log = format!("{} - {} - {}\n", formatted, record.level(), record.args());
            writeln!(buf, "{log}")?;

            let file_path = file_path.clone();
            tokio::spawn(async move {
                let mut file = OpenOptions::new()
                    .write(true)
                    .read(true)
                    .create(true)
                    .append(true)
                    .open(&file_path)
                    .await
                    .expect("Can not write log file");
                file.write_all(log.as_bytes())
                    .await
                    .expect("Can not write log file");
            });
            Ok(())
        })
        .init();

    Ok(())
}