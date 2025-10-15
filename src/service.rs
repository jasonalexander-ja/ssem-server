
use tokio::sync::mpsc::{self, Receiver, Sender};
use baby_emulator::core::BabyModel;
use std::{sync::mpsc::{SendError, TryRecvError, TrySendError}, time::{SystemTime, UNIX_EPOCH}};


pub enum Request {
    Run(BabyModel, Sender<bool>),
    Cancel,
    Exit
}

pub async fn start_service() -> (JoinHandle<()>, Sender<Request>) {
    let (send, recv) = mpsc::channel::<Request>(1024);
    let handle = tokio::spawn(async move { service_routine(recv).await; });
    (handle, send)
}

async fn service_routine(rec: Receiver<Request>) {
    loop {
        match rec.recv().await {
            Some(Request::Exit) => return,
            Some(Request::Cancel) => continue,
            Some(Request::Run(model, sender)) => {
                if let Err(TrySendError::Disconnected(_)) = sender.send(true).await { return };
                if !run_model(model, &rec).await {
                    return
                }
            },
            None => return 
        }
    }
}

async fn run_model(model: BabyModel, rec: &Receiver<Request>) -> bool {
    let start = SystemTime::now();
    model = if let Ok(v) = model.execute() { v } else { return true };
    display_model(&model);
    loop {
        let time = if let Ok(v) = SystemTime::now().duration_since(start) { v.as_secs() } else { return true };
        if time >= 2 {
            let start = SystemTime::now();
            model = if let Ok(v) = model.execute() { v } else { return true };
            display_model(&model);
        }
        match rec.try_recv().await {
            Err(TryRecvError::Empty) => continue,
            Err(TryRecvError::Disconnected) | Ok(Request::Exit) => return false,
            Ok(Request::Cancel) => return true,
            Ok(Request::Run(_, s)) => {
                if let Err(TrySendError::Disconnected(_)) = s.try_send(true).await { return false }
            }
        }
    }
}

async fn display_model(model: &BabyModel) {
    println!("{:?}", model)
}

