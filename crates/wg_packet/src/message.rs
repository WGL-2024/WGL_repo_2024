use wg_network::{NodeId, SourceRoutingHeader};

#[derive(Debug, Clone)]
pub struct Message {
    pub message_data: MessageData,
    pub routing_header: SourceRoutingHeader,
}

// Only part fragmentized
#[derive(Debug, Clone)]
pub struct MessageData {
    pub source_id: NodeId,
    pub session_id: u64,
    pub content: MessageContent,
}

#[derive(Debug, Clone)]
pub enum MessageContent {
    // Client -> Server
    ReqServerType,
    ReqFilesList,
    ReqFile(u64),
    ReqMedia(u64),

    ReqClientList,
    ReqRegistrationToChat,
    ReqMessageSend { to: NodeId, message: Vec<u8> },

    // Server -> Client
    RespServerType(ServerType),
    RespFilesList(Vec<u64>),
    RespFile(Vec<u8>),
    RespMedia(Vec<u8>),
    ErrUnsupportedRequestType,
    ErrRequestedNotFound,

    RespClientList(Vec<NodeId>),
    RespMessageFrom { from: NodeId, message: Vec<u8> },
    ErrWrongClientId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerType {
    Chat,
    Text,
    Media,
}
