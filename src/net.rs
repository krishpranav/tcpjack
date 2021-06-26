use crate::errors::*;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::{Ipv4Flags, MutableIpv4Packet};
use pnet::packet::ipv6::MutableIpv6Packet;
use pnet::packet::tcp::MutableTcpPacket;
use pnet::packet::MutablePacket;
use pnet::transport::TransportChannelType::Layer3;
use pnet::transport::{transport_channel, TransportReceiver, TransportSender};

pub use pnet::packet::tcp::{ipv4_checksum, ipv6_checksum, TcpFlags};

use log::Level;
use pktparse::ethernet;
use pktparse::tcp::{self, TcpHeader};
use pktparse::{ip, ipv4, ipv6};

use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Connection {
    pub src: SocketAddr,
    pub dst: SocketAddr,
    pub seq: Arc<Mutex<u32>>,
    pub ack: Arc<Mutex<u32>>,
}