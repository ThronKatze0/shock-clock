use tauri::async_runtime;
use tokio::sync::mpsc;
use uuid::{uuid, Uuid};

const DEV_NAME: &str = "Shock Clock";
const SHOCK_FLAG: Uuid = uuid!("2986fdbe-26d6-4127-92e3-392644d38121"); // TODO: change to correct one

async fn scan() {
    async_runtime::block_on(async {
        let handler = tauri_plugin_blec::get_handler().unwrap();
        let (tx, mut rx) = mpsc::channel(1);
        handler.lock().await.discover(Some(tx), 1000).await.unwrap();
        while let Some(devices) = rx.recv().await {
            for dev in devices.iter() {
                if dev.name == DEV_NAME && !dev.is_connected {
                    println!("Found clock: {}", dev.address);
                    let mut handler = handler.lock().await;
                    match handler
                        .connect(dev.address.clone(), Some(|| println!("disconnected!")))
                        .await
                    {
                        Ok(_) => println!("Successfully connected!"),
                        Err(err) => println!("Error during connection: {err}"),
                    }
                }
            }
        }
    });
}

#[tauri::command]
pub fn shock() -> Result<(), ()> {
    let mut res = Err(());
    async_runtime::block_on(async {
        let mut handler = tauri_plugin_blec::get_handler().unwrap().lock().await;
        if let Ok(_) = handler.connected_device().await {
            if let Err(err) = handler.send_data(SHOCK_FLAG, &[1u8]).await {
                eprintln!("While sending data: {err}");
            }
            res = Ok(());
        }
    });
    res
}
