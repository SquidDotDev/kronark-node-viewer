use kronark_node_parser::kronarknode::instance::Instance;
use kronark_node_parser::kronarknode::socket::SocketType;
use crate::socket_tui::{Additional, RepetiveSocket, SocketDefault, SocketTui};
use crate::utils::validate_socket;
use crate::{errors::NodeConversionError, node_tui::NodeTui};

pub fn parse_bytes(instance: Instance) -> Result<NodeTui, NodeConversionError> {
    let mut sockets = Vec::new();
    let mut padding: usize = 0;

    // prefix port

    let prefix = validate_socket(&instance, sockets.len() - padding, SocketType::IncomingNamed)?;

    sockets.push(SocketTui{ 
        name: "prefix".to_string(),
        socket: prefix.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
        connective: true,
    });

    while let Ok(elem) = validate_socket(&instance, sockets.len() - padding, SocketType::IncomingText) {
        sockets.push(SocketTui{ 
            name: "".to_string(),
            socket: elem.clone(),
            default: SocketDefault::String(String::new()),
            additional: Additional::Text { minimum: "0".to_string(), maximum: "".to_string(), valid: "01".to_string() },
            connective: true,
        });
    }

    sockets.push(RepetiveSocket());
    padding += 1;

    // result port
    
    let result = validate_socket(&instance, sockets.len() - padding, SocketType::OutgoingNamed)?;
    

    sockets.push(SocketTui{ 
        name: "result".to_string(),
        socket: result.clone(),
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