use wg_network::{NodeId, SourceRoutingHeader};

#[derive(Debug, Clone)]
pub struct Message<M: MessageContent> {
    pub message_data: MessageData<M>,
    pub routing_header: SourceRoutingHeader,
}

// Only part fragmentized
#[derive(Debug, Clone)]
pub struct MessageData<M: MessageContent> {
    pub source_id: NodeId,
    pub session_id: u64,
    pub content: M,
}

pub trait MessageContent {
    fn serialize(&self) -> String;
    fn deserialize(serialized: String) -> Result<Self, String>
    where
        Self: Sized;
}

// ReqServerType,
#[derive(Debug, Clone)]
pub enum TextRequest {
    FileList,
    File(u64),
}

impl MessageContent for TextRequest {
    fn serialize(&self) -> String {
        match self {
            TextRequest::FileList => "FileList".to_string(),
            TextRequest::File(id) => format!("File({})", id),
        }
    }

    fn deserialize(serialized: String) -> Result<Self, String> {
        if serialized == "FileList" {
            Ok(TextRequest::FileList)
        } else if let Ok(id) = serialized
            .trim_start_matches("File(")
            .trim_end_matches(')')
            .parse()
        {
            Ok(TextRequest::File(id))
        } else {
            Err("Failed to deserialize TextRequest".to_string())
        }
    }
}

#[derive(Debug, Clone)]
pub enum MediaRequest {
    Media(u64),
}

impl MessageContent for MediaRequest {
    fn serialize(&self) -> String {
        todo!()
    }
    fn deserialize(_: String) -> Result<Self, String> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum ChatRequest {
    ClientList,
    Register(NodeId),
    SendMessage { to: NodeId, message: String },
}

impl MessageContent for ChatRequest {
    fn serialize(&self) -> String {
        todo!()
    }
    fn deserialize(_: String) -> Result<Self, String> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum FileResponse {
    FileList(Vec<u64>),
    File(String),
    NotFound,
}

impl MessageContent for FileResponse {
    fn serialize(&self) -> String {
        todo!()
    }
    fn deserialize(_: String) -> Result<Self, String> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum MediaResponse {
    Media(Vec<u8>), // should we use some other type?
}

impl MessageContent for MediaResponse {
    fn serialize(&self) -> String {
        todo!()
    }
    fn deserialize(_: String) -> Result<Self, String> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum ChatResponse {
    RespClientList(Vec<NodeId>),
    RespMessageFrom { from: NodeId, message: Vec<u8> },
}

impl MessageContent for ChatResponse {
    fn serialize(&self) -> String {
        todo!()
    }
    fn deserialize(_: String) -> Result<Self, String> {
        todo!()
    }
}
