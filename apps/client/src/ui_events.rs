#[derive(Debug)]
pub enum UiEvent {
    ConnectionStatus(String),
    ConnectionLost,
}
