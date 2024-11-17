use kronark_node_parser::prelude::Node;
use nodegraph::NodeGraph;

mod draw_utils;
mod socket_tui;
mod node_tui;
mod nodegraph;
mod built_in;

#[derive(Debug, PartialEq, Clone)]
pub struct Camera {
    pub x: i32,
    pub y: i32,
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
}