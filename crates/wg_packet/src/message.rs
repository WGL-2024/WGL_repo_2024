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

mod example {
    use wg_network::{NodeId, SourceRoutingHeader};
    use wg_network::topology::ServerType;
    use crate::{ChatRequest, ChatResponse, Message, Serializable, MessageData, Request, Response};

    trait Server {
        type RequestType: Request;
        type ResponseType: Response;

        fn compose_message(routing_header: SourceRoutingHeader, source_id: NodeId, session_id: u64, raw_content: String) -> Result<Message<Self::RequestType>, String> {
            let content = Self::RequestType::deserialize(raw_content)?;
            Ok(Message {
                routing_header,
                message_data: MessageData {
                    session_id,
                    source_id,
                    content,
                },
            })
        }

        fn on_request_arrived(&mut self, routing_header: SourceRoutingHeader, source_id: NodeId, session_id: u64, raw_content: String) {
            if raw_content == "ServerType" {
                let _server_type = Self::get_sever_type();
                // send response
                return;
            }
            match Self::compose_message(routing_header, source_id, session_id, raw_content) {
                Ok(message) => {
                    let response = self.handle_request(message.message_data.content);
                    self.send_response(response);
                }
                Err(str) => panic!("{}", str)
            }
        }

        fn send_response(&mut self, _response: Self::ResponseType) {
            // send response
        }

        fn handle_request(&mut self, request: Self::RequestType) -> Self::ResponseType;

        fn get_sever_type() -> ServerType;
    }

    struct ChatServer;

    impl Server for ChatServer {
        type RequestType = ChatRequest;
        type ResponseType = ChatResponse;

        fn handle_request(&mut self, request: Self::RequestType) -> Self::ResponseType {
            match request {
                ChatRequest::ClientList => {
                    println!("Sending ClientList");
                    ChatResponse::ClientList(vec![1, 2])
                }
                ChatRequest::Register(id) => {
                    println!("Registering {}", id);
                    ChatResponse::ClientList(vec![1, 2])
                }
                ChatRequest::SendMessage { message, to, from: _ } => {
                    println!("Sending message \"{}\" to {}", message, to);
                    // effectively forward message
                    ChatResponse::MessageSent
                }
            }
        }

        fn get_sever_type() -> ServerType {
            ServerType::Chat
        }
    }
}
