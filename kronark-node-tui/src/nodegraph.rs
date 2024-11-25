use ratatui::buffer::Buffer;
use ratatui::style::Color;

use crate::{node_tui::NodeTui, utils::{color_rect, write_line}, Camera};


#[derive(Debug, PartialEq, Clone)]
pub struct InternalGraph {
    pub nodes: Vec<NodeTui>,
    pub input: (i32, i32),
    pub output: (i32, i32),
    pub output_connections: Vec<(u8, u8)>,
}

impl InternalGraph {
    pub fn render(&self, buf: &mut Buffer, camera: &Camera) {
        self.render_connections(buf, camera);

        self.render_input(buf, camera);
        self.render_output(buf, camera);

        for node in self.nodes.iter() {
            node.render(buf, camera);
        }
    }

    fn render_input(&self, buf: &mut Buffer, camera: &Camera) {
        let (screen_x, screen_y) = camera.apply(self.input);
        //println!("{:?}", camera.apply(self.input));
        color_rect(screen_x, screen_x + 9, screen_y, screen_y + 3, Color::DarkGray, Color::White, buf);
        write_line(screen_x, screen_x + 10, screen_y, "  input  ".to_string(), buf);
        write_line(screen_x + 8, screen_x + 9, screen_y + 1, "⬤".to_string(), buf);
    }

    fn render_output(&self, buf: &mut Buffer, camera: &Camera) {
        let (screen_x, screen_y) = camera.apply(self.output);
        //println!("{:?}", camera.apply(self.output));
        color_rect(screen_x, screen_x + 10, screen_y, screen_y + 7, Color::DarkGray, Color::White, buf);
        write_line(screen_x, screen_x + 10, screen_y, "  output  ".to_string(), buf);
        write_line(screen_x, screen_x + 1, screen_y + 1, "⬤".to_string(), buf);
        write_line(screen_x, screen_x + 1, screen_y + 3, "⬤".to_string(), buf);
        write_line(screen_x, screen_x + 1, screen_y + 5, "⬤".to_string(), buf);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeGraph {
    pub internal: InternalGraph,
    pub external: NodeTui
}

