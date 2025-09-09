# Project 0: Woonsocket

Welcome to your first project in CS1675!

In this project, you will learn how to measure performance and understand how
different workloads affect the performance of network system. You will be
implementing a closed-loop and open-loop workload generator and a simple server
that handles these requests. After this, you will measure the performance of
your system.

## Deliverables

You will submit a report in the style of a Jupyter Notebook (or similar) with
**at least** the following:

1. A graph showing that your implementation has correct performance behavior (relevant for your open-loop request generator implementation).
2. Throughput-latency graphs for median, 95th, and 99th percentile latency across different workloads
3. A written analysis, using data the graphs show, of the performance characteristics of the closed and open-loop clients.

While this is the minimum requirement, your report should describe and justify
your design decisions and provide evidence that the performance characteristics
you discuss are represented correctly in whichever way you need to. For example,
you can include flamegraphs in your report.

## Grading

We will be live grading (details will be forthcoming). In summary, you will be
meeting one of the course staff, and walking us through what you describe in
your report. We will be asking questions during this process, and you will use
your report's contents as evidence that your project is correct.

## Local Development Infrastructure

You will first develop locally and test your implementation for correctness
locally. To help you in the process, we included a `Dockerfile` in the project
repository which specifies a container with all the necessary tools for this
project.

You can setup the container as follows:

1. First build the image

`docker build -t cs1675 -f Dockerfile .`

1. Then run your docker container and log in

`docker run -v /path/to/your/repo:/repo --name cs1675_container -it cs1675`

2. In another terminal, you can see the running docker container using

`docker ps`

3. You can also run multiple terminal sessions to that same docker container

`docker exec -it cs1675_container /bin/bash`

## Course VMs

While the Dockerfile helps with developing across different platforms, there are
certain tools that can only be used on Linux machines. `perf` is one of those
tools.

You have two options to use `perf`
1. You can develop on a machine that gives `perf` the information it needs or
2. You can use the course VMs.

The submission site to use the course VMs is
[https://cs1675.cs.brown.edu/submit](https://cs1675.cs.brown.edu/submit).
To use our course VMs, you use PA to enter a queue, submitting your Github
assignment repository and the commit ID. Our VMs will run your project and
upload files to PA when complete.

It will upload the following files:

1. stdout and stderr for client and server
2. perf.data for client and server
3. the client's output that you will generate

*Important: to ensure consistency between students, your final report should
only include data from runs on the course VMs.*

## The code

This is a client-server application. The crux of the project is for your client to be able to send requests to the server, the server should then be able to perform a certain task associated with that request and then return a message to the client. We should then record the latency associated with each request sent by the client. 

The client sends a work request (defined in `app.rs`) to the the server as a `ClientWorkPacket` struct and the server responds with a `ServerWorkPacket` struct, both of which are defined in `serialize.rs`. The client and server both communicate via a protocol which you will define in `protocol.rs`. 

The server code for this project will be defined in `tcp_server.rs`. There are 2 types of clients that you will need to implement: a closed-loop client and an open-loop client, defined respectively in `closed_client.rs` and `open_client.rs`. For more information on what exactly the server and clients do, read their respective sections in the handout.

Upon each run of the client, a csv will be produced detailing the latency of reach request sent by the client to the server. You will use these generated csvs to produce a report using Python that will graph different metrics on the latency such as p95 latency, p99 latency, median latency in order to analyze the performance of your applicatin.

> ⚠️⚠️⚠️ If you don't have previous experience with developing multi-threaded applications in Rust, we **strongly** recommend that you check out [The Rust Book](https://rust-book.cs.brown.edu/title-page.html) for an introduction to programming in Rust. Specifically, the chapters on [Ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html) and [Concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html) are going to be very relevant to all projects in this class.

We will now describe the files in this project in more detail.

## Server

The first part of your project that you should focus on should be the server side. 

The server should be able to do the following tasks:
 - Listen for incoming TCP connections on a port
 - Deserialize any packets sent by the client
 - Perform the appropriate work request given by the client

### `tcp_server.rs`
Most of the heavy logic for the server will be implemented in `tcp_server.rs`. 

As you can see, there are 2 functions that you need to fill out:

- `tcp_server(addr: SocketAddrV4)`: This function is what actually sets up the server to bind to the address given in the function arguments. This function should ensure that the server can:
    - Create a TCP listener bound to the provided addr. For more reference on creating TCP listeners in Rust, check out this [link](https://doc.rust-lang.org/std/net/struct.TcpListener.html).
    - Implement an infinite loop to continously accept new client connections
    - Spawn a new thread for each connection using `handle_conn`
    - Handle any errors gracefully

- `handle_conn(stream: TcpStream)`: This function is what is called to manage individual client connections. This function needs:
    - Set up connection handlers for both receiving and sending messages
    - Continuously handle client requests
    - Process work requests and send back responses

#### Considerations

You will need to handle errors gracefully. A server should not crash if your
client so happens to send an erroneous message.

To understand the structs and methods to implement the server, consult
`serialize.rs` and `protocol.rs`. We suggest implementing `protocol.rs` and
`tcp_server.rs` together.

For each client connection, the server will send and receive messages using the
same TCP connection. You will need to initialize a `ClientWorkPacketConn` and a
`ServerWorkPacketConn` using the same stream.

### `chunked_tcp_stream.rs`

In this project you will be using our version of a TcpStream defined in this
file. While it is not necessary for Project 0, you should keep in mind that this
TcpStream can only send or receive data in 128-byte chunks.

### `serialize.rs`

This file contains struct definitions for the `ClientWorkPacket`,
`ServerWorkPacket`, and a `LatencyRecord`. It also implements the `MessageTrait`
for the `ClientWorkPacket` and `ServerWorkPacket` which contains function that
serialize and deserialize these structs.

You will be using these methods defined in this file for when you implement
functions in other files.

### `app.rs`

This file contains the definitions and methods for the `Work` enum. The client
uses this enum (in the `ClientWorkPacket`) to specify what kind of work the
server will do.

There are four types of work:
- `Immediate`: Zero overhead. There is no actual computation and the server
  completes this immediately.
- `Const`: The server pauses for a constant amount of time.
- `Poisson`: The server pauses for an amount of time controlled by the Poisson
  distribution
- `Payload`: The server generates a payload to send back to the client. This is
  not necessary for Project 0.

### `protocol.rs`

You will implement a protocol so that your client and server can communicate.
The protocol consists of two main connection handlers:

#### `ClientWorkPacketConn`

This handler manages sending and receiving `ClientWorkPackets` using the
`ChunkedTcpStream` (`chunked_tcp_stream.rs`).

You need to implement:

- `send_work_msg(&mut self, work_packet: ClientWorkPacket)`

    - This function is responsible for sending a `ClientWorkPacket` on the
      stream it is currently connected on.

- `recv_work_msg(&mut self)`

    - This function recieves responses and then deserializes them into a
      `ClientWorkPacket`.

#### `ServerWorkPacketConn`
This handler manages sending and receiving `ServerWorkPackets` using the
`ChunkedTcpStream`

You need to implement:

- `send_work_msg(&mut self, packet: ServerWorkPacket)`

    - This function is responsible for sending a `ServerWorkPacket` on the
      stream it is currently connected on

- `recv_work_msg(&mut self)`
    - This function recieves responses and then deserializes them into a
      `ServerWorkPacket`.

#### Considerations

You can only add members to these structs but are forbidden from changing the
`stream` member. You can implement your own helper methods and other helper
functions as necessary.

You should serialize the `ClientWorkPacket` and `ServerWorkPacket` using the
serialization methods we provide in `serialize.rs`. For the purposes of this
project, the size of the resulting bytes is indeterminate but is less than 128
bytes. So, you'll need to figure out how to make sure the receiver understands
how many bytes to read to deserialize the struct correctly.

## Client

### `closed_loop_client.rs`

This file defines the logic for a closed loop load generator. The `run` function
initializes `num_threads` clients using the `init_client` function, each with
its own thread. The client work is done in the the `client_worker` function
which you implement.

When the threads finish, it returns `LatencyRecords` (`serialize.rs`). You will
collect these latency records and write result to `outdir` that you can use to
generate throughput-latency graphs.

#### The `run` function

The `run` function has the following arguments

- `num_threads`: The number of clients (threads) your closed loop generator will start
- `runtime`: How long the experiment should last for
- `server_addr`: The socket address on which your client will start the TCP connection
- `work`: The worktype that will be sent to the server from the client
- `outdir`: The directory in which to store the results

After doing all the work, you'll need to do some post processing and write the
data out to a file. Write these results to the directory specified in `outdir`.

The `client_worker` will generally do the following:

1. Create a TCP connection using the `server_addr`. The client connection will
   send and receive messages using the same TCP connection. You will need to
   initialize a `ClientWorkPacketConn` and a `ServerWorkPacketConn` using the
   same stream.

2. Until the end of the experiment (`runtime`), send a `ClientWorkPacket`, wait
   for a server's `ServerWorkPacket` response.

3. Record the latency of the request.

4. A the end of the experiment, return the latencies recorded.
 
### `open_loop_client.rs`

This file defines the logic for the open loop load generator. Similar to the
`closed_loop`, the run function initializes `num_threads` client connections
(see `init_client`). Each connection initializes two threads: one thread sends
messages based on the `interrarrival` and another receives responses. The
`client_open_loop` and `client_recv_loop` functions implement these two tasks
respectively.

A key consideration when running your evaluation: how does `open_loop_client`
scale? What parameters determine the client's offered load?

#### The `run` function

The `run` function has the following arguments

- `server_addr`: The socket address on which your client will start the TCP connection
- `num_threads`: The number of clients (threads) your closed loop generator will start
- `interrarrival`: The number of microseconds to wait between each request sent
- `runtime`: How long the experiment should last for
- `work`: The worktype that will be sent to the server from the client
- `outdir`: The directory in which to store the results

After doing all the work, you'll need to do some post-processing and write the
data out to a file. Write these results to the directory specified in `outdir`.

#### `client_open_loop()`

This function sends requests every `thread_delay`.

First, you need to make sure this function tells the `client_recv_loop` when
it's done and how many requests it sent. The former is handled in `init_client`.
The latter is something you need to keep track of with `packets_sent`.

Second, consider how you want to wait for `thread_delay`. There are several options. Pick one, see what happens. During live grading, we'll ask you for evidence that your implementation strategy was correct.

Third, is there a point at which you can't send requests any faster? Why?

Third, is there a point at which you can't send requests any faster? Why?

#### `client_recv_loop()`

This function receives responses from the server. Expected behavior is that this
should exit when the `client_open_loop` is done. However, this might mean it
receives fewer records than was sent.

Here are a few considerations to think about. Why might this function receive
fewer responses than was sent? What determines how many responses this function
receives?

## Compiling and Running your application

`cargo run --release --bin client -- --help` shows the arguments for the client.
Replace `client` with server for the server. When running in the docker
container or locally, set `ip` to be the localhost IP. `--outpath` will write data out
to this path.

### Running your application on our VMs

As mentioned, we have a set of VMs to run your code and generate output. We'll
be running a pre-determined combination of arguments but will be calling your
application in the same way that you compile and run your application.

## Summary of required functions

### tcp_server.rs
- `tcp_server(addr: SocketAddrV4)`
- `handle_conn(stream: TcpStream)`

### protocol.rs
For `ClientWorkPacketConn`:
- `send_work_msg(
            &mut self,
            work_packet: ClientWorkPacket,
        )`
- `recv_work_msg(&mut self)`

For `ServerWorkPacketConn`:
- `send_work_msg(&mut self, packet: ServerWorkPacket)`
- `recv_work_msg(&mut self)`

### open_loop_client.rs
- `client_recv_loop(
    recv_stream: TcpStream,
    receiver_complete: Arc<AtomicBool>,
    sent: Arc<AtomicU64>,
    received: Arc<AtomicU64>,
) `
- `client_open_loop(
    send_stream: TcpStream,
    thread_start_time: Instant,
    thread_delay: Duration,
    runtime: Duration,
    packets_sent: Arc<AtomicU64>,
    work: Work,
)`

### closed_loop_client.rs
- `client_worker(server_addr: SocketAddrV4, runtime: Duration, work: Work)`
- `init_client(
    server_addr: SocketAddrV4,
    runtime: Duration,
    work: Work,
)`
