use service::Request;

use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use log::error;
use anyhow::{Result, Error};
use reqwest::{header::{HeaderMap, HeaderValue}, StatusCode};
use warp::{filters::BoxedFilter, reject::{self, Rejection}, reply::Reply, Filter};
use tokio::sync::mpsc::{self, Receiver, Sender};

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

    let (service_handle, sender): (JoinHandle<()>, Sender<Request>) = service::start_service();

    let with_service = |sender: Sender<service::Request>| warp::any().map(move || sender);

    let run = warp::path!("run")
        .and(warp::post())
        .and(with_service(sender.clone()))
        .and(warp::body::json::<models::BabyModel>())
        .and_then(reply_run);

}

async fn reply_run(sender: Sender<Request>, model: models::BabyModel) -> Result<impl warp::Reply, Infallible> {
    let (confirm_sender, confirm_recv) = mpsc::channel<bool>(100);

}
