use crate::{node_tui::NodeTui, socket_tui::Connection};

#[derive(Debug, PartialEq, Clone)]
pub struct InternalGraph {
    pub nodes: Vec<NodeTui>,
    pub input: (i32, i32),
    pub output: (i32, i32),
    pub output_connections: Vec<Connection>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeGraph {
    pub internal: InternalGraph,
    pub external: NodeTui
}

