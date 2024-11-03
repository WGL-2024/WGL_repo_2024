// Server is multype
#[derive(Debug)]
pub struct ServerType{ // 1 or more must be true
	isChatServer: bool,
	isTextServer: bool, // must be true if media is true
	isMediaServer: bool
}

#[derive(Debug)]
pub struct Message{
	message_data: MessageData,
	routing_header: SourceRoutingHeader
}

#[derive(Debug)]
pub struct MessageData { // Only part fragmentized
	source_id: NodeId,
	session_id: u64,
	content: MessageContent
}

#[derive(Debug)]
pub enum MessageContent{
	Request(MessageRequest),
	Response(MessageResponse)
}

#[derive(Debug)]
pub enum MessageRequest{ //C -> S
	Chat(ChatRequest),
	Data(DataRequest),  // text and media
	ServerType,
}

#[derive(Debug)]
pub enum MessageResponse{ // S -> C
	Chat(ChatResponse),
	Data(DataResponse),  // text and media
	ServerType(ServerType)
}

#[derive(Debug)]
pub enum ChatRequest{
	ClientList,
	MessageFor {
		to: NodeId,
		message: Vec<u8>
	}
}

#[derive(Debug)]
pub enum ChatResponse{
	ClientList(Vec<NodeId>),
	MessageFrom {
		from: NodeId,
		message: Vec<u8>
	},
	ErrWrongClientId
}

#[derive(Debug)]
pub enum DataRequest{
	FilesList,
	File(u64),
	Media(u64)
}

#[derive(Debug)]
pub enum DataResponse{
	FilesList(Vec<u64>),
	File(Vec<u8>),
	Media(Vec<u8>),
	ErrIsNotMediaServer,
	ErrRequestedNotFound
}

impl Message{
	fn new(routing_header: SourceRoutingHeader, source_id: NodeId, session_id: u64, content: MessageContent) -> Self{
		Self{
			routing_header,
			message_data: MessageData{
				source_id,
				session_id,
				content
			},
		}
	}
}

fn main(){


	match content{
		Request(DataResponse(data)) =>{
			match data{
				FilesList => {
					let files = getFilesList();
					respondWithContent(
						Response(DataResponse(FilesList(files))));
				}
				// [...]
			}
		}
		_ => {}
	}
}
