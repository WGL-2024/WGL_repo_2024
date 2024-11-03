use crate::types::NodeId;
use crate::types::SourceRoutingHeader;

// Server is multype (1 or more must be true)
#[derive(Debug)]
pub struct ServerType {
    is_chat_server: bool,
    // Text support must be true if media support is true
    is_text_server: bool,
    is_media_server: bool,
}

#[derive(Debug)]
pub struct Message {
    message_data: MessageData,
    routing_header: SourceRoutingHeader,
}

// Only part fragmentized
#[derive(Debug)]
pub struct MessageData {
    source_id: NodeId,
    session_id: u64,
    content: MessageContent,
}

#[derive(Debug)]
pub enum MessageContent {
    // Client -> Server
    ReqServerType,
    ReqFilesList,
    ReqFile(u64),
    ReqMedia(u64),

    ReqClientList,
    ReqMessageSend { to: NodeId, message: Vec<u8> },

    // Server -> Client
    RespServerType(ServerType),
    RespFilesList(Vec<u64>),
    RespFile(Vec<u8>),
    RespMedia(Vec<u8>),
    ErrUnsupporedRequestType,
    ErrRequestedNotFound,

    RespClientList(Vec<NodeId>),
    RespMessageFrom { from: NodeId, message: Vec<u8> },
    ErrWrongClientId,
}

impl Message {
    pub fn new(
        routing_header: SourceRoutingHeader,
        source_id: NodeId,
        session_id: u64,
        content: MessageContent,
    ) -> Self {
        Self {
            routing_header,
            message_data: MessageData {
                source_id,
                session_id,
                content,
            },
        }
    }
}
