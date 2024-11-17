use kronark_node_parser::kronarknode::instance::Instance;

use crate::{errors::NodeConversionError, socket_tui::SocketTui};

#[derive(Debug, PartialEq, Clone)]
pub struct NodeTui {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub sockets: Vec<SocketTui>,
    pub color: ratatui::style::Color,
    pub type_index: u8,
}

impl NodeTui {
    pub fn from_instance(instance: Instance) -> Result<Self, NodeConversionError> {
        match instance.node_type {
            _ => Err(NodeConversionError::UnknownNodeType)
        }
    }
}