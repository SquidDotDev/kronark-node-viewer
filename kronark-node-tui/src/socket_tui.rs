use std::ops::RangeInclusive;

use kronark_node_parser::kronarknode::socket::DataType;
use ratatui::buffer::Buffer;
use ratatui::style::Color;

use crate::utils::{color_line, color_rect, format_text_center, format_text_left, format_text_right, write_line};

#[derive(Debug, PartialEq, Clone)]
pub enum DataTypeTui {
    Output,
    Named,
    Text(String),
    Number { range: RangeInclusive<i64>, value: i64 },
    Switch { on: String, off: String, value: bool },
    Select { options: Vec<String>, value: String },
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataTui {
    pub data_type: DataTypeTui,
    pub connection: Option<Connection>
}

impl DataTui {
    pub fn get_size(&self) -> usize {
        match &self.data_type {
            DataTypeTui::Output | DataTypeTui::Named => 0,
            DataTypeTui::Number { value, .. } => value.to_string().len(),
            DataTypeTui::Switch { on, off, value } => if *value { on.len() + 2 } else { off.len() + 2 },
            DataTypeTui::Text(s) => s.len(),
            DataTypeTui::Select { value, .. } => value.len() + 2,
        }
    }

    pub fn is_output(&self) -> bool {
        if self.data_type == DataTypeTui::Output {
            return true;
        }
        false
    }

    pub fn render(&self, transform: SocketTuiTransform, no_name: bool, buf: &mut Buffer) {
        let (x, width) = if no_name {
            (transform.x + 1, transform.width - 2)
        } else {
            (transform.x + 1 + transform.name_width as i32, transform.width - transform.name_width - 2)
        };

        match &self.data_type {
            DataTypeTui::Named => (),
            DataTypeTui::Switch { on, off, value } => self.render_switch(x, transform.y, width, buf, on, off, value),
            DataTypeTui::Number { range, value } => self.render_number(x, transform.y, width, buf, range, value),
            DataTypeTui::Text(value) => self.render_text(x, transform.y, width, buf, value),
            DataTypeTui::Select { value, .. } => self.render_select(x, transform.y, width, buf, value),
            _ => (),
        }
    }

    fn render_text(&self, x: i32, y: i32, width: usize, buf: &mut Buffer, value: &String) {
        color_line(x, x + width as i32, y, Color::White, Color::Black, buf);
        write_line(x, x + width as i32, y, format_text_left(&value, width), buf);
    }

    fn render_number(&self, x: i32, y: i32, width: usize, buf: &mut Buffer, range: &RangeInclusive<i64>, value: &i64) {
        color_line(x, x + width as i32, y, Color::White, Color::Black, buf);
        write_line(x, x + width as i32, y, format_text_left(&(value.to_string()), width), buf);
    }

    fn render_select(&self, x: i32, y: i32, width: usize, buf: &mut Buffer, value: &String) {
        color_line(x, x + width as i32, y, Color::White, Color::Black, buf);
        let mut switch_string = "☰ ".to_string();
        switch_string.push_str(value);
        
        write_line(x, x + width as i32, y, format_text_center(&switch_string, width), buf); 

    }

    fn render_switch(&self, x: i32, y: i32, width: usize, buf: &mut Buffer, on: &String, off: &String, value: &bool) {
         
        let mut switch_string = "☼ ".to_string();
        if value.clone() {
            switch_string.push_str(on);
            color_line(x, x + width as i32, y, Color::Gray, Color::Black, buf);
        } else {
            switch_string.push_str(off);
            color_line(x, x + width as i32, y, Color::White, Color::Black, buf);
        };

        write_line(x, x + width as i32, y, format_text_center(&switch_string, width), buf); 

    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SocketTuiType {
    Repetitive(Vec<DataTui>),
    Single(DataTui),
}

impl SocketTuiType {
    pub fn get_size(&self) -> usize {
        match self {
            SocketTuiType::Repetitive(v) => v.iter().fold(0, |a, d| {a.max(d.get_size())} ),
            Self::Single(d) => d.get_size(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Connection {
    pub node: u8,
    pub port_index: u8
} 

impl Connection {
    pub fn from_data(data: Option<DataType>) -> Option<Self> {
        if let Some(DataType::Connection(node, port_index)) = data {
            Some(Connection {node, port_index})
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SocketTui {
    pub name: String,
    pub socket_type: SocketTuiType,
    pub type_index: u8,
}

impl SocketTui {
    pub fn get_size(&self) -> usize {
        self.socket_type.get_size()
    }

    pub fn render(&self, transform: SocketTuiTransform, buf: &mut Buffer) -> i32 {
        match &self.socket_type {
            SocketTuiType::Single(d) => {
                self.render_single(transform, d, buf);
                2
            },
            SocketTuiType::Repetitive(v) => {
                self.render_repetitive(transform, v, buf);
                v.len() as i32 * 2
            }
        }
    }

    fn render_single(&self, transform: SocketTuiTransform, data: &DataTui, buf: &mut Buffer) {
        if data.is_output() {
           color_line(transform.x+1, transform.x+1 + transform.width as i32 - 2, transform.y, Color::Black, Color::White, buf); 
           write_line(transform.x+1, transform.x+1 + transform.width as i32 - 2, transform.y, format_text_right(&self.name, transform.width - 2), buf); 
           write_line(transform.x+1 + transform.width as i32 - 2, transform.x+1 + transform.width as i32 - 1, transform.y, "⬤".to_string(), buf); 
           return;
        }
        color_line(transform.x+1, transform.x+1 + transform.width as i32 - 2, transform.y, Color::Black, Color::White, buf); 
        write_line(transform.x+1, transform.x+1 + transform.width as i32 - 2, transform.y, format_text_left(&self.name, transform.name_width), buf); 
        write_line(transform.x, transform.x+1, transform.y, "⬤".to_string(), buf); 
        
        data.render(transform, self.name.is_empty(), buf);
    }

    fn render_repetitive(&self, transform: SocketTuiTransform, vec: &Vec<DataTui>, buf: &mut Buffer) {

    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SocketTuiTransform {
    pub width: usize,
    pub name_width: usize,
    pub y: i32,
    pub x: i32, 
}

