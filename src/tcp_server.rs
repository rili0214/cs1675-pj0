use std::{
    net::{SocketAddrV4, TcpStream},
};

// TODO: Students will have to fill in tcp_server, handle_conn and use the code
// they implemented in recv_work_msg and do_work

pub fn tcp_server(addr: SocketAddrV4) {
    // TODO: Students will have to write this code.
    // There's nothing special here except the server should listen for new
    // client connections and then spin off a new thread to handle that
    // connection.
    unimplemented!()
}

fn handle_conn(stream: TcpStream) {
    // TODO: Students will have to write this code.
    // NOTE: It might be helpful to look at protocol.rs first. You'll probably
    // be implementing that alongside this function.
    //
    // This function handles one client connection. It receives a
    // ClientWorkPacket, does work using ClientWorkPacket::do_work which returns
    // a ServerWorkPacket, then sends this ServerWorkPacket back to the client.
    unimplemented!()
}
