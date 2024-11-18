use errors::NodeConversionError;
use kronark_node_parser::prelude::Node;
use node_tui::NodeTui;
use nodegraph::{InternalGraph, NodeGraph};
use socket_tui::Connection;

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

impl App {
    pub fn from_node(node: Node) -> Result<Self, NodeConversionError> {
        let node_def = if let Node::V1(desc) = node {
            desc
        } else {
            return Err(NodeConversionError::NodeVersionNotSupported);
        };

        let input = (node_def.roots.input_root_x as i32, node_def.roots.input_root_y as i32);
        let output = (node_def.roots.output_root_x as i32, node_def.roots.output_root_y as i32);
        let output_connections: Vec<Connection> = node_def.roots.output_connections
        .iter()
        .map(|(node, port)| Connection { node: node.clone(), port_index: port.clone() })
        .collect();

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

        let camera = Camera { x: input.0, y: input.1 }; 

        Ok(App { camera, node_graph: graph, graph_view: GraphView::Internal })
    }
}