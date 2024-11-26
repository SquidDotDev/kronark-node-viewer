use std::collections::HashSet;

use kronark_node_parser::kronarknode::instance::Instance;
use kronark_node_parser::kronarknode::socket::SocketType;
use crate::socket_tui::{Additional, SocketDefault, SocketTui};
use crate::utils::{data_get_constant, validate_socket};
use crate::{errors::NodeConversionError, node_tui::NodeTui};


pub fn parse_path(instance: Instance) -> Result<NodeTui, NodeConversionError> {
    let mut sockets = Vec::new();

    // path port

    let path = validate_socket(&instance, sockets.len(), SocketType::OutgoingNamed)?;

    sockets.push(SocketTui{ 
        name: "path".to_string(),
        socket: path.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
    });

    // type port

    let type_socket = validate_socket(&instance, sockets.len(), SocketType::IncomingSelect)?;

    let mut options = HashSet::new();
    options.insert("module".to_string());
    options.insert("absolute".to_string());

    sockets.push(SocketTui{ 
        name: "type".to_string(),
        socket: type_socket.clone(),
        default: SocketDefault::String("absolute".to_string()),
        additional: Additional::Select { options }
    });

    while let Ok(name_socket) = validate_socket(&instance, sockets.len(), SocketType::IncomingText) {
        sockets.push(SocketTui{ 
            name: "name".to_string(),
            socket: name_socket.clone(),
            default: SocketDefault::String(String::new()),
            additional: Additional::Text { minimum: "0".to_string(), maximum: "".to_string(), valid: "qwertyuiopasdfghjklzxcvbnm_1234567890 ".to_string() }
        });
    }


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