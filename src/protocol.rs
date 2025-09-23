pub use crate::chunked_tcp_stream::MSG_SIZE_BYTES;
use crate::{
    chunked_tcp_stream::ChunkedTcpStream,
    serialize::{ClientWorkPacket, ServerWorkPacket, MessageTrait},
};

pub mod work_request {
    #[allow(unused_mut)]
    use super::*;

    pub struct ClientWorkPacketConn {
        stream: ChunkedTcpStream,
	// TODO: students can add their own members here
    }

    impl ClientWorkPacketConn {
        pub fn send_work_msg(
            &mut self,
            work_packet: ClientWorkPacket,
        ) -> Result<(), anyhow::Error> {
            let mut msg = [0u8; MSG_SIZE_BYTES];
            let len = work_packet.to_bytes(&mut msg[4..])? as usize;
            msg[0..4].copy_from_slice(&(len as u32).to_be_bytes());
            self.stream.send_msg_chunk(&msg)?;
            Ok(())
        }

        pub fn recv_work_msg(&mut self) -> Result<ClientWorkPacket, anyhow::Error> {
            let mut msg = [0u8; MSG_SIZE_BYTES];
            self.stream.recv_msg_chunk(&mut msg)?;
            let len = u32::from_be_bytes(msg[0..4].try_into().unwrap()) as usize;
            let work_packet = ClientWorkPacket::from_bytes(&msg[4..4 + len])?;
            Ok(work_packet)
        }

        pub fn new(stream: ChunkedTcpStream) -> Self {
            Self { stream }
        }
    }

    // TODO: Students can add helper functions here.
}

pub mod work_response {
    use super::*;

    pub struct ServerWorkPacketConn {
        stream: ChunkedTcpStream,
	// TODO: students can add their own members here
    }

    impl ServerWorkPacketConn {
        pub fn send_work_msg(&mut self, packet: ServerWorkPacket) -> Result<(), anyhow::Error> {
            let mut msg = [0u8; MSG_SIZE_BYTES];
            let len = packet.to_bytes(&mut msg[4..])? as usize;
            msg[0..4].copy_from_slice(&(len as u32).to_be_bytes());
            self.stream.send_msg_chunk(&msg)?;
            Ok(())
        }

        pub fn recv_work_msg(&mut self) -> Result<ServerWorkPacket, anyhow::Error> {
            let mut msg = [0u8; MSG_SIZE_BYTES];
            self.stream.recv_msg_chunk(&mut msg)?;
            let len = u32::from_be_bytes(msg[0..4].try_into().unwrap()) as usize;
            let work_packet = ServerWorkPacket::from_bytes(&msg[4..4 + len])?;
            Ok(work_packet)
        }

        pub fn new(stream: ChunkedTcpStream) -> Self {
            Self { stream }
        }
    }

    // TODO: Students can add helper functions here.
}
