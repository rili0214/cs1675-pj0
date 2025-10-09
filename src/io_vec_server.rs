//! io_vec_server.rs
use crate::{
    chunked_tcp_stream::{writev, MSG_SIZE_BYTES},
    serialize::{ClientWorkPacket, MessageTrait},
};
use libc::iovec;
use std::{
    io::{Read, Write},
    net::{SocketAddrV4, TcpListener, TcpStream},
    os::fd::AsRawFd,
    thread,
};

// TODO: Students will have to implement this function
pub fn io_vec_server(addr: SocketAddrV4) {
    let listener = TcpListener::bind(addr).expect("failed to bind TCP listener");
    println!("io_vec_server listening on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    let mut server = IOVecServer { stream };
                    if let Err(e) = server.handle_conn() {
                        if e.downcast_ref::<std::io::Error>().map_or(true, |io_err| io_err.kind() != std::io::ErrorKind::UnexpectedEof) {
                            eprintln!("[io_vec_server] Connection error: {}", e);
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("[io_vec_server] Incoming connection error: {}", e);
            }
        }
    }
}

struct IOVecServer {
    stream: TcpStream,
}

impl IOVecServer {
    // TODO: Students will have to implement this function.
    // Students SHOULD use chunked_tcp_stream::writev/readv which checks to
    // make sure messages are no larger than MSG_SIZE_BYTES. It is a
    // thin wrapper around libc::writev/readv.
    fn handle_conn(&mut self) -> Result<(), anyhow::Error> {
        loop {
            let mut first_chunk = [0u8; MSG_SIZE_BYTES];
            self.stream.read_exact(&mut first_chunk)?;
            let total_len = u64::from_be_bytes(first_chunk[..8].try_into()?) as usize;

            let mut received_data = Vec::with_capacity(total_len);

            let first_data_len = (total_len).min(MSG_SIZE_BYTES - 8);
            received_data.extend_from_slice(&first_chunk[8..8 + first_data_len]);

            while received_data.len() < total_len {
                let mut chunk_buf = [0u8; MSG_SIZE_BYTES];
                self.stream.read_exact(&mut chunk_buf)?;
                let bytes_to_take = (total_len - received_data.len()).min(MSG_SIZE_BYTES);
                received_data.extend_from_slice(&chunk_buf[..bytes_to_take]);
            }

            let request = ClientWorkPacket::from_bytes(&received_data)?;

            let response_packet = request.do_work();
            
            let mut response_data = Vec::new();
            response_packet.to_vec(&mut response_data)?;
            let response_len = response_data.len() as u64;

            self.stream.write_all(&response_len.to_be_bytes())?;

            let mut iovecs: Vec<iovec> = Vec::new();
            for chunk in response_data.chunks(MSG_SIZE_BYTES) {
                let iov = iovec {
                    iov_base: chunk.as_ptr() as *mut _,
                    iov_len: chunk.len(), 
                };
                iovecs.push(iov);
            }

            let iovecs_len = iovecs.len() as i32;
        
            unsafe {
                writev(self.stream.as_raw_fd(), &mut iovecs, iovecs_len);
            }
        }
    }

    // TODO: Students can add helper functions here
}
