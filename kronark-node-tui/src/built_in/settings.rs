use kronark_node_parser::kronarknode::instance::Instance;
use kronark_node_parser::kronarknode::socket::SocketType;
use crate::socket_tui::{Additional, SocketDefault, SocketTui};
use crate::utils::{data_get_constant, validate_socket};
use crate::{errors::NodeConversionError, node_tui::NodeTui};

pub fn parse_settings(instance: Instance) -> Result<NodeTui, NodeConversionError> {
    let mut sockets = Vec::new();

    // output port

    let output = validate_socket(&instance, 0, SocketType::OutgoingNamed)?;

    sockets.push(SocketTui{ 
        name: "output".to_string(),
        socket: output.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
    });

    // module port

    let module = validate_socket(&instance, 1, SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "module".to_string(),
        socket: module.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
    });

    // name port 

    let name = validate_socket(&instance, 2, SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "name".to_string(),
        socket: name.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "64".to_string(), valid: "qwertyuiopasdfghjklzxcvbnm_1234567890 ".to_string() }
    });

    // color port 

    let color = validate_socket(&instance, 3, SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "color".to_string(),
        socket: color.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "64".to_string(), valid: "abcdef1234567890".to_string() }
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