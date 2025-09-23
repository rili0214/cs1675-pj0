use crate::protocol::MSG_SIZE_BYTES;
use std::os::fd::RawFd;

/// HINT: here are some potentially useful imports from external libraries. You shouldn't need
/// other external libraries. You can remove the allow(unused) in your code.
#[allow(unused)]
use io_uring::{cqueue, opcode, squeue, types, IoUring};
#[allow(unused)]
use libc::{iovec, msghdr, sockaddr_in};
#[allow(unused)]
use nix::sys::socket::{SockaddrIn, SockaddrLike};

pub struct RingMsg<'a> {
    pub raw_fd: RawFd,
    pub data: &'a mut [u8; MSG_SIZE_BYTES],
    pub user_data: u64,
    // TODO: students can add members to this struct.
}

// ------------------------------------
// TODO: students add structs and impls here

// ------------------------------------

#[allow(unused)]
pub struct IOUring {
    // Students should NOT modify the members here
    ring: IoUring<squeue::Entry, cqueue::Entry>,
}

impl IOUring {
    // TODO: Students implement this function
    #[allow(unused)]
    pub fn new(slots: u32) -> Result<Self, anyhow::Error> {
        unimplemented!();
    }

    // TODO: Students implement this function
    #[allow(unused)]
    pub fn send_msgs(&mut self, msgs: &mut [RingMsg]) -> Result<(), anyhow::Error> {
        unimplemented!();
    }

    // TODO: Students implement this function
    #[allow(unused)]
    pub fn recv_msgs(&mut self, msgs: &mut [RingMsg]) -> Result<(), anyhow::Error> {
        unimplemented!();
    }

    // TODO: students can make their own helper functions
}
