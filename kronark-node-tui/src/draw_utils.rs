use ratatui::buffer::Buffer;

pub fn format_text_left(mut string: String, max_len: usize) -> String {
    if string.len() > max_len {
        string = string[0..max_len-1].to_owned()
    }
    string.insert(0, ' ');
    string
}

pub fn format_text_right(mut string: String, max_len: usize) -> String {
    if string.len() > max_len {
        string = string[0..max_len-1].to_owned()
    }
    for _ in 0..max_len-string.len()-1 {
        string.insert(0, ' ');
    }
    string.push(' ');
    string
}

pub fn format_text_center(mut string: String, max_len: usize) -> String {
    if string.len() > max_len {
        string = string[0..max_len-2].to_owned()
    }
    for _ in 0..((max_len - string.len()) as f32 / 2.0).floor() as u16 {
        string.insert(0, ' ');
    }
    string.push(' ');
    string
}

pub fn color_line(start: i32, end: i32, y: i32, bg: ratatui::style::Color, fg: ratatui::style::Color, buf: &mut Buffer) {
    for x in (start.max(0) as u16)..(end.max(1) as u16) {
        if let Some(mut cell) = buf.cell_mut((x, y.max(0) as u16)) {
            cell.set_bg(bg).set_fg(fg);
        }
    }
}

pub fn write_line(start: i32, end: i32, y: i32, string: String, buf: &mut Buffer) {
    for x in (start.max(0) as u16)..(string.len() as i32 +1 + end).max(1) as u16 {
        if let Some(mut cell) = buf.cell_mut((x, y.max(0) as u16)) {
            cell.set_char(string.chars().nth((x as i32 - start) as usize).unwrap_or(' '));
        }
    }
}

pub fn color_rect(start_x: i32, end_x: i32, start_y: i32, end_y: i32, bg: ratatui::style::Color, fg: ratatui::style::Color, buf: &mut Buffer) {
    for y in (start_y.max(0) as u16)..(end_y.max(1) as u16) {
        for x in (start_x.max(0) as u16)..(end_x.max(1) as u16) {
            if let Some(mut cell) = buf.cell_mut((x, y)) {
                cell.set_bg(bg).set_fg(fg);
            }
        }
    } 
}