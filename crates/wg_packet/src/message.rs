use crate::packet::Fragment;
use wg_network::topology::ServerType;
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
    ErrUnsupporedRequestType,
    ErrRequestedNotFound,

    RespClientList(Vec<NodeId>),
    RespMessageFrom { from: NodeId, message: Vec<u8> },
    ErrWrongClientId,
}

impl Message {
    // takes message and returns the data struct serialized in a String
    // so it goes from the actual data struct to a String
    #[allow(unused_variables)]
    pub fn serialize(&self) -> String {
        unimplemented!()
    }

    // takes the content String and makes an instance of Message from it
    #[allow(unused_variables)]
    pub fn deserialize(serialized: String) -> Message {
        unimplemented!()
    }

    // takes the String and splits it into Fragments
    #[allow(unused_variables)]
    pub fn disassembly(serialized: String) -> Vec<Fragment> {
        unimplemented!()
    }

    // takes a bunch of Fragments and composes them in a serialized string.
    #[allow(unused_variables)]
    pub fn assembly(fragments: Vec<Fragment>) -> String {
        unimplemented!()
    }
}
