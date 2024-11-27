use std::ops::Div;

use kronark_node_parser::{kronarknode::socket::DataType, prelude::Node};
use ratatui::buffer::Buffer;

use crate::{nodegraph::InternalGraph, utils::{write_cell, write_line}, Camera};



impl InternalGraph {
    pub fn render_connections(&self, buf: &mut Buffer, camera: &Camera) {
        for node_tui in self.nodes.iter() {
            for (i, socket_tui) in node_tui.sockets.iter().enumerate() {
                if let Some(DataType::Connection(node, socket)) = socket_tui.socket.data {
                    if node == 255 {
                        let (x_input, y_input) = camera.apply(self.input);
                        let (x_start, y_start) = (x_input + 9, y_input + 1);
                        let (x_node, y_node) = camera.apply((node_tui.x, node_tui.y));
                        let (x_end, y_end) = (x_node, y_node + i as i32 * 2 + 4);
                        render_connection(x_start, x_end, y_start, y_end, buf, camera);
                    } else {
                        if let Some(socket_pos) = self.get_socket_pos(camera, node.clone(), socket.clone()) {
                            let (x_node, y_node) = camera.apply((node_tui.x, node_tui.y));
                            let (x_end, y_end) = (x_node, y_node + i as i32 * 2 + 4);
                            render_connection(socket_pos.0, x_end, socket_pos.1, y_end, buf, camera);
                        }
                    }
                }
            }
        }

        for (i, (node, socket)) in self.output_connections.iter().enumerate() {
            if let Some(socket_pos) = self.get_socket_pos(camera, node.clone(), socket.clone()) {
                let (x_output, y_output) = camera.apply(self.output);
                let (x_end, y_end) = (x_output, y_output + i as i32 * 2 + 1);
                render_connection(socket_pos.0, x_end, socket_pos.1, y_end, buf, camera);
            }
        }
    }

    fn get_socket_pos(&self, camera: &Camera, node: u8, socket: u8) -> Option<(i32, i32)> {
        for node_tui in self.nodes.iter().filter(|n| n.key == node) {
            let mut padding = 0;
            for (i, socket_tui) in node_tui.sockets.iter().enumerate() {
                if socket_tui.is_repetitive_end() {
                    padding += 1;
                    continue;
                }
                if (i-padding) as u8 == socket {
                    let (x_node, y_node) = camera.apply((node_tui.x, node_tui.y));
                    return Some((x_node + node_tui.get_x_size() as i32, y_node + (i-padding) as i32 * 2 + 4));
                }
            }
        }
        None
    }
}



fn render_connection(x_start: i32, x_end: i32, y_start: i32, y_end: i32, buf: &mut Buffer, camera: &Camera) {
    
    for x in x_start..x_start + (x_end - x_start).div(2) {
        write_cell(x, y_start, '-', buf);
    }
    let (y_min, y_max) = (y_start.min(y_end), y_start.max(y_end));
    for y in y_min..y_max {
        write_cell(x_start + (x_end - x_start).div(2), y, '|', buf);
    }
    for x in x_start + (x_end - x_start).div(2)..x_end {
        write_cell(x, y_end, '-', buf);
    }
}