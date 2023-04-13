use std::path::PathBuf;
use std::{io::Write, path::Path};

use crate::config::Config;
use anyhow::Result;
use chrono::Local;
use env_logger::{Builder, Env};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};

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

pub async fn init_logger(config: &Config) -> Result<()> {
    let log_path = if let Some(path) = &config.log_path {
        path
    } else {
        panic!("Can not read log path from config")
    };
    let log_level = config.log_level.clone().unwrap();

    let now = Local::now();
    let formatted = format!("{}.log", now.format("%Y-%m-%d"));
    let file_path = PathBuf::from(&log_path).join(formatted);
    create_folder(&file_path).await?;

    let env = Env::default().filter_or("RUA_LOG_LEVEL", &log_level);
    let mut builder = Builder::from_env(env);

    builder
        .format(move |buf, record| {
            let formatted = format!("{}", now.format("%Y-%m-%d %H:%M:%S"));
            let log = format!("{} - {} - {}", formatted, record.level(), record.args());
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