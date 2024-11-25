use std::fmt::Write;

use kronark_node_parser::kronarknode::{instance::Instance, socket::{DataType, Socket, SocketType}};
use ratatui::buffer::Buffer;

use crate::{errors::NodeConversionError, node_tui::NodeTui};

pub fn validate_socket(instance: &Instance, index: usize, socket_type: SocketType) -> Result<&Socket, NodeConversionError> {
    if let Some(s) = instance.sockets.get(index) {
        
        if s.flags.get_type() != socket_type {
            return Err(NodeConversionError::InvalidSocketType(format!("index: {}, type: {:?}, expected:{:?}", index, s.flags.get_type(), socket_type)));
        }
        Ok(s)
    } else {
        return Err(NodeConversionError::InvalidSocketCount(index));
    }
}

pub fn data_get_constant(data: &Option<DataType>) -> Option<String> {
    if let Some(DataType::Constant(value)) = data {
        Some(value.clone())
    } else {
        None
    }
}

pub fn format_text_left(mut string: &String, max_len: usize) -> String {
    let mut result = string.clone();
    if result.len() > max_len {
        result = result[0..max_len-1].to_owned()
    }
    result.insert(0, ' ');
    result
}

pub fn format_text_right(mut string: &String, max_len: usize) -> String {
    let mut result = string.clone();
    if result.len() > max_len {
        result = result[0..max_len-1].to_owned()
    }
    for _ in 0..max_len-result.len()-1 {
        result.insert(0, ' ');
    }
    result.push(' ');
    result
}

pub fn format_text_center(string: &String, max_len: usize) -> String {
    let mut result = string.clone();
    if result.len() > max_len {
        result = result[0..max_len-2].to_owned()
    }
    for _ in 0..((max_len - result.len()) as f32 / 2.0).floor() as u16 {
        result.insert(0, ' ');
    }
    result
}

pub fn color_line(start: i32, end: i32, y: i32, bg: ratatui::style::Color, fg: ratatui::style::Color, buf: &mut Buffer) {
    if y < 0 {
        return;
    }
    for x in (start.max(0) as u16)..(end.max(0) as u16) {
        if let Some(mut cell) = buf.cell_mut((x, y.max(0) as u16)) {
            cell.set_bg(bg).set_fg(fg);
        }
    }
}

pub fn write_line(start: i32, end: i32, y: i32, string: String, buf: &mut Buffer) {
    if y < 0 {
        return;
    }
    for x in (start.max(0) as u16)..(end.max(0)  as u16){
        if let Some(mut cell) = buf.cell_mut((x, y.max(0) as u16)) {
            cell.set_char(string.chars().nth((x as i32 - start) as usize).unwrap_or(' '));
        }
    }
}

pub fn write_cell(x: i32, y: i32, char: char , buf: &mut Buffer) {
    if y < 0 || x < 0 {
        return;
    }
    if let Some(mut cell) = buf.cell_mut((x.max(0) as u16, y.max(0) as u16)) {
        cell.set_char(char);
    }
}

pub fn color_rect(start_x: i32, end_x: i32, start_y: i32, end_y: i32, bg: ratatui::style::Color, fg: ratatui::style::Color, buf: &mut Buffer) {
    for y in (start_y.max(0) as u16)..(end_y.max(0) as u16) {
        for x in (start_x.max(0) as u16)..(end_x.max(0) as u16) {
            if let Some(mut cell) = buf.cell_mut((x, y)) {
                cell.set_bg(bg).set_fg(fg);
            }
        }
    } 
}