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

impl Connection {
    #[inline]
    pub fn new(src: SocketAddr, dst: SocketAddr, seq: u32, ack: u32) -> Connection {
        Connection {
            src,
            dst,
            seq: Arc::new(Mutex::new(seq)),
            ack: Arc::new(Mutex::new(ack)),
        }
    }

    #[inline]
    pub fn bump_seq(&self, inc: u32) {
        let mut guard = self.seq.lock().unwrap();
        *guard += inc;
    }

    #[inline]
    pub fn set_ack(&self, ack: u32) {
        let mut guard = self.ack.lock().unwrap();
        *guard = ack;
    }

    #[inline]
    pub fn get_seq(&self) -> u32 {
        *self.seq.lock().unwrap()
    }

    #[inline]
    pub fn get_ack(&self) -> u32 {
        *self.ack.lock().unwrap()
    }

    #[inline]
    pub fn sendtcp(&mut self, tx: &mut TransportSender, flags: u16, data: &[u8]) -> Result<()> {
        sendtcp(
            tx,
            &self.src,
            &self.dst,
            flags,
            self.get_seq(),
            self.get_ack(),
            &data,
        )?;
        self.bump_seq(data.len() as u32);
        Ok(())
    }

    #[inline]
    pub fn ack(&mut self, tx: &mut TransportSender, mut ack: u32, data: &[u8]) -> Result<()> {
        ack += data.len() as u32;
        self.set_ack(ack);
        sendtcp(
            tx,
            &self.src,
            &self.dst,
            TcpFlags::ACK,
            self.get_seq(),
            ack,
            &[],
        )
    }

    #[inline]
    pub fn reset(&mut self, tx: &mut TransportSender) -> Result<()> {
        sendtcp(
            tx,
            &self.src,
            &self.dst,
            TcpFlags::RST,
            self.get_seq(),
            0,
            &[],
        )
    }
}


pub struct IpHeader {
    source_addr: IpAddr,
    dest_addr: IpAddr,
}