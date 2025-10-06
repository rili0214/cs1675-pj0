use std::net::SocketAddrV4;

// TODO: Students will have to implement this function
pub fn io_vec_server(addr: SocketAddrV4) {
    unimplemented!();
}

struct IOVecServer {
    // TODO: Students will have to implement the server and can add members
    // to the server struct here.
}

impl IOVecServer {
    // TODO: Students will have to implement this function.
    // Students SHOULD use chunked_tcp_stream::writev/readv which checks to
    // make sure messages are no larger than MSG_SIZE_BYTES. It is a
    // thin wrapper around libc::writev/readv.
    fn handle_conn(&mut self) -> Result<(), anyhow::Error> {
        unimplemented!()
    }

    // TODO: Students can add helper functions here
}
