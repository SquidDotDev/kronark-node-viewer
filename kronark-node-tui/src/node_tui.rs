use crate::socket_tui::SocketTui;

#[derive(Debug, PartialEq, Clone)]
pub struct NodeTui {
    name: String,
    x: i32,
    y: i32,
    sockets: Vec<SocketTui>,
    color: ratatui::style::Color,
    type_index: u8,
}