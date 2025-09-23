use std::{
    net::{SocketAddrV4, TcpStream, TcpListener},
    thread,
};

use crate::{
    chunked_tcp_stream::ChunkedTcpStream,
    protocol::work_request::ClientWorkPacketConn,
    protocol::work_response::ServerWorkPacketConn,
};

// TODO: Students will have to fill in tcp_server, handle_conn and use the code
// they implemented in recv_work_msg and do_work

pub fn tcp_server(addr: SocketAddrV4) {
    let listener = TcpListener::bind(addr).expect("failed to bind TCP listener");
    println!("server listening on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { thread::spawn(move || handle_conn(stream)); }
            Err(e) => { eprintln!("Incoming Error: {}", e); }
        }
    }
}

fn handle_conn(stream: TcpStream) {
    let stream_clone = stream.try_clone().unwrap();
    let mut client_conn = ClientWorkPacketConn::new(ChunkedTcpStream::new(stream_clone));
    let mut server_conn = ServerWorkPacketConn::new(ChunkedTcpStream::new(stream));

    loop {
        let msg = match client_conn.recv_work_msg() {
            Ok(msg) => msg,
            Err(e) => {
                println!("recv_work_msg Error: {}", e);
                break;
            }
        };

        let reply = msg.do_work();

        match server_conn.send_work_msg(reply) {
            Ok(()) => {
                continue;
            }
            Err(e) => {
                eprintln!("send_work_msg error: {}", e);
                break;
            }
        }
    }
}
