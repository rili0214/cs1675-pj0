use libc::{self, iovec};
use std::{
    io::{Read, Write},
    net::TcpStream,
    os::fd::RawFd,
};

pub const MSG_SIZE_BYTES: usize = 128;

pub struct ChunkedTcpStream(TcpStream);

impl ChunkedTcpStream {
    pub fn send_msg_chunk(&mut self, bytes: &[u8]) -> Result<(), anyhow::Error> {
        assert!(bytes.len() <= MSG_SIZE_BYTES);
        self.0.write_all(&bytes)?;
        self.0.flush()?;
        Ok(())
    }

    pub fn recv_msg_chunk(&mut self, bytes: &mut [u8]) -> Result<(), anyhow::Error> {
        assert!(bytes.len() <= MSG_SIZE_BYTES);
        self.0.read_exact(bytes)?;
        Ok(())
    }

    pub fn new(tcp: TcpStream) -> Self {
        Self(tcp)
    }
}

pub unsafe fn writev(raw_fd: RawFd, iovecs: &mut [iovec], send_idx: i32) -> isize {
    for v in iovecs.iter() {
        assert!(v.iov_len as usize <= MSG_SIZE_BYTES);
    }
    libc::writev(raw_fd, iovecs.as_mut_ptr(), send_idx)
}

pub unsafe fn readv(raw_fd: RawFd, iovecs: &mut [iovec], send_idx: i32) -> isize {
    for v in iovecs.iter() {
        assert!(v.iov_len as usize <= MSG_SIZE_BYTES);
    }
    libc::readv(raw_fd, iovecs.as_mut_ptr(), send_idx)
}
