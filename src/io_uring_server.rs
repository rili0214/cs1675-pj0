use crate::{io_uring::IOUring, serialize::ClientWorkPacket};
use std::{
    error::Error,
    fmt,
    net::{SocketAddrV4, TcpStream},
};

// TODO: Students should implement this function. Probably start by deleting the current body.
#[allow(unused)]
pub fn io_uring_server(addr: SocketAddrV4, ring_sz: usize) {
    let _stream: TcpStream = unimplemented!();
    #[allow(unreachable_code)]
    handle_conn(_stream, ring_sz).unwrap();
}

// TODO: Students should implement this function. Students should not change the function signature
// of this thing. Probably start by deleting the current body.
#[allow(unused)]
fn handle_conn(stream: TcpStream, ring_sz: usize) -> Result<(), anyhow::Error> {
    let _server: IOUringServer = unimplemented!();
    #[allow(unreachable_code)]
    _server.serve()
}

// TODO: Students should use this struct to store objects used in the functions
// above. Students can change the struct's type signature if they want.
#[allow(unused)]
struct IOUringServer {
    ring: IOUring,
    // TODO: Students can add their own members
}

impl IOUringServer {
    // TODO: Students should implement this function
    fn serve(self) -> Result<(), anyhow::Error> {
        unimplemented!();
    }

    // TODO: Students should implement this function
    #[allow(unused)]
    fn handle_recv_msgs(&mut self) -> Result<(), anyhow::Error> {
        unimplemented!();
    }

    // TODO: Students should implement this function
    #[allow(unused)]
    fn do_work_request(&mut self, request: ClientWorkPacket) -> Result<(), anyhow::Error> {
        unimplemented!();
    }

    // TODO: Students should implement this function
    // Receives data from the ring and writes them to self.ring.recv_msgs.
    // Returns error if ring returns an error
    #[allow(unused)]
    fn recv_msgs_from_ring(&mut self) -> Result<(), anyhow::Error> {
        unimplemented!();
    }

    // TODO: Students should implement this function
    // Sends whatver is in self.send_msgs. Checks each msg (draining the vec)
    // and return an error if any one of them failed.
    #[allow(unused)]
    fn send_messages_to_ring(&mut self) -> Result<(), anyhow::Error> {
        unimplemented!();
    }

    // TODO: Students can implement any other methods here
}

#[derive(Debug)]
struct UnexpectedError(String);

impl fmt::Display for UnexpectedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for UnexpectedError {}
