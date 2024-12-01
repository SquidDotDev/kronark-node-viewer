use kronark_node_parser::kronarknode::instance::Instance;
use kronark_node_parser::kronarknode::socket::SocketType;
use crate::socket_tui::{Additional, RepetiveSocket, SocketDefault, SocketTui};
use crate::utils::validate_socket;
use crate::{errors::NodeConversionError, node_tui::NodeTui};

pub fn parse_option(instance: Instance) -> Result<NodeTui, NodeConversionError> {
    let mut sockets = Vec::new();

    // when port

    let when = validate_socket(&instance, sockets.len(), SocketType::IncomingNamed)?;

    sockets.push(SocketTui{ 
        name: "when".to_string(),
        socket: when.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
        connective: true,
    });

    // option port

    let option = validate_socket(&instance, sockets.len(), SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "".to_string(),
        socket: option.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: String::new(), maximum: String::new(), valid: "qwertyuiopasdfghjklzxcvbnm_1234567890 ".to_string() },
        connective: false,
    });
    
    // then port
    
    let then = validate_socket(&instance, sockets.len(), SocketType::OutgoingNamed)?;
    

    sockets.push(SocketTui{ 
        name: "then".to_string(),
        socket: then.clone(),
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