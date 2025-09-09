use crate::serialize::LatencyRecord;
use minstant::Instant;
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

use crate::app::Work;

fn client_open_loop(
    send_stream: TcpStream,
    thread_start_time: Instant,
    thread_delay: Duration,
    runtime: Duration,
    packets_sent: Arc<AtomicU64>,
    work: Work,
) {
    // TODO: Students will have to write this code.
    // NOTE: It might be helpful to look at protocol.rs first. You'll probably
    // be implementing that alongside this function. If you've done
    // closed_loop_client.rs, then much of the work there applies here too so we
    // recommend working on the closed_loop_client.rs file first.
    //
    // This function is the send side of an open loop client. It sends data every `thread_delay`.
    // Your report should include an evaluation of how consistently your implementation achieved a
    // request inter-send duration of `thread_delay`

    // TA NOTE: In Project 0, we use constant arrivals; in project 1, we use exponentially-distributed
    // arrivals. The reference solution below is the project 1 solution. The project 0 solution is
    // the same thing except without sampling the Exp random variable and instead just set
    // `wait_duration` to `thread_delay` directly.
    let mut rng = rand::thread_rng();
    let lambda = 1. / thread_delay.as_micros() as f64;
    let exp = rand_distr::Exp::new(lambda).unwrap();
    let mut excess_duration = Duration::from_secs(0);

    unimplemented!()
}

fn client_recv_loop(
    recv_stream: TcpStream,
    receiver_complete: Arc<AtomicBool>,
) -> Vec<LatencyRecord> {
    // TODO: Students will have to write this code.
    // This function is the recvs responses for an open loop client.
    unimplemented!()
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

    // TODO: Output your request latencies to make your graph. You can calculate
    // your graph data here, or output raw data and calculate them externally.
    // You SHOULD write this output to outdir.

    unimplemented!()
}
