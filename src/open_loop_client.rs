//! open_loop_client.rs
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
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use minstant::Instant;
use csv::Writer;
use rand_distr::Distribution;

fn client_open_loop(
    send_stream: TcpStream,
    thread_start_time: Instant,
    thread_delay: Duration,
    runtime: Duration,
    packets_sent: Arc<AtomicU64>,
    work: Work,
) {
    let mut rng = rand::thread_rng();
    let lambda = 1. / thread_delay.as_secs_f64(); 
    let exp = rand_distr::Exp::new(lambda).unwrap(); 
    let mut sender = ClientWorkPacketConn::new(ChunkedTcpStream::new(send_stream));
    let mut id: u64 = 0;

    while thread_start_time.elapsed() < runtime {
        let delay_secs = exp.sample(&mut rng);
        let excess_duration = Duration::from_secs_f64(delay_secs);

        thread::sleep(excess_duration);

        let work_packet = ClientWorkPacket::new(id, work);
        if sender.send_work_msg(work_packet).is_err() {
            break;
        }

        packets_sent.fetch_add(1, Ordering::SeqCst);
        id += 1;
    }
}

fn client_recv_loop(
    recv_stream: TcpStream,
    receiver_complete: Arc<AtomicBool>,
) -> Vec<LatencyRecord> {
    let mut receiver = ServerWorkPacketConn::new(ChunkedTcpStream::new(recv_stream));
    let mut latencies: Vec<LatencyRecord> = Vec::new();

    loop {
        match receiver.recv_work_msg() {
            Ok(msg) => {
                if let Some(lat) = msg.calculate_latency(get_current_time_micros()) {
                    latencies.push(lat);
                }
            }
            Err(_e) => {
                if receiver_complete.load(Ordering::SeqCst) {
                    break;
                }
                continue;
            }
        }
    }

    latencies
}

fn init_client(
    server_addr: SocketAddrV4,
    thread_delay: Duration,
    runtime: Duration,
    work: Work,
) -> JoinHandle<Vec<LatencyRecord>> {
    let stream = TcpStream::connect(&server_addr).expect("Couldn't connect to server");
    stream.set_nodelay(true).expect("set_nodelay call failed");
    let thread_start_time = Instant::now();

    let sent = Arc::new(AtomicU64::new(0));
    let done = Arc::new(AtomicBool::new(false));

    {
        let stream = stream.try_clone().expect("Failed to clone stream");
        let sent = sent.clone();
        let done = done.clone();
        let _ = thread::spawn(move || {
            client_open_loop(stream, thread_start_time, thread_delay, runtime, sent, work);
            done.store(true, Ordering::SeqCst);
        });
    }

    let recv_handle = {
        let stream = stream.try_clone().expect("Failed to clone stream");
        let done = done.clone();
        thread::spawn(move || client_recv_loop(stream, done))
    };

    recv_handle
}



pub fn run(
    server_addr: SocketAddrV4,
    num_threads: usize,
    interarrival: Duration,
    runtime: Duration,
    work: Work,
    outdir: PathBuf,
) {
    let thread_delay = interarrival * (num_threads as _);

    println!("start: thread_delay {:?}", thread_delay);
    let join_handles: Vec<JoinHandle<Vec<LatencyRecord>>> = (0..num_threads)
        .map(|_| init_client(server_addr, thread_delay, runtime, work))
        .collect();

    // Collect latencies
    let mut request_latencies: Vec<Vec<LatencyRecord>> = Vec::new();
    for handle in join_handles {
        let thread_latencies = handle.join().unwrap();
        request_latencies.push(thread_latencies);
    }

    let path = outdir.join("open_loop_latencies.csv");
    let mut w = Writer::from_path(&path).unwrap();
    w.write_record(&["idx", "send_us", "recv_us", "server_proc_us", "latency_us"]).unwrap();

    for thread_recs in request_latencies {
        for (i, rec) in thread_recs.iter().enumerate() {
            w.write_record(&[
                i.to_string(),
                rec.send_timestamp.to_string(), 
                rec.recv_timestamp.to_string(), 
                rec.server_processing_time.to_string(), 
                rec.latency.to_string()]).unwrap();
        }
    }
    w.flush().unwrap();
}
