use kronark_node_parser::kronarknode::instance::Instance;
use kronark_node_parser::kronarknode::socket::SocketType;
use crate::socket_tui::{Additional, SocketDefault, SocketTui};
use crate::utils::validate_socket;
use crate::{errors::NodeConversionError, node_tui::NodeTui};

pub fn parse_size(instance: Instance) -> Result<NodeTui, NodeConversionError> {
    let mut sockets = Vec::new();

    // input port

    let input = validate_socket(&instance, sockets.len(), SocketType::IncomingNamed)?;

    sockets.push(SocketTui{ 
        name: "input".to_string(),
        socket: input.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
        connective: true,
    });
    
    // size port
    
    let size = validate_socket(&instance, sockets.len(), SocketType::OutgoingNamed)?;
    

    sockets.push(SocketTui{ 
        name: "size".to_string(),
        socket: size.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
        connective: true,
    });

    let node = NodeTui {
        name: instance.name,
        x: instance.position_x as i32,
        y: instance.position_y as i32,
        color: ratatui::style::Color::DarkGray,
        type_index: instance.node_type as u8,
        sockets: sockets,
        key: instance.key as u8,
    };
    Ok(node)
}