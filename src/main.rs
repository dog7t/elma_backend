/*
Learn a bit about TCP and HTTP.
Listen for TCP connections on a socket.
Parse a small number of HTTP requests.
Create a proper HTTP response.
Improve the throughput of our server with a thread pool.
make rust backend for encrypted lightnovel abode app, epic gaming basically
*/
use json::*;
use std::fs;
use tiny_http::{Response, Server, IncomingRequests, Request};
fn main() {
    let server = Server::http("192.168.1.106:7878").unwrap();
    let message_file = fs::read_to_string("src/messages.json").unwrap();
    let mut messages = json::parse(&message_file).unwrap();
    
    let mut refresh_requests: Vec<Request> = vec![];

    for mut request in server.incoming_requests() {
        let mut content = String::new();
        request.as_reader().read_to_string(&mut content).unwrap();
        
        println!(
            "received request! url: {:?}, body: {:?}",
            request.url(),
            content
        );

        if request.url() == "/sendmessage" {

        }

        // check if new message sent, if new message sent, then send new message list to other users
        // make this code multithreaded (is it multithreaded rn???? not sure)
        let response = match request.url() {
            "/login" => {
                let userid: i32 = 6969;
                let response_json = object! {
                    userId: userid,
                    messages: messages.clone(),
                };
                Response::from_string(response_json.dump())
            }
            "/sendmessage" => { // content = { "userId": 42, "messageText": "Bandana" } 
                // add message id as well
                let json_content = json::parse(&content).unwrap(); // TODO: remove unwrap to check for empty messages
                messages.push(json_content.clone()).unwrap(); // is this clone okay???? whoops probably
                // todo!("send messages to the other users");
                refresh_requests.push(request);

                for req in refresh_requests {
                    req.respond(Response::from_string(json_content.dump())).unwrap();
                }

                refresh_requests = vec![];
                
                // we continue because this request should not be responded to yet (if this was the refresh command which is not yet oopps)
                continue;
            },
            _ => Response::from_string("404 error lul"),
        };
        
        request.respond(response).unwrap();
    }
}
