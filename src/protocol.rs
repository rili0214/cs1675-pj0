//! protocol.rs
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
        pub fn new(stream: ChunkedTcpStream) -> Self {
            Self { stream }
        }

        pub fn send_work_msg(
            &mut self,
            work_packet: ClientWorkPacket,
        ) -> Result<(), anyhow::Error> {
            let mut data_to_send = Vec::new();
            work_packet.to_vec(&mut data_to_send)?;
            let total_len = data_to_send.len() as u64;

            let mut header_chunk = [0u8; MSG_SIZE_BYTES];
            header_chunk[..8].copy_from_slice(&total_len.to_be_bytes());
            self.stream.send_msg_chunk(&header_chunk)?;

            for chunk in data_to_send.chunks(MSG_SIZE_BYTES) {
                let mut buffer = [0u8; MSG_SIZE_BYTES];
                buffer[..chunk.len()].copy_from_slice(chunk);
                self.stream.send_msg_chunk(&buffer)?;
            }

            Ok(())
        }

        pub fn recv_work_msg(&mut self) -> Result<ClientWorkPacket, anyhow::Error> {
            let mut first_chunk = [0u8; MSG_SIZE_BYTES];
            self.stream.recv_msg_chunk(&mut first_chunk)?;
            let total_len = u64::from_be_bytes(first_chunk[..8].try_into()?) as usize;

            let mut received_data = Vec::with_capacity(total_len);
            let first_data_len = (total_len).min(MSG_SIZE_BYTES - 8);
            received_data.extend_from_slice(&first_chunk[8..8 + first_data_len]);

            while received_data.len() < total_len {
                let mut chunk_buf = [0u8; MSG_SIZE_BYTES];
                self.stream.recv_msg_chunk(&mut chunk_buf)?;

                let bytes_to_take = (total_len - received_data.len()).min(MSG_SIZE_BYTES);
                received_data.extend_from_slice(&chunk_buf[..bytes_to_take]);
            }

            let packet = ClientWorkPacket::from_bytes(&received_data)?;
            Ok(packet)
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
            let mut data_to_send = Vec::new();
            packet.to_vec(&mut data_to_send)?;
            let total_len = data_to_send.len() as u64;

            let mut header_chunk = [0u8; MSG_SIZE_BYTES];
            header_chunk[..8].copy_from_slice(&total_len.to_be_bytes());
            self.stream.send_msg_chunk(&header_chunk)?;

            for chunk in data_to_send.chunks(MSG_SIZE_BYTES) {
                let mut buffer = [0u8; MSG_SIZE_BYTES];
                buffer[..chunk.len()].copy_from_slice(chunk);
                self.stream.send_msg_chunk(&buffer)?;
            }

            Ok(())
        }

        pub fn recv_work_msg(&mut self) -> Result<ServerWorkPacket, anyhow::Error> {
            let mut first_chunk = [0u8; MSG_SIZE_BYTES];
            self.stream.recv_msg_chunk(&mut first_chunk)?;
            let total_len = u64::from_be_bytes(first_chunk[..8].try_into()?) as usize;

            let mut received_data = Vec::with_capacity(total_len);
            let first_data_len = (total_len).min(MSG_SIZE_BYTES - 8);
            received_data.extend_from_slice(&first_chunk[8..8 + first_data_len]);

            while received_data.len() < total_len {
                let mut chunk_buf = [0u8; MSG_SIZE_BYTES];
                self.stream.recv_msg_chunk(&mut chunk_buf)?;

                let bytes_to_take = (total_len - received_data.len()).min(MSG_SIZE_BYTES);
                received_data.extend_from_slice(&chunk_buf[..bytes_to_take]);
            }

            let packet = ServerWorkPacket::from_bytes(&received_data)?;
            Ok(packet)
        }

        pub fn new(stream: ChunkedTcpStream) -> Self {
            Self { stream }
        }
    }

    // TODO: Students can add helper functions here.
}
