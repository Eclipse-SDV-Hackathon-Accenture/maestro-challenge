use crate::commands::COMMANDS;
use async_process::Command;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::{task::JoinHandle, time::sleep};
use tokio_util::sync::CancellationToken;

pub type ItemCache = Mutex<HashMap<String, HashMap<String, String>>>;

pub fn init_resource_statistic_cache() -> (Arc<ItemCache>, JoinHandle<()>, CancellationToken) {
    let cache = Arc::new(ItemCache::default());

    let cache_sweep_cancel = CancellationToken::new();
    (
        cache.clone(),
        tokio::spawn(fetch_statistics(cache, cache_sweep_cancel.clone())),
        cache_sweep_cancel,
    )
}

async fn exec_command(cmd: &str) -> anyhow::Result<String> {
    let result = Command::new("sh").args(["-c", cmd]).output().await?;
    Ok(String::from_utf8(result.stdout)?)
}

async fn fetch_statistics(cache: Arc<ItemCache>, stop_signal: CancellationToken) {
    let host = exec_command("hostname")
        .await
        .unwrap_or("unknown".to_string())
        .strip_suffix('\n')
        .unwrap()
        .to_owned();

    loop {
        for (typ, cmd) in COMMANDS.iter() {
            match Command::new("sh").args(["-c", cmd]).output().await {
                Ok(result) => {
                    let output_stdout = String::from_utf8(result.stdout).unwrap();

                    match cache.lock().unwrap().entry(host.clone()) {
                        std::collections::hash_map::Entry::Vacant(new_value) => {
                            log::debug!("Inserting new resource statistic value '{}' with value '{}' for new host '{}'", typ, output_stdout, host);
                            let mut new_map = HashMap::new();
                            new_map.insert(typ.to_string(), output_stdout);
                            new_value.insert(new_map);
                        }
                        std::collections::hash_map::Entry::Occupied(existing_value) => {
                            log::debug!("Inserting new resource statistic value '{}' with value '{}' for existing host '{}'", typ, output_stdout, host);
                            existing_value
                                .into_mut()
                                .insert(typ.to_string(), output_stdout);
                        }
                    }
                }
                Err(err) => log::error!("{}", err),
            }
        }

        tokio::select! {
            _ = sleep(Duration::from_secs(5)) => {
                continue;
            }

            _ = stop_signal.cancelled() => {
                log::info!("gracefully shutting down resource statistics job");
                break;
            }
        };
    }
}
