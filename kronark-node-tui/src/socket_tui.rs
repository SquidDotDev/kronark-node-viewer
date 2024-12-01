use std::{collections::HashSet, ops::RangeInclusive};

use kronark_node_parser::kronarknode::socket::{DataType, Socket, SocketFlags, SocketType};
use ratatui::buffer::Buffer;
use ratatui::style::Color;

use crate::utils::{color_line, data_get_constant, format_text_center, format_text_left, format_text_right, write_line};

#[derive(Debug, PartialEq, Clone)]
pub enum Additional {
    None,
    Text { minimum: String, maximum: String, valid: String },
    Number { minimum: String, maximum: String, step: String },
    Switch { on: String, off: String },
    Select { options: HashSet<String> },
}

impl Additional {
    fn get_switch_string(&self, value: bool) -> String {
        if let Additional::Switch {on, off} = self {
            if value { on.clone() } else { off.clone() }
        } else { if value { "on".to_string() } else { "off".to_string() } }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SocketDefault {
    None,
    String(String),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SocketTui {
    pub name: String,
    pub socket: Socket,
    pub default: SocketDefault,
    pub additional: Additional,
    pub connective: bool,
}

impl SocketTui {
    pub fn get_size(&self) -> usize {
        match &self.socket.flags.get_type() {
            SocketType::IncomingNamed | SocketType::OutgoingNamed => 0,
            SocketType::IncomingNumber | SocketType::IncomingText => get_data_size(&self.socket.data),
            SocketType::IncomingSelect => get_data_size(&self.socket.data) + 2,
            SocketType::IncomingSwitch => self.additional.get_switch_string(self.socket.flags.is_switch_on()).len() + 2,
            
        }
    }

    pub fn render(&self, transform: SocketTuiTransform, buf: &mut Buffer) {
        if !self.socket.flags.get_type().is_incoming() {
            if let SocketDefault::Bool(_) = self.default {
                self.render_repetitive_end(transform.x+1, transform.y, transform.width - 2, buf);
                return;
            }
            color_line(transform.x+1, transform.x+1 + transform.width as i32 - 2, transform.y, Color::Black, Color::White, buf); 
            write_line(transform.x+1, transform.x+1 + transform.width as i32 - 2, transform.y, format_text_right(&self.name, transform.width - 2), buf); 
            write_line(transform.x+1 + transform.width as i32 - 2, transform.x+1 + transform.width as i32 - 1, transform.y, "⬤".to_string(), buf); 
            return;
        }

        color_line(transform.x+1, transform.x+1 + transform.width as i32 - 2, transform.y, Color::Black, Color::White, buf); 
        write_line(transform.x+1, transform.x+1 + transform.width as i32 - 2, transform.y, format_text_left(&self.name, transform.name_width), buf); 
        if self.connective{
            write_line(transform.x, transform.x+1, transform.y, "⬤".to_string(), buf); 
        }

        if self.socket.flags.is_connected() {
            return;
        }

        let (x, width) = if self.name.is_empty() {
            (transform.x + 1, transform.width - 2)
        } else {
            (transform.x + 1 + transform.name_width as i32, transform.width - transform.name_width - 2)
        };

        match self.socket.flags.get_type() {
            SocketType::IncomingNamed | SocketType::OutgoingNamed => (),
            SocketType::IncomingText => self.render_text(x, transform.y, width, buf),
            SocketType::IncomingNumber => self.render_number(x, transform.y, width, buf),
            SocketType::IncomingSelect => self.render_select(x, transform.y, width, buf),
            SocketType::IncomingSwitch => self.render_switch(x, transform.y, width, buf),
        }
    }

    fn render_repetitive_end(&self, x: i32, y: i32, width: usize, buf: &mut Buffer) {
        color_line(x, x + width as i32, y, Color::DarkGray, Color::White, buf);
        let button_width = (width as i32 - 2) / 3;
        color_line(x + button_width+1, x + 2*button_width +1 as i32, y, Color::Black, Color::White, buf);
        color_line(x + 2*button_width +2, x + 3*button_width +2, y, Color::Black, Color::White, buf);
        write_line(x + (button_width as f32 * 0.5) as i32, x + (button_width as f32 * 0.5) as i32 + 1, y, "↪".to_string(), buf);
        write_line(x +(button_width as f32 * 1.5) as i32 +1, x + (button_width as f32 * 1.5) as i32 + 2, y, "+".to_string(), buf);
        write_line(x + (button_width as f32 * 2.5) as i32 +2, x + (button_width as f32 * 2.5) as i32 + 3, y, "-".to_string(), buf);
    }

    fn render_text(&self, x: i32, y: i32, width: usize, buf: &mut Buffer) {
        color_line(x, x + width as i32, y, Color::White, Color::Black, buf);
        write_line(x, x + width as i32, y, format_text_left(&data_get_constant(&self.socket.data).unwrap_or(String::new()), width), buf);
    }

    fn render_number(&self, x: i32, y: i32, width: usize, buf: &mut Buffer) {
        color_line(x, x + width as i32, y, Color::White, Color::Black, buf);
        write_line(x, x + width as i32, y, format_text_left(&data_get_constant(&self.socket.data).unwrap_or(String::new()), width), buf);
    }

    fn render_select(&self, x: i32, y: i32, width: usize, buf: &mut Buffer) {
        color_line(x, x + width as i32, y, Color::White, Color::Black, buf);
        let mut switch_string = "☰ ".to_string();
        switch_string.push_str(&data_get_constant(&self.socket.data).unwrap_or(String::new()));
        
        write_line(x, x + width as i32, y, format_text_center(&switch_string, width), buf); 
    }

    fn render_switch(&self, x: i32, y: i32, width: usize, buf: &mut Buffer) {
        let mut switch_string = "☼ ".to_string();
        switch_string.push_str(&self.additional.get_switch_string(self.socket.flags.is_switch_on()));
        if self.socket.flags.is_switch_on() {
            color_line(x, x + width as i32, y, Color::Gray, Color::Black, buf);
        } else {
            color_line(x, x + width as i32, y, Color::White, Color::Black, buf);
        }

        write_line(x, x + width as i32, y, format_text_center(&switch_string, width), buf); 
    }

    pub fn is_repetitive_end(&self) -> bool {
        if !self.socket.flags.get_type().is_incoming() {
            if let SocketDefault::Bool(_) = self.default {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SocketTuiTransform {
    pub width: usize,
    pub name_width: usize,
    pub y: i32,
    pub x: i32, 
}

fn get_data_size(data: &Option<DataType>) -> usize {
    data_get_constant(data).unwrap_or(String::new()).len()
}


pub fn RepetiveSocket() -> SocketTui {
    let socket = Socket {
        flags: SocketFlags::from_byte(0, 0).unwrap(),
        type_index: 0,
        port_slot: 1,
        data: None,
    };
    SocketTui {
        name: "repetitive end".to_string(),
        socket: socket,
        default: SocketDefault::Bool(true),
        additional: Additional::None,
        connective: false,
    }
}