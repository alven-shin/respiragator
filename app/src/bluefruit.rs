// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use futures::StreamExt;
use std::error::Error;
use std::sync::mpsc::SyncSender;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;

use crate::Message;

type Bluefruit = btleplug::platform::Peripheral;

const UART_SERVICE_UUID: Uuid = Uuid::from_u128(0x6e400001_b5a3_f393_e0a9_e50e24dcca9e);
const UART_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x6e400003_b5a3_f393_e0a9_e50e24dcca9e);

pub async fn bluefruit_reciever(tx: SyncSender<Message>) {
    if let Err(e) = handle_messages(&tx).await {
        tx.send(Message::Log(e.to_string())).unwrap();
    }
}

async fn handle_messages(tx: &SyncSender<Message>) -> Result<(), Box<dyn Error>> {
    tx.send(Message::Log("Connecting to bluefruit...".to_owned()))?;
    let bluefruit = get_bluefruit(tx).await?;
    let is_connected = bluefruit.is_connected().await?;
    if !is_connected {
        bluefruit.connect().await?
    }

    let is_connected = bluefruit.is_connected().await?;
    bluefruit.discover_services().await?;

    let uart_service = bluefruit
        .services()
        .into_iter()
        .filter(|service| service.uuid == UART_SERVICE_UUID)
        .next()
        .unwrap();
    let uart_characteristic = uart_service
        .characteristics
        .into_iter()
        .filter(|x| x.uuid == UART_CHARACTERISTIC_UUID)
        .next()
        .unwrap();

    bluefruit.subscribe(&uart_characteristic).await?;
    let mut notifs = bluefruit.notifications().await?;

    while let Ok(Some(data)) = tokio::time::timeout(Duration::from_secs(5), notifs.next()).await {
        // let x = String::from_utf8_lossy(&data.value);
        // print!("{x}");
        if let Some(value) = data.value.last().copied() {
            tx.send(Message::ResistanceValue(value))?;
        }
    }

    if is_connected {
        bluefruit.disconnect().await?
    }
    Ok(())
}

async fn get_bluefruit(tx: &SyncSender<Message>) -> Result<Bluefruit, Box<dyn Error>> {
    tx.send(Message::Log("Getting bluetooth adapters...".to_owned()))?;
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        tx.send(Message::Log("No adapters found!".to_owned()))?;
        return Err("No adapters detected!".into());
    }

    for adapter in adapter_list {
        tx.send(Message::Log(format!(
            "Scanning for devices with {:?}",
            adapter
        )))?;
        adapter.start_scan(ScanFilter::default()).await?;
        time::sleep(Duration::from_secs(10)).await;
        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty() {
            tx.send(Message::Log(
                "No peripherals were found... Please ensure Bluetooth is turned on!".to_owned(),
            ))?;
            continue;
        }

        tx.send(Message::Log(format!(
            "Peripherals found: {:?}",
            peripherals
        )))?;

        // All peripheral devices in range
        for peripheral in peripherals {
            let properties = peripheral.properties().await?;
            if let Some(true) = properties.and_then(|x| {
                Some(matches!(
                    x.local_name.as_deref(),
                    Some("Adafruit Bluefruit LE"),
                ))
            }) {
                tx.send(Message::Log("Bluefruit found!".to_owned()))?;
                return Ok(peripheral);
            }
        }
    }
    Err("Bluefruit not found".into())
}
