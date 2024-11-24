use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use errors::NodeConversionError;
use kronark_node_parser::prelude::Node;
use node_tui::NodeTui;
use nodegraph::{InternalGraph, NodeGraph};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget, DefaultTerminal, Frame};

mod utils;
mod socket_tui;
mod node_tui;
mod nodegraph;
mod built_in;
mod errors;

#[derive(Debug, PartialEq, Clone)]
pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub scale: f32,
}

impl Camera {
    pub fn apply(&self, position: (i32, i32)) -> (i32, i32) {
        ((position.0 as f32 * self.scale) as i32 - self.x, (-position.1 as f32 * self.scale) as i32 - self.y)
    }

    pub fn with_scale(position: (i32, i32), scale: f32) -> Self {
        Camera { x: (position.0 as f32 * scale) as i32, y: (-position.1 as f32 * scale) as i32, scale }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GraphView {
    Internal,
    External,
}

#[derive(Debug, PartialEq, Clone)]
pub struct App {
    camera: Camera,
    node_graph: NodeGraph,
    graph_view: GraphView,
    exit: bool,
}

impl App {
    pub fn from_node(node: Node) -> Result<Self, NodeConversionError> {
        let node_def = if let Node::V1(desc) = node {
            desc
        } else {
            return Err(NodeConversionError::NodeVersionNotSupported);
        };

        let input = (node_def.roots.input_root_x as i32, node_def.roots.input_root_y as i32);
        let output = (node_def.roots.output_root_x as i32, node_def.roots.output_root_y as i32);
        let output_connections: Vec<(u8, u8)> = node_def.roots.output_connections;

        let mut nodes = Vec::<NodeTui>::new();
        for instance in node_def.instances.iter() {
            match NodeTui::from_instance(instance.clone()) {
                Ok(n) => nodes.push(n),
                Err(e) => println!("{:?}", e),
            };
        }

        let internal = InternalGraph { nodes, input, output, output_connections };
        let graph = NodeGraph { 
            internal,
            external: NodeTui { name: "".to_string(), x: 0, y: 0, sockets: vec![], color: ratatui::style::Color::DarkGray, type_index: 0 } 
        };

        let camera = Camera::with_scale(input, 2.0); 

        Ok(App { camera, node_graph: graph, graph_view: GraphView::Internal, exit: false })
    }

    pub fn launch(&mut self) -> io::Result<()> {
        let mut terminal = ratatui::init();
        let app_result = self.run(&mut terminal);
        ratatui::restore();
        app_result
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        let speed: i32 = if key_event.modifiers.contains(KeyModifiers::SHIFT) {
            5 
        } else {
            1 
        };

        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Down => self.camera.y += speed,
            KeyCode::Up => self.camera.y -= speed,
            KeyCode::Left => self.camera.x -= speed,
            KeyCode::Right => self.camera.x += speed,
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.node_graph.internal.render(buf, &self.camera);
    }
}