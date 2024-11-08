use crate::types::packet::Fragment;
use crate::types::source_routing_header::{NodeId, SourceRoutingHeader};

// Server is multype (1 or more must be true)
#[derive(Debug)]
pub enum ServerType {
    Chat,
    Text,
    Media,
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

    //takes message and returns the data struct serialized in a String
    //so it goes from the actual data struct to a String
    #[allow(unused_variables)]
    fn serialize(&self) -> String {
        unimplemented!()
    }

    //Takes the content String and makes an instance of Message from it
    #[allow(unused_variables)]
    fn deserialize(serialized: String) -> Message {
        unimplemented!()
    }

    //takes the String and splits it into Fragments
    #[allow(unused_variables)]
    fn disassembly(serialized: String) -> Vec<Fragment> {
        unimplemented!()
    }

    //takes a bunch of Fragments and composes them in a serialized string.
    #[allow(unused_variables)]
    fn assembly(fragments: Vec<Fragment>) -> String {
        unimplemented!()
    }
}
