use serde::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum AudioStatus {
    Ready,
    Closed,
    Polling,
}
