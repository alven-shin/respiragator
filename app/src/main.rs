// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use futures::StreamExt;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;

const UART_SERVICE: Uuid = Uuid::from_u128(0x6e400003_b5a3_f393_e0a9_e50e24dcca9e);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // pretty_env_logger::init();

    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan on {}...", adapter.adapter_info().await?);
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(10)).await;
        let peripherals = adapter.peripherals().await?;
        if peripherals.is_empty() {
            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
            continue;
        }

        // All peripheral devices in range
        let mut peripherals = peripherals.into_iter();
        let (bluefruit, local_name) = loop {
            let Some(peripheral) = peripherals.next() else {break None};
            let properties = peripheral.properties().await?;
            let is_connected = peripheral.is_connected().await?;
            let Some(local_name) = properties.unwrap().local_name else {continue};
            if local_name == "Adafruit Bluefruit LE" {
                println!(
                    "Peripheral {:?} is connected: {:?}",
                    local_name, is_connected
                );
                break Some((peripheral, local_name));
            } else {
                continue;
            }
        }
        .unwrap();
        let is_connected = bluefruit.is_connected().await?;
        if !is_connected {
            println!("Connecting to bluefruit {:?}...", &local_name);
            if let Err(err) = bluefruit.connect().await {
                eprintln!("Error connecting to bluefruit, skipping: {}", err);
                continue;
            }
        }
        let is_connected = bluefruit.is_connected().await?;
        println!(
            "Now connected ({:?}) to bluefruit {:?}...",
            is_connected, &local_name
        );
        bluefruit.discover_services().await?;
        println!("Discover peripheral {:?} services...", &local_name);
        for service in bluefruit.services() {
            println!(
                "Service UUID {}, primary: {}",
                service.uuid, service.primary
            );
            for characteristic in service.characteristics {
                println!("\t{:?}", characteristic);
                if characteristic.uuid == UART_SERVICE {
                    bluefruit.subscribe(&characteristic).await?;
                    let mut notif_stream = bluefruit.notifications().await?.take(4);
                    while let Some(data) = notif_stream.next().await {
                        let x = String::from_utf8_lossy(&data.value);
                        dbg!(x);
                    }
                }
                // if characteristic.properties.contains(CharPropFlags::READ) {
                //     let x = bluefruit.read(&characteristic).await.unwrap();
                //     let x = String::from_utf8_lossy(&x);
                //     dbg!(x);
                // }
            }
        }

        if is_connected {
            println!("Disconnecting from bluefruit {:?}...", &local_name);
            bluefruit
                .disconnect()
                .await
                .expect("Error disconnecting from bluefruit");
        }
    }
    Ok(())
}
