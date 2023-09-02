/* 
Learn a bit about TCP and HTTP.
Listen for TCP connections on a socket.
Parse a small number of HTTP requests.
Create a proper HTTP response.
Improve the throughput of our server with a thread pool.
make rust backend for encrypted lightnovel abode app, epic gaming basically
*/
use tiny_http::{Server, Response};
fn main() {


    let server = Server::http("127.0.0.1:7878").unwrap();

    for mut request in server.incoming_requests() {
        let mut content = String::new();
        request.as_reader().read_to_string(&mut content).unwrap();

        println!("received request! method: {:?}, url: {:?}, headers: {:?}, body: {:?}",
            request.method(),
            request.url(),
            request.headers(),
            content
        );

        let response = Response::from_string("gaming gamer madoka");
        request.respond(response).unwrap(); 
    }
}