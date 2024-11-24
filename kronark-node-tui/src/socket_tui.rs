use std::ops::RangeInclusive;

use kronark_node_parser::kronarknode::socket::{DataType, Socket, SocketType};
use ratatui::buffer::Buffer;
use ratatui::style::Color;

use crate::utils::{color_line, data_get_constant, format_text_center, format_text_left, format_text_right, write_line};

#[derive(Debug, PartialEq, Clone)]
pub enum Additional {
    None,
    Text { minimum: String, maximum: String, valid: String },
    Number { minimum: String, maximum: String, step: String },
    Switch { on: String, off: String },
    Select { options: Vec<String> },
}

impl Additional {
    fn get_switch_string(&self, value: bool) -> String {
        if let Additional::Switch {on, off} = self {
            if value { on.clone() } else { off.clone() }
        } else { if value { "true".to_string() } else { "false".to_string() } }
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
    pub additional: Additional
}

impl SocketTui {
    pub fn get_size(&self) -> usize {
        match &self.socket.flags.get_type() {
            SocketType::IncomingNamed | SocketType::OutgoingNamed => 0,
            SocketType::IncomingNumber | SocketType::IncomingText => get_date_size(&self.socket.data),
            SocketType::IncomingSelect => get_date_size(&self.socket.data) + 2,
            SocketType::IncomingSwitch => self.additional.get_switch_string(self.socket.flags.is_switch_on()).len() + 2,
            
        }
    }

    pub fn render(&self, transform: SocketTuiTransform, buf: &mut Buffer) {
        if !self.socket.flags.get_type().is_incoming() {
           color_line(transform.x+1, transform.x+1 + transform.width as i32 - 2, transform.y, Color::Black, Color::White, buf); 
           write_line(transform.x+1, transform.x+1 + transform.width as i32 - 2, transform.y, format_text_right(&self.name, transform.width - 2), buf); 
           write_line(transform.x+1 + transform.width as i32 - 2, transform.x+1 + transform.width as i32 - 1, transform.y, "⬤".to_string(), buf); 
           return;
        }

        color_line(transform.x+1, transform.x+1 + transform.width as i32 - 2, transform.y, Color::Black, Color::White, buf); 
        write_line(transform.x+1, transform.x+1 + transform.width as i32 - 2, transform.y, format_text_left(&self.name, transform.name_width), buf); 
        write_line(transform.x, transform.x+1, transform.y, "⬤".to_string(), buf); 

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
}

#[derive(Debug, PartialEq, Clone)]
pub struct SocketTuiTransform {
    pub width: usize,
    pub name_width: usize,
    pub y: i32,
    pub x: i32, 
}

fn get_date_size(data: &Option<DataType>) -> usize {
    data_get_constant(data).unwrap_or(String::new()).len()
}
