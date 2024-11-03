use crate::types::NodeId;
use crate::types::SourceRoutingHeader;

// Server is multype
#[derive(Debug)]
pub struct ServerType {
    // 1 or more must be true
    is_chat_server: bool,
    is_text_server: bool, // must be true if media is true
    is_media_server: bool,
}

#[derive(Debug)]
pub struct Message {
    message_data: MessageData,
    routing_header: SourceRoutingHeader,
}

#[derive(Debug)]
pub struct MessageData {
    // Only part fragmentized
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
    // Do we need request of new messages? or directly sent by server?

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
