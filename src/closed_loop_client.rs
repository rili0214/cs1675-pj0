use crate::{
    app::Work,
    chunked_tcp_stream::ChunkedTcpStream,
    protocol::work_request::ClientWorkPacketConn,
    protocol::work_response::ServerWorkPacketConn,
    serialize::{ClientWorkPacket, LatencyRecord},
    get_current_time_micros,
};

use std::{
    net::{SocketAddrV4, TcpStream},
    path::PathBuf,
    thread::{self, JoinHandle},
    time::Duration,
};

fn client_worker(server_addr: SocketAddrV4, runtime: Duration, work: Work) -> Vec<LatencyRecord> {
    let stream = TcpStream::connect(server_addr).unwrap();
    let sender = stream.try_clone().unwrap();
    let mut client_conn = ClientWorkPacketConn::new(ChunkedTcpStream::new(sender));
    let mut server_conn = ServerWorkPacketConn::new(ChunkedTcpStream::new(stream));

    let mut latencies: Vec<LatencyRecord> = Vec::new();
    let mut id: u64 = 0;
    let start = Instant::now();

    while start.elapsed() < runtime {
        let work_packet = ClientWorkPacket::new(id, work);
        client_conn.send_work_msg(work_packet).unwrap();

        let msg = server_conn.recv_work_msg().unwrap();
        if let Some(lat) = msg.calculate_latency(get_current_time_micros()) {
            latencies.push(lat);
        }
        
        id += 1;
    }

    latencies
}

pub fn init_client(
    server_addr: SocketAddrV4,
    runtime: Duration,
    work: Work,
) -> JoinHandle<Vec<LatencyRecord>> {
    thread::spawn(move || client_worker(server_addr, runtime, work))
}

pub fn run(
    server_addr: SocketAddrV4,
    num_threads: usize,
    runtime: Duration,
    work: Work,
    outdir: PathBuf,
) {
    let join_handles: Vec<_> = (0..num_threads)
        .map(|_| init_client(server_addr, runtime, work))
        .collect();

    // Collect latencies
    let mut request_latencies: Vec<Vec<LatencyRecord>> = Vec::new();
    for handle in join_handles {
        let thread_latencies = handle.join().unwrap();
        request_latencies.push(thread_latencies);
    }

    let path = outdir.join("closed_loop_latencies.csv");
    let mut w = Writer::from_path(&path).unwrap();
    w.write_record(&["latency_us"]).unwrap();

    for thread_recs in request_latencies {
        for rec in thread_recs {
            w.write_record(&[rec.latency.to_string()]).unwrap();
        }
    }
    w.flush().unwrap();
}
