mod message;

use crate::message::{ChatRequest, ChatResponse, DroneSend, Message, Request, Response};
use wg_2024::network::topology::ServerType;
use wg_2024::network::*;

pub trait Server {
    type RequestType: Request;
    type ResponseType: Response;

    fn compose_message(
        source_id: NodeId,
        session_id: u64,
        raw_content: String,
    ) -> Result<Message<Self::RequestType>, String> {
        let content = Self::RequestType::from_string(raw_content)?;
        Ok(Message {
            session_id,
            source_id,
            content,
        })
    }

    fn on_request_arrived(&mut self, source_id: NodeId, session_id: u64, raw_content: String) {
        if raw_content == "ServerType" {
            let _server_type = Self::get_sever_type();
            // send response
            return;
        }
        match Self::compose_message(source_id, session_id, raw_content) {
            Ok(message) => {
                let response = self.handle_request(message.content);
                self.send_response(response);
            }
            Err(str) => panic!("{}", str),
        }
    }

    fn send_response(&mut self, _response: Self::ResponseType) {
        // send response
    }

    fn handle_request(&mut self, request: Self::RequestType) -> Self::ResponseType;

    fn get_sever_type() -> ServerType;
}

pub(crate) struct ChatServer;

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
            ChatRequest::SendMessage {
                message,
                to,
                from: _,
            } => {
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

fn main() {
    let mut server = ChatServer;
    server.on_request_arrived(1, 1, ChatRequest::Register(1).stringify());
    server.on_request_arrived(
        1,
        1,
        ChatRequest::SendMessage {
            from: 1,
            to: 2,
            message: "Hello".to_string(),
        }
        .stringify(),
    );
    server.on_request_arrived(1, 1, "ServerType".to_string());
}
