use crate::{
    app::Work,
    chunked_tcp_stream::ChunkedTcpStream,
    protocol::work_request::ClientWorkPacketConn,
    protocol::work_response::ServerWorkPacketConn,
    serialize::{ClientWorkPacket, LatencyRecord},
    get_current_time_micros,
};

use std::{
    io, // 导入 io 模块以使用 ErrorKind
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
    thread_delay: Duration, // This is the constant delay for P0
    runtime: Duration,
    packets_sent: Arc<AtomicU64>,
    work: Work,
) {
    // --- REMOVE THE POISSON LOGIC FOR PROJECT 0 ---
    // let mut rng = rand::thread_rng();
    // let lambda = 1. / thread_delay.as_secs_f64(); 
    // let exp = rand_distr::Exp::new(lambda).unwrap(); 
    
    let mut sender = ClientWorkPacketConn::new(ChunkedTcpStream::new(send_stream));
    let mut id: u64 = 0;

    // --- ADD ABSOLUTE SCHEDULING ---
    let mut next_send_time = thread_start_time;
    let end_time = thread_start_time + runtime;

    while Instant::now() < end_time {
        // --- REPLACE FLAWED SLEEP WITH ABSOLUTE PACING ---
        // let delay_secs = exp.sample(&mut rng);
        // let excess_duration = Duration::from_secs_f64(delay_secs);
        // thread::sleep(excess_duration);

        // Calculate the next send time based on a constant rate
        next_send_time += thread_delay;
        
        let now = Instant::now();
        if next_send_time > now {
            // If we have time, sleep until the next scheduled send
            thread::sleep(next_send_time - now);
        }
        // If we are *behind* schedule (now >= next_send_time),
        // this loop will run again immediately to "catch up".

        // Ensure we don't send after the runtime has expired
        if Instant::now() >= end_time {
            break;
        }
        // --- END OF PACING FIX ---

        let work_packet = ClientWorkPacket::new(id, work);
        if sender.send_work_msg(work_packet).is_err() {
            break;
        }

        packets_sent.fetch_add(1, Ordering::SeqCst);
        id += 1;
    }
}

// Add this import at the top of the file
use std::io;

fn client_recv_loop(
    recv_stream: TcpStream,
    receiver_complete: Arc<AtomicBool>,
    packets_sent: Arc<AtomicU64>, // <-- ADD THIS ARGUMENT
) -> Vec<LatencyRecord> {
    
    // --- SET A READ TIMEOUT ---
    // This prevents the receiver from blocking forever if the connection is idle
    recv_stream.set_read_timeout(Some(Duration::from_millis(500)))
        .expect("Failed to set read timeout");

    let mut receiver = ServerWorkPacketConn::new(ChunkedTcpStream::new(recv_stream));
    let mut latencies: Vec<LatencyRecord> = Vec::new();
    
    // --- ADD A PACKET COUNTER ---
    let mut packets_received = 0u64;

    loop {
        // --- NEW, ROBUST LOOP LOGIC ---
        let is_complete = receiver_complete.load(Ordering::SeqCst);
        let total_sent = packets_sent.load(Ordering::Relaxed);
        
        // Exit condition: sender is done AND we've received all sent packets.
        if is_complete && packets_received >= total_sent {
            break;
        }

        match receiver.recv_work_msg() {
            Ok(msg) => {
                if let Some(lat) = msg.calculate_latency(get_current_time_micros()) {
                    latencies.push(lat);
                }
                packets_received += 1; // Increment received counter
            }
            Err(e) => {
                // Check if the error is a timeout
                if let Some(io_err) = e.downcast_ref::<io::Error>() {
                    if io_err.kind() == io::ErrorKind::WouldBlock || io_err.kind() == io::ErrorKind::TimedOut {
                        // It's just a timeout.
                        // If the sender is done but we're still missing packets,
                        // this timeout allows us to loop again and re-check the exit condition.
                        // If sender isn't done, we just continue waiting.
                        continue;
                    }
                }

                // It's a real error (e.g., connection closed)
                if is_complete {
                    // Sender is done and connection closed. This is an expected end.
                    break;
                } else {
                    // Connection broke unexpectedly
                    eprintln!("Receiver loop error: {}", e);
                    break;
                }
            }
        }
    }

    latencies
}


// 在 src/open_loop_client.rs 中

fn init_client(
    server_addr: SocketAddrV4,
    thread_delay: Duration,
    runtime: Duration,
    work: Work,
) -> JoinHandle<Vec<LatencyRecord>> {
    // ... (stream setup is the same) ...
    let stream = TcpStream::connect(&server_addr).expect("Couldn't connect to server");
    stream.set_nodelay(true).expect("set_nodelay call failed");
    let thread_start_time = Instant::now();

    let sent = Arc::new(AtomicU64::new(0));
    let done = Arc::new(AtomicBool::new(false));

    {
        // ... (sender thread spawn is the same) ...
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
        // --- ADD sent.clone() HERE ---
        let sent_clone = sent.clone();
        thread::spawn(move || client_recv_loop(stream, done, sent_clone)) // <-- PASS IT HERE
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
    let thread_delay = interarrival * (num_threads as u32); // 注意: 之前这里乘以 usize 可能导致溢出，改为 u32

    println!("start: thread_delay {:?}", thread_delay);
    let join_handles: Vec<JoinHandle<Vec<LatencyRecord>>> = (0..num_threads)
        .map(|_| init_client(server_addr, thread_delay, runtime, work))
        .collect();

    let mut request_latencies: Vec<Vec<LatencyRecord>> = Vec::new();
    for handle in join_handles {
        let thread_latencies = handle.join().unwrap();
        request_latencies.push(thread_latencies);
    }

    let path = outdir;

    let mut w = Writer::from_path(&path).expect("Failed to create CSV writer");

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
