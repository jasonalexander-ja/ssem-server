use mosquitto_rs::*;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::mpsc::{error::TryRecvError, error::TrySendError};
use base64::{Engine as _, engine::general_purpose};
use baby_emulator::core::{MEMORY_WORDS, BabyModel};
use std::time::SystemTime;
use tokio::task::JoinHandle;

use crate::config::MqttConfig;

pub enum Request {
    Run(BabyModel, Sender<bool>),
    Cancel,
    Exit
}

pub fn start_service(mqtt: MqttConfig) -> (JoinHandle<()>, Sender<Request>) {
    let (send, recv) = mpsc::channel::<Request>(1024);
    let handle = tokio::spawn(async move { service_routine(mqtt, recv).await; });
    (handle, send)
}

async fn service_routine(mqtt: MqttConfig, rec: Receiver<Request>) {
    let mut rec = rec;
    loop {
        match rec.recv().await {
            Some(Request::Exit) => return,
            Some(Request::Cancel) => continue,
            Some(Request::Run(model, sender)) => {
                if let Err(TrySendError::Closed(_)) = sender.try_send(true) { return };
                if !run_model(mqtt.clone(), model, &mut rec).await {
                    return
                }
            },
            None => return 
        }
    }
}

async fn run_model(mqtt: MqttConfig, model: BabyModel, rec: &mut Receiver<Request>) -> bool {
    let mut start = SystemTime::now();
    let mut model = if let Ok(v) = model.execute() { v } else { return true };
    display_model(&mqtt, &model);
    loop {
        let time = if let Ok(v) = SystemTime::now().duration_since(start) { v.as_secs() } else { return true };
        if time >= 1 {
            start = SystemTime::now();
            model = if let Ok(v) = model.execute() { v } else { return true };
            display_model(&mqtt, &model);
        }
        match rec.try_recv() {
            Err(TryRecvError::Empty) => continue,
            Err(TryRecvError::Disconnected) | Ok(Request::Exit) => return false,
            Ok(Request::Cancel) => return true,
            Ok(Request::Run(_, s)) => {
                if let Err(TrySendError::Closed(_)) = s.try_send(false) { return false }
            }
        }
    }
}

fn display_model(mqtt: &MqttConfig, model: &BabyModel) {
    let mut result: Vec<u8> = vec![0; 84];
    for i in 0..MEMORY_WORDS {
        result[i] = model.main_store[i] as u8;
    }
    result[MEMORY_WORDS] = model.accumulator as u8;
    result[MEMORY_WORDS + 1] = (model.instruction_address & 0xFF) as u8;
    result[MEMORY_WORDS + 2] = ((model.instruction >> 8) + (model.instruction & 0x1F)) as u8;
    let b64 = general_purpose::STANDARD.encode(result);
    let _ = publish_image(b64, mqtt);
    println!("{:?}", model)
}

async fn publish_image(value: String, mqtt: &MqttConfig) -> Result<(), mosquitto_rs::Error> {
    let client = Client::with_auto_id()?;
    let _rc = client.connect(&mqtt.address, 1883, std::time::Duration::from_secs(5), None).await?;
    client.publish(&mqtt.topic, value, QoS::AtMostOnce, false)
        .await?;
    Ok(())
}

