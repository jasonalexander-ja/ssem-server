use service::Request;
use config::AppSettings;
use models::{BabyModelDef, Assembly};

use std::{convert::Infallible, net::SocketAddr};

use log::error;
use anyhow::{Result, Error};
use baby_emulator::assembler::linker::LinkerData;
use warp::Filter;
use tokio::sync::mpsc::{self, Sender, error::{TrySendError}};
use baby_emulator::core::{BabyModel, instructions::BabyInstruction};
use tokio::task::JoinHandle;
use warp::http::{Response, StatusCode};

mod config;
mod service;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let appsettings = match AppSettings::new() {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to read .env file: \r\n{e}");
            return Err(Error::msg(e))
        }
    };

    let listen: SocketAddr = appsettings.listen.clone()
        .parse()?;

    let mqtt_config = appsettings.get_mqtt_config();
    let (service_handle, sender): (JoinHandle<()>, Sender<Request>) = service::start_service(mqtt_config);

    let with_service = |sender: Sender<service::Request>| warp::any().map(move || sender.clone());

    let run = warp::path!("run")
        .and(warp::post())
        .and(with_service(sender.clone()))
        .and(warp::body::json::<models::BabyModelDef>())
        .and_then(reply_run);

    let assembler = warp::path!("assemble")
        .and(warp::post())
        .and(warp::body::json::<models::Assembly>())
        .and_then(reply_assemble);

    let cancel = warp::path!("cancel")
        .and(warp::post())
        .and(with_service(sender.clone()))
        .and_then(reply_cancel);

    let assemble_run = warp::path!("assemble_run")
        .and(warp::post())
        .and(with_service(sender.clone()))
        .and(warp::body::json::<models::Assembly>())
        .and_then(reply_assemble_run);

    let endpoints = run.or(assembler)
        .or(cancel)
        .or(assemble_run);

    warp::serve(endpoints)
        .run(listen)
        .await;

    let _ = sender.send(Request::Exit).await;
    let _ = service_handle.await;

    Ok(())
}

async fn reply_run(sender: Sender<Request>, model: BabyModelDef) -> Result<Response<String>, Infallible> {
    let (confirm_sender, mut confirm_recv) = mpsc::channel::<bool>(100);
    match sender.try_send(Request::Run(model.to_baby_model(), confirm_sender)) {
        Ok(_) => (),
        Err(TrySendError::Full(_)) => return response(StatusCode::SERVICE_UNAVAILABLE, "".to_owned()),
        Err(TrySendError::Closed(_)) => return response(StatusCode::INTERNAL_SERVER_ERROR, "".to_owned()),
    };
    return match confirm_recv.recv().await {
        Some(true) => response(StatusCode::OK, "".to_owned()),
        Some(false) => response(StatusCode::LOCKED, "".to_owned()),
        None => response(StatusCode::INTERNAL_SERVER_ERROR, "Emulation Service Stopped!".to_owned())
    }
}

async fn reply_assemble(asm: Assembly) -> Result<Response<String>, Infallible> {
    match baby_emulator::assembler::assemble(&asm.listing, asm.og_notation) {
        Err(e) => response(StatusCode::BAD_REQUEST, e.describe(true)),
        Ok(LinkerData(i, _)) => {
            let store = BabyInstruction::to_numbers(i);
            let model = BabyModelDef::from_baby_model(
                &BabyModel::new_with_program(store));
            let data = if let Ok(v) = serde_json::to_string(&model) { v }
                else { return response(StatusCode::INTERNAL_SERVER_ERROR, "".to_owned()) };
            response(StatusCode::OK, data)
        }
    }
}

async fn reply_cancel(sender: Sender<Request>) -> Result<Response<String>, Infallible> {
    match sender.try_send(Request::Cancel) {
        Ok(_) => response(StatusCode::OK, "".to_owned()),
        Err(_) => response(StatusCode::INTERNAL_SERVER_ERROR, "".to_owned()),
    }
}

async fn reply_assemble_run(sender: Sender<Request>, asm: Assembly) -> Result<Response<String>, Infallible> {
    let model = match baby_emulator::assembler::assemble(&asm.listing, asm.og_notation) {
        Err(e) => return response(StatusCode::BAD_REQUEST, e.describe(true)),
        Ok(LinkerData(i, _)) => {
            let store = BabyInstruction::to_numbers(i);
            BabyModel::new_with_program(store)
        }
    };
    reply_run(sender, BabyModelDef::from_baby_model(&model)).await
}

fn response(stat: StatusCode, body: String) -> Result<Response<String>, Infallible> {
    Ok(Response::builder()
        .status(stat)
        .body(body)
        .unwrap())
}
