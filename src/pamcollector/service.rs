use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};
use pamcollector::metric::Metric;
use std::io;
pub struct MetricInputService {
    pub tx: Arc<Mutex<SyncSender<Vec<u8>>>>,
}

impl Service for MetricInputService {
    // These types must match the corresponding protocol types:
    type Request = Metric;
    type Response = String;
    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;
    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;
    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        let _ = self.tx
            .lock()
            .unwrap()
            .try_send(req.to_val().join(", ").into_bytes());
        future::ok(format!("")).boxed()
    }
}
