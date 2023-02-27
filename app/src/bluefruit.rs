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
    if let Err(error) = handle_messages(&tx).await {
        tracing::error!(error);
    }
}

async fn handle_messages(tx: &SyncSender<Message>) -> Result<(), Box<dyn Error>> {
    tracing::info!("Connecting to bluefruit...");
    let bluefruit = get_bluefruit().await?;
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

async fn get_bluefruit() -> Result<Bluefruit, Box<dyn Error>> {
    tracing::info!("Getting bluetooth adapters...");
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        return Err("No adapters detected!".into());
    }

    for adapter in adapter_list {
        tracing::info!("Scanning for devices with {:?}", adapter);
        adapter.start_scan(ScanFilter::default()).await?;
        time::sleep(Duration::from_secs(10)).await;
        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty() {
            tracing::warn!("No peripherals were found... Please ensure Bluetooth is turned on!");
            continue;
        }

        tracing::trace!("Peripherals found: {:?}", peripherals);

        // All peripheral devices in range
        for peripheral in peripherals {
            let properties = peripheral.properties().await?;
            if let Some(true) = properties.and_then(|x| {
                Some(matches!(
                    x.local_name.as_deref(),
                    Some("Adafruit Bluefruit LE"),
                ))
            }) {
                tracing::info!("Bluefruit found!");
                return Ok(peripheral);
            }
        }
    }
    Err("Bluefruit not found".into())
}
