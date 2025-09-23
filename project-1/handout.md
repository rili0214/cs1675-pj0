# Project 1: Woonsocket

Good job on Project 0!

In this project, you will re-use much of the implementation you built in Project 0. You will learn how to

1. more accurately characterize client request patterns using the Poisson distribution,
2. send messages that are larger than a pre-determined fixed chunk size,
3. use vectored I/O, and
4. get familiar with `io-uring`.

Running `cargo run --release --bin server -- --help` will show two new possible kinds of servers: `iouring-0` and `io-vec`.

## Deliverables

Along with your implementation of the functionality described in detail below, you will submit a report in the style of a Jupyter notebook that demonstrates and analyzes:

1. the difference between Poisson and constant-rate request arrivals and how you implemented it.
2. the impact of message sizes, both below and above `MSG_SIZE_BYTES`.
3. the difference in performance characteristics between (a) your project 0 implementation, (b) vectored I/O APIs, and (c) `io-uring`.

Specifically, the project rubric's 5 points will be for (i) implementation correctness (0.5 of which will be the milestone 1 checkoff), (ii) evaluating Poisson arrivals, (iii) evaluating Vectored IO, (iv) evaluating io-uring, and (v) comparing io-uring with vectored IO.

Your report should include quantitative evidence. At a minimum, this includes throughput-latency graphs,
a histogram of Poisson interarrival times of the open-loop clients, and unit tests to check the correctness
of handling variable-sized messages. Your report should describe and justify your design decisions and
provide evidence that the performance characteristics you discuss are represented correctly in whichever
way you need to.

### Using the VMs

Project 1 VMs have the following workload options:

1. Immediate
2. Const(10)
3. Const(50)
4. Payload
5. Poisson Work

The first three are the same as in the Project 0. With the Payload workload, the
server returns a variable sized set of bytes (see `app.rs`). With the Poisson
work workload option, the work takes a random amount of time to process by the server
based on a Poisson distribution. This is distinct from client arrivals in your open-loop
workload generator, which you will need to implement.

Each of these options are available for a regular TCP server that you
implemented in Project 0, a server using vectored I/O (Scatter-Gather), and a
server using `io-uring` (IO-uring).

You do not need to include every workload in your report. Use your judgment to
decide which results to include as quantitative evidence for the claims you make
in your deliverable.

### Grading

Milestone 1 is due the week of October 6-10. There's no official submission for Milestone 1. To get credit, we ask that you come to one of course staff's office hours during this week (Oct. 6-10) for a brief demonstration (~2-3 minutes) that you've finished the milestone tasks (i.e. Poisson arrivals, variable-sized messages, and vectored IO server) and your implementation works. You can make changes to your Milestone 1 code after this meeting, but your final report should explain what changes were made and why.

Milestone 2 and the project report is due Tuesday, October 21. At this point, we will be live grading Project 1 in a similar fashion to Project 0. In summary, you will be meeting one of the course staff, and walking us through what you describe in your report. We will be asking questions during this process, and you will use your report's contents as evidence that your project implementation is correct.

## Milestone 1
### Poisson arrivals
Your current implementation of the `client_open_loop` function in `open_loop_client.rs` sends 1 request every `thread_delay` duration. This models a scenario where clients arrive at a constant rate, which does not accurately
reflect the randomness of real-world user behavior.

Your first task is to modify this behavior so that clients arriving follow a __Poisson process__, simulating bursty traffic patterns as well as longer intervals between requests. In this process, the average time of between events is known (in the function, this will be `thread_delay`), but the exact timing is a random variable distributed *exponentially*.

> Note that Poisson distribution refers to the number of clients that arrive within a time interval. However, the length of time between each occurance is given by an exponential distribution. For more information, refer to [this document](https://neurophysics.ucsd.edu/courses/physics_171/exponential.pdf) explaining the relationship between Poisson and exponential distributions.

We will use an open-loop client that models Poisson arrivals for the remainder of the project (we will leave our closed-loop client behind in Project 0 land).

---
### Variable sized messages

In Project 0, we assumed that the size of a `ServerWorkPacket` will always be less than `MSG_SIZE_BYTES`. Beginning this project, this will no longer be the case.

You will need to modify your code in `protocol.rs` so that `ServerWorkPacketConn::send_work_msg` and `ServerWorkPacketConn::recv_work_msg` can send and receive handle `ServerWorkPacket`s that are larger (and smaller) than `MSG_SIZE_BYTES`.

In the `send_work_msg`, this will entail breaking apart `bytes` so that you can send them in separate chunks. In the `recv_work_msg` you will need to assemble these chunks together so that the client can reconstruct the message.

You should not need to modify your client for this to work.

---
### Vectored I/O Server

The file at `src/io_vec_server.rs` contains template code for you to implement a server that uses vectored I/O.

This file defines the logic for a TCP server that uses vectored I/O. You will need to implement two functions:

- `io_vec_server(addr: SocketAddrV4)`: This function is similar to the `tcp_server(addr: SocketAddrV4)` function you implemented in Project 0. It listens for new client connections. For each client connection, it should instantiate an `IOVecServer` and then call `IOVecServer::handle_conn`. You can add any members necessary to `IOVecServer`.

- `IOVecServer::handle_conn(&mut self)`: This function handles one client connection. It should use `chunked_tcp_stream::writev/readv` to receive many messages from the client and send messages in response.

#### Considerations:

This version of the server no longer uses `ClientWorkPacketConn` and `ServerWorkPacketConn`.
You will likely need to modify some of your implementation to handle variable sized messages so that you can re-use them for this server.

Feel free to add members into `IOVecServer` to hold state (such as buffers).

`chunked_tcp_stream::writev/readv` in `chunked_tcp_stream.rs` contains our version of `readv` and `writev`. These are thin wrappers around the `libc` versions that checks that every io_vec sends at most `MSG_SIZE_BYTES` messages.

## Milestone 2

> ⚠️⚠️⚠️ This would be a good time to review the chapter on [Lifetimes](https://rust-book.cs.brown.edu/ch10-03-lifetime-syntax.html) from The Rust Programming Language Book.

### `io-uring` Library

The file at `src/io_uring.rs` will provide __library__ functions for your `io-uring` server. It will contain the logic for sending and receiving `RingMsg`s using an io-uring.

You will need to implement the following:

- `RingMsg<'a>`: This struct encapsulates a message to write into the ring. You can add members to this struct. For example, you should probably keep track of whether the kernel successfully sent the message.

- `IOUring`: This struct encapsulates IoUring behavior in `send_msgs` and `recv_msgs`. You will need to implement those functions. This struct should __only__ care about the IOUring so its members __should not__ change.

#### Considerations:

`RingMsg<'a>` is bound to the lifetime of its member `RawMsg::data`. This member is an exclusively borrowed mutable array (`&'a mut [u8; MSG_SIZE_BYTES]`). Your server implementation (see below) will need to make sure that any arrays you pass into the ring will have a sufficient lifetime. We also chose the  `&'a mut [u8; MSG_SIZE_BYTES]` type so that we can ensure we don't send or recv messages larger than `MSG_SIZE_BYTES`.

---
### `io-uring` Server

The file at `src/io_uring_server.rs` contains the server logic that uses the `io_uring` library you implemented (see above).

You will need to implement the following:

- `io_uring_server(addr: SocketAddrV4, ring_sz: usize)` listens for connections and calls `handle_conn` for each.

- `handle_conn(addr: SocketAddrV4, ring_sz: usize)`: This function handles one client connection.  It will need to setup an `IOUringServer` struct and then call `IOUringServer::serve`.

- `IOUringServer`: This struct contains the logic for your `io-uring`-based server. To manage the complexity of `io-uring`, we have broken up the logic into several functions you will need to implement. We outline each below.

  - `IOUringServer::serve(serve)`: This function will contain your server loop.  This function consumes the struct that calls it. It should:

    - Receive `RingMsg`s from the ring
    - Construct `ClientWorkPackets` from these messages
    - For each `ClientWorkPacket`, perform work and prepare `RingMsg`s to send to the ring
    - And then send prepared messages to the ring.
    - Repeat.

  - `IOUringServer::recv_msgs_from_ring(&mut self)`: This function receives messages from the ring.

  - `IOUringServer::handle_recv_msgs(&mut self)`: After receiving messages from the ring, this function will put together `ClientWorkPacket`s. It will then call `do_work_request` for each.

  - `IOUringServer::do_work_request(&mut self, request: ClientWorkPacket)`: This function performs the request's work. The resulting `ServerWorkPacket` will be variable sized so you will need to split a serialized `ServerWorkPacket` to `MSG_SIZE_BYTES` chunks.

  - `IOUringServer::send_messages_to_ring(&mut self)`: This function will send prepared `RingMsg`s to the ring and wait for completion.

#### Considerations:

You can change `IOUringServer`'s struct definition if useful. This might help resolve lifetime errors when sending `RingMsg<'a>` to the ring.
