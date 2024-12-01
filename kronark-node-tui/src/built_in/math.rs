use std::collections::HashSet;

use kronark_node_parser::kronarknode::instance::Instance;
use kronark_node_parser::kronarknode::socket::SocketType;
use crate::socket_tui::{Additional, RepetiveSocket, SocketDefault, SocketTui};
use crate::utils::{data_get_constant, validate_socket};
use crate::{errors::NodeConversionError, node_tui::NodeTui};

pub fn parse_math(instance: Instance) -> Result<NodeTui, NodeConversionError> {
    let mut sockets = Vec::new();
    let mut padding: usize = 0;

    // operation port

    let operation = validate_socket(&instance, sockets.len(), SocketType::IncomingSelect)?;

    let mut options = HashSet::new();
    options.insert("multiply".to_string());
    options.insert("divide".to_string());
    options.insert("substract".to_string());

    sockets.push(SocketTui{ 
        name: "operation".to_string(),
        socket: operation.clone(),
        default: SocketDefault::String("compiler/any".to_string()),
        additional: Additional::Select { options },
        connective: false,
    });

    match data_get_constant(&operation.data).unwrap_or(String::new()).as_str() {
        "divide" => parse_divide(instance.clone(), &mut sockets)?,
        "multiply" => parse_multiply(instance.clone(), &mut sockets)?,
        _ => (),
    };

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


fn parse_divide(instance: Instance, sockets: &mut Vec<SocketTui>) -> Result<(), NodeConversionError> {
    
    // dividend port

    let dividend = validate_socket(&instance, sockets.len(), SocketType::IncomingNumber)?;

    sockets.push(SocketTui{ 
        name: "dividend".to_string(),
        socket: dividend.clone(),
        default: SocketDefault::String("0".to_string()),
        additional: Additional::Number { minimum: String::new(), maximum: String::new(), step: String::new() },
        connective: true,
    });

    // divisor port

    let divisor = validate_socket(&instance, sockets.len(), SocketType::IncomingNumber)?;

    sockets.push(SocketTui{ 
        name: "divisor".to_string(),
        socket: divisor.clone(),
        default: SocketDefault::String("1".to_string()),
        additional: Additional::Number { minimum: String::new(), maximum: String::new(), step: String::new() },
        connective: true,
    });

    // fraction port

    let fraction = validate_socket(&instance, sockets.len(), SocketType::OutgoingNamed)?;

    sockets.push(SocketTui{ 
        name: "fraction".to_string(),
        socket: fraction.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
        connective: true,
    });
    
    Ok(())
}

fn parse_multiply(instance: Instance, sockets: &mut Vec<SocketTui>) -> Result<(), NodeConversionError> {
    
    // factor port

    while let Ok(factor) = validate_socket(&instance, sockets.len(), SocketType::IncomingNumber) {
        if !(factor.flags.is_repetitive()) {
            break;
        }
        sockets.push(SocketTui{ 
            name: "factor".to_string(),
            socket: factor.clone(),
            default: SocketDefault::String("1".to_string()),
            additional: Additional::Number { minimum: String::new(), maximum: String::new(), step: String::new() },
            connective: true,
        });    
    }

    sockets.push(RepetiveSocket());

    // product port

    let product = validate_socket(&instance, sockets.len()-1, SocketType::OutgoingNamed)?;

    sockets.push(SocketTui{ 
        name: "product".to_string(),
        socket: product.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
        connective: true,
    });
    
    Ok(())
}