extern crate hyper;

use std::io::net::ip::Ipv4Addr;
use hyper::server::{Request, Response};
use hyper::Server;

fn hello(_: Request, resp: Response) {
	//*resp.status_mut() = Response::status::Ok;
	let mut resp = resp.start().unwrap();
	resp.write(b"Hello, World!");
	resp.end().unwrap();
}

fn main() {
    let server = Server::http(Ipv4Addr(127, 0, 0, 1), 8080);
    server.listen(hello).unwrap();
}
