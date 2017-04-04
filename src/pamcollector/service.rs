use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};
use serde_json;
use pamcollector::metric::Metric;
use std::io;
pub struct Echo {
    pub tx: Arc<Mutex<SyncSender<Vec<u8>>>>,
}

impl Service for Echo {
    // These types must match the corresponding protocol types:
    type Request = Metric;
    type Response = String;
    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;
    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;
    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        // let rev: String = req.chars().rev().collect();
        let rencoded = serde_json::to_vec(&req).unwrap();
        self.tx.lock().unwrap().try_send(rencoded);
        future::ok(format!("OK")).boxed()
    }
}

