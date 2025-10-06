//! Server logic for the CS1675 Woonsocket project.

use clap::{Parser, ValueEnum};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use woonsocket::{
    io_uring_server::io_uring_server, io_vec_server::io_vec_server, tcp_server::tcp_server,
};

#[derive(Copy, Clone, Debug, ValueEnum, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum ServerKind {
    tcp,
    iouring_0,
    io_vec,
}

impl ServerKind {
    pub fn as_string_arg(&self) -> String {
        match self {
            Self::tcp => "tcp",
            Self::iouring_0 => "iouring-0",
            Self::io_vec => "io-vec",
        }
        .into()
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    port: u16,

    #[arg(short, long)]
    kind: ServerKind,

    #[arg(long)]
    ring_sz: Option<usize>,

    #[arg(long)]
    runtime_secs: u64,
}

fn main() {
    let args = Args::parse();
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, args.port);
    std::thread::spawn(move || match args.kind {
        ServerKind::tcp => tcp_server(addr),
        ServerKind::io_vec => io_vec_server(addr),
        ServerKind::iouring_0 => io_uring_server(addr, args.ring_sz.unwrap()),
    });
    std::thread::sleep(Duration::from_secs(args.runtime_secs));
}
