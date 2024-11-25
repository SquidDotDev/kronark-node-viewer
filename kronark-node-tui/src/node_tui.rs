use kronark_node_parser::kronarknode::instance::Instance;
use ratatui::buffer::Buffer;
use ratatui::style::Color;

use crate::socket_tui::{SocketTuiTransform};
use crate::utils::{color_line, color_rect, format_text_center, write_line};
use crate::Camera;
use crate::{errors::NodeConversionError, socket_tui::SocketTui};
use crate::built_in::port::parse_port;

const MAX_NAME_SIZE: usize = 30;
const MIN_NAME_SIZE: usize = 30;
const MIN_SOCKET_NAME_SIZE: usize = 11;
const MAX_SOCKET_NAME_SIZE: usize = 11;
const MIN_SOCKET_DATA_SIZE: usize = 19;
const MAX_SOCKET_DATA_SIZE: usize = 19;

#[derive(Debug, PartialEq, Clone)]
pub struct NodeTui {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub sockets: Vec<SocketTui>,
    pub color: Color,
    pub type_index: u8,
    pub key: u8,
}

impl NodeTui {
    pub fn from_instance(instance: Instance) -> Result<Self, NodeConversionError> {
        match instance.node_type {
            255 => parse_port(instance),
            _ => Err(NodeConversionError::UnknownNodeType)
        }
    }

    pub fn render(&self, buf: &mut Buffer, camera: &Camera) {
        let (screen_x, screen_y) = camera.apply((self.x, self.y));

        let max_name = (self.name.len() + 2).min(MAX_NAME_SIZE).max(MIN_NAME_SIZE);
        let max_sockets_name = self.sockets.iter()
        .fold(MIN_SOCKET_NAME_SIZE, |a, s| {a.max(s.name.len() + 1)} )
        .min(MAX_SOCKET_NAME_SIZE);
        let max_sockets_value = self.sockets.iter()
        .fold(MIN_SOCKET_DATA_SIZE, |a, s| {a.max(s.get_size())} )
        .min(MAX_SOCKET_DATA_SIZE);
        let max_sockets_line = max_sockets_name + max_sockets_value;
        let width = std::cmp::max(max_sockets_line + 2, max_name + 2);


        color_rect(screen_x, screen_x + width as i32, screen_y, screen_y + self.get_y_size() as i32, self.color,  Color::White, buf);

        color_line(screen_x + 1, screen_x + 1 + max_name as i32, screen_y+1, Color::White, Color::Black, buf);
        write_line(screen_x + 1, screen_x + 1 + max_name as i32, screen_y+1, format_text_center(&self.name, max_name), buf);

        let mut socket_tranform = SocketTuiTransform {
            width,
            name_width: max_sockets_name,
            y: screen_y + 4,
            x: screen_x,
        };

        for socket in self.sockets.iter() {
            socket.render(socket_tranform.clone(), buf);
            socket_tranform.y += 2;
        }
        
    }

    pub fn get_y_size(&self) -> usize {
        self.sockets.len() as usize * 2 + 4
    }
}