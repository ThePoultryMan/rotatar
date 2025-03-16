use std::sync::Arc;

use async_channel::{Receiver, Sender};

#[derive(Debug, Clone)]
pub enum Message {
    Ready(Sender<Arc<Receiver<Message>>>),
    CurrentImageChanged,
}
