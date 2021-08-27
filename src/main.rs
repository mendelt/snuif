use std::{fmt::Display, ops::Deref};

use rawsock::open_best_library;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "snuif")]
enum Cli {
    /// Capture and display raw packets
    Raw {
        /// The number of packets to capture
        count: u64,
    },
    /// List the available interfaces
    List,
}

impl Cli {
    fn run(self) {
        match self {
            Cli::Raw { count } => raw(count),
            Cli::List => todo!(),
        }
    }
}

fn main() {
    let command = Cli::from_args();
    command.run();
}

fn raw(count: u64) {
    println!("Opening packet capturing library");
    let lib = open_best_library().expect("Could not open any packet capturing library");
    println!("Library opened, version is {}", lib.version());
    let interf_name = lib.all_interfaces()
        .expect("Could not obtain interface list").first()
        .expect("There are no available interfaces").name.clone();
    println!("Opening the {} interface", &interf_name);
    let mut interf = lib.open_interface(&interf_name).expect("Could not open network interface");
    println!("Interface opened, data link: {}", interf.data_link());

    //receive some packets.
    println!("Receiving {} packets:", count);
    for _ in 0..count {
        let packet = interf.receive().expect("Could not receive packet");
        println!("{}", packet);

        let _parser = EthernetFrame::new(packet.deref());
    }
}

/// Parse ethernet packet
struct EthernetFrame<'a> {
    packet: &'a [u8]
}

impl<'a> EthernetFrame<'a> {
    pub fn new(bytes: &'a [u8]) -> EthernetFrame<'a> {
        EthernetFrame { packet: bytes }
    }

    pub fn source(&self) -> u64 {
        todo!()
    }

    pub fn dest(&self) -> u64 {
        todo!()
    }

    pub fn vlan(&self) -> Option<u8> {
        todo!()
    }

    pub fn ethertype(&self) -> EtherType {
        todo!()
    }

    pub fn payload(&self) -> &[u8] {
        todo!()
    }
}

impl Display for EthernetFrame<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "")
    }
}

#[repr(u16)]
pub enum EtherType {
    IPv4 = 0x0800,

    Unknown,
}

