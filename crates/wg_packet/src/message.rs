use wg_network::{NodeId, SourceRoutingHeader};

#[derive(Debug, Clone)]
pub struct Message<M: Serializable> {
    pub message_data: MessageData<M>,
    pub routing_header: SourceRoutingHeader,
}

// Only part fragmentized
#[derive(Debug, Clone)]
pub struct MessageData<M: Serializable> {
    pub source_id: NodeId,
    pub session_id: u64,
    pub content: M,
}

pub trait Serializable {
    fn serialize(&self) -> String;
    fn deserialize(serialized: String) -> Result<Self, String>
    where
        Self: Sized;
}

pub trait Request: Serializable {}
pub trait Response: Serializable {}

// ReqServerType,
#[derive(Debug, Clone)]
pub enum TextRequest {
    TextList,
    Text(u64),
}

impl Serializable for TextRequest {
    fn serialize(&self) -> String {
        match self {
            TextRequest::TextList => "TextList".to_string(),
            TextRequest::Text(id) => format!("Text({})", id),
        }
    }

    fn deserialize(serialized: String) -> Result<Self, String> {
        if serialized == "TextList" {
            Ok(TextRequest::TextList)
        } else if let Ok(id) = serialized
            .trim_start_matches("Text(")
            .trim_end_matches(')')
            .parse()
        {
            Ok(TextRequest::Text(id))
        } else {
            Err("Failed to deserialize TextRequest".to_string())
        }
    }
}

impl Request for TextRequest {}

#[derive(Debug, Clone)]
pub enum MediaRequest {
    MediaList,
    Media(u64),
}

impl Serializable for MediaRequest {
    fn serialize(&self) -> String {
        todo!()
    }
    fn deserialize(_: String) -> Result<Self, String> {
        todo!()
    }
}

impl Request for MediaRequest {}

#[derive(Debug, Clone)]
pub enum ChatRequest {
    ClientList,
    Register(NodeId),
    SendMessage { from: NodeId, to: NodeId, message: String },
}

impl Serializable for ChatRequest {
    fn serialize(&self) -> String {
        todo!()
    }
    fn deserialize(_: String) -> Result<Self, String> {
        todo!()
    }
}

impl Request for ChatRequest {}

#[derive(Debug, Clone)]
pub enum TextResponse {
    TextList(Vec<u64>),
    Text(String),
    NotFound,
}

impl Serializable for TextResponse {
    fn serialize(&self) -> String {
        todo!()
    }
    fn deserialize(_: String) -> Result<Self, String> {
        todo!()
    }
}

impl Response for TextResponse {}

#[derive(Debug, Clone)]
pub enum MediaResponse {
    MediaList(Vec<u64>),
    Media(Vec<u8>), // should we use some other type?
}

impl Serializable for MediaResponse {
    fn serialize(&self) -> String {
        todo!()
    }
    fn deserialize(_: String) -> Result<Self, String> {
        todo!()
    }
}

impl Response for MediaResponse {}

#[derive(Debug, Clone)]
pub enum ChatResponse {
    ClientList(Vec<NodeId>),
    MessageFrom { from: NodeId, message: Vec<u8> },
    MessageSent,
}

impl Serializable for ChatResponse {
    fn serialize(&self) -> String {
        todo!()
    }
    fn deserialize(_: String) -> Result<Self, String> {
        todo!()
    }
}

impl Response for ChatResponse {}
