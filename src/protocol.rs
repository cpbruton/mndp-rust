#![allow(unused_imports)]
#![allow(dead_code)]

use std::convert::{TryInto, TryFrom};
use std::mem::size_of;
use std::time::Duration;

use bytes::{Bytes, BytesMut, Buf, BufMut};

use crate::{Neighbor, Unpack};

// MNDP type values
const MNDP_MAC_ADDRESS: u16 = 1;
const MNDP_IDENTITY: u16 = 5;
const MNDP_VERSION: u16 = 7;
const MNDP_PLATFORM: u16 = 8;
const MNDP_UPTIME: u16 = 10;
const MNDP_SOFTWARE_ID: u16 = 11;
const MNDP_BOARD: u16 = 12;
const MNDP_UNPACK: u16 = 14;
const MNDP_IPV6_ADDRESS: u16 = 15;
const MNDP_INTERFACE_NAME: u16 = 16;
const MNDP_IPV4_ADDRESS: u16 = 17;

/// MNDP field type (converts to/from `u16`)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(strum::EnumIter))]
#[repr(u16)]
pub enum MndpType {
    MacAddress = MNDP_MAC_ADDRESS,
    Identity = MNDP_IDENTITY,
    Version = MNDP_VERSION,
    Platform = MNDP_PLATFORM,
    Uptime = MNDP_UPTIME,
    SoftwareId = MNDP_SOFTWARE_ID,
    Board = MNDP_BOARD,
    Unpack = MNDP_UNPACK,
    Ipv6Address = MNDP_IPV6_ADDRESS,
    InterfaceName = MNDP_INTERFACE_NAME,
    Ipv4Address = MNDP_IPV4_ADDRESS,
    // Important: All variants must implement TryFrom<u16> correctly, below.
}

impl TryFrom<u16> for MndpType {
    type Error = ();

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        use MndpType::*;
        match n {
            x if x == MacAddress as u16 => Ok(MacAddress),
            x if x == Identity as u16 => Ok(Identity),
            x if x == Version as u16 => Ok(Version),
            x if x == Platform as u16 => Ok(Platform),
            x if x == Uptime as u16 => Ok(Uptime),
            x if x == SoftwareId as u16 => Ok(SoftwareId),
            x if x == Board as u16 => Ok(Board),
            x if x == Unpack as u16 => Ok(Unpack),
            x if x == Ipv6Address as u16 => Ok(Ipv6Address),
            x if x == InterfaceName as u16 => Ok(InterfaceName),
            x if x == Ipv4Address as u16 => Ok(Ipv4Address),
            _ => Err(()),
        }
    }
}

/// Individual TLV field within an MNDP packet.
/// The length is implicit from the value.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct TypeValue {
    /// MNDP type
    pub typ: u16,
    /// Field bytes.
    pub value: Bytes
}

impl TypeValue {
    /// Create a new TLV field with default/empty contents.
    pub fn new() -> TypeValue {
        Default::default()
    }
}

/// MNDP packet struct with conversions to/from `Neighbor` and raw bytes.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Packet {
    header: u16,
    sequence: u16,
    fields: Vec<TypeValue>
}

impl From<Packet> for Bytes {
    fn from(packet: Packet) -> Self {
        packet.to_bytes()
    }
}

impl Packet {
    /// Create a new `Packet` with default values (0) for header and sequence and
    /// an empty `Vec<TypeValue>` for fields to be added to.
    pub fn new() -> Packet {
        Default::default()
    }

    /// Produce raw bytes from a `Packet` in MNDP protocol format.
    pub fn to_bytes<B: From<Bytes>>(&self) -> B {

        // Allocate a new Bytes buffer with a reasonable capacity
        // (Ethernet payload size minus IPv6 and UDP headers)
        let mut buf = BytesMut::with_capacity(1452);
        
        // Write the header and sequence
        buf.put_u16(self.header);
        buf.put_u16(self.sequence);

        // Write each TLV
        for tv in &self.fields {
            buf.put_u16(tv.typ);

            // A bit of an edge case but we should check that
            // the length will fit into a u16
            let len = if tv.value.len() >= 65535 {
                65535
            } else {
                tv.value.len().into()
            };
            
            // This (usize -> u16) will not panic because we check length above
            buf.put_u16(len.try_into().unwrap());
            buf.put(tv.value.slice(0..len));
        }

        // Convert to immutable and return
        buf.freeze().into()
    }
    /// Create a new `Packet` instance by parsing raw bytes in MNDP format.
    /// Returns an error if input is shorter than 4 bytes.
    pub fn from_bytes<B: Into<Bytes>>(bytes: B) -> Result<Packet, ()> {
        let mut buf: Bytes = bytes.into();

        // Check that buf is minimum required length (2 byte header, 2 byte seq id)
        if buf.len() < 4 {
            return Err(());
        }

        // Create a new packet
        let mut packet = Packet::new();

        // Get the header and seq
        packet.header = buf.get_u16();
        packet.sequence = buf.get_u16();

        // Eat the TLVs
        while buf.remaining() >= 4 {
            // Get the type and length
            let typ = buf.get_u16();
            let len = buf.get_u16();

            // Get the data if enough bytes remain
            if buf.remaining() >= len.into() {
                let bytes = buf.split_to(len.into());
                packet.fields.push(TypeValue {
                    typ: typ,
                    value: bytes
                });
            }
        }

        Ok(packet)
    }

    /// Create a new `Neighbor` from a `Packet`.
    pub fn to_neighbor(&self) -> Neighbor {

        // Todo: Do length checks for non-string types

        let mut neighbor = Neighbor::builder();

        for tv in &self.fields {
            if let Ok(typ) = tv.typ.try_into() {
                neighbor = match typ {
                    MndpType::Board => neighbor.board(String::from_utf8_lossy(&tv.value).to_string()),
                    MndpType::Identity => neighbor.identity(String::from_utf8_lossy(&tv.value).to_string()),
                    MndpType::InterfaceName => neighbor.interface_name(String::from_utf8_lossy(&tv.value).to_string()),
                    MndpType::Ipv4Address => neighbor.ipv4_address::<[u8; 4]>(tv.value.as_ref().try_into().unwrap()),
                    MndpType::Ipv6Address => neighbor.ipv6_address::<[u8; 16]>(tv.value.as_ref().try_into().unwrap()),
                    MndpType::MacAddress => neighbor.mac_address::<[u8; 6]>(tv.value.as_ref().try_into().unwrap()),
                    MndpType::Platform => neighbor.platform(String::from_utf8_lossy(&tv.value).to_string()),
                    MndpType::SoftwareId => neighbor.software_id(String::from_utf8_lossy(&tv.value).to_string()),
                    MndpType::Unpack => match tv.value[0] {
                        0 => neighbor.unpack(Unpack::No),
                        1 => neighbor.unpack(Unpack::Simple),
                        // ?? => neighbor.unpack(Unpack::UncompressedHeaders), // todo
                        // ?? => neighbor.unpack(Unpack::UncompressedAll), // todo
                        _ => neighbor
                    },
                    MndpType::Uptime => neighbor.uptime(Duration::from_secs(tv.value.as_ref().get_u32_le().into())),
                    MndpType::Version => neighbor.version(String::from_utf8_lossy(&tv.value).to_string())
                };
            }
        }

        neighbor.build()
    }

    /// Create a new `Packet` from a `Neighbor`.
    pub fn from_neighbor(neighbor: &Neighbor) -> Packet {
        let mut packet = Packet::new();

        if let Some(val) = &neighbor.board {
            packet.fields.push(TypeValue { typ: MndpType::Board as u16, value: val.clone().into() });
        }

        if let Some(val) = &neighbor.identity {
            packet.fields.push(TypeValue { typ: MndpType::Identity as u16, value: val.clone().into() });
        }

        if let Some(val) = &neighbor.interface_name {
            packet.fields.push(TypeValue { typ: MndpType::InterfaceName as u16, value: val.clone().into() });
        }

        if let Some(val) = &neighbor.ipv4_address {
            packet.fields.push(TypeValue { typ: MndpType::Ipv4Address as u16, value: Bytes::copy_from_slice(&val.octets()) });
        }

        if let Some(val) = &neighbor.ipv6_address {
            packet.fields.push(TypeValue { typ: MndpType::Ipv6Address as u16, value: Bytes::copy_from_slice(&val.octets()) });
        }

        if let Some(val) = &neighbor.mac_address {
            packet.fields.push(TypeValue { typ: MndpType::MacAddress as u16, value: Bytes::copy_from_slice(val.as_bytes()) });
        }

        if let Some(val) = &neighbor.platform {
            packet.fields.push(TypeValue { typ: MndpType::Platform as u16, value: val.clone().into() });
        }

        if let Some(val) = &neighbor.software_id {
            packet.fields.push(TypeValue { typ: MndpType::SoftwareId as u16, value: val.clone().into() });
        }

        if let Some(val) = &neighbor.unpack {
            let byte: u8 = match val {
                Unpack::No => 0,
                Unpack::Simple => 1,
                // Unpack::UncompressedHeaders => todo!(), // Protocol research needed
                // Unpack::UncompressedAll => todo!() // Protocol research needed
            };
            packet.fields.push(TypeValue { typ: MndpType::Unpack as u16, value: Bytes::copy_from_slice(&[byte]) });
        }

        if let Some(val) = &neighbor.uptime {
            // Silently ignore the uptime if it won't fit into a u32
            if let Ok(secs) = TryInto::<u32>::try_into(val.as_secs()) {
                packet.fields.push(TypeValue { typ: MndpType::Uptime as u16, value: Bytes::copy_from_slice(&secs.to_le_bytes()) });
            }
        }

        if let Some(val) = &neighbor.version {
            packet.fields.push(TypeValue { typ: MndpType::Version as u16, value: val.clone().into() });
        }

        packet
    }

}

#[test]
fn test_packet_from_bytes() {
    let bytes: Bytes = hex::decode("3cc6000000010006c4ad34bf91110005000b656f622d726f75746572310007000f362e34382e312028737461626c6529000800084d696b726f54696b000a000441752e00000b0009324150372d5a564335000c00085242373630694753000e000101000f001026006c50067f7700000000000000000100100007766c616e31353700110004ac129d01").unwrap().into();
    let packet = Packet::from_bytes(bytes.clone()).unwrap();
    let res: Bytes = packet.clone().to_bytes();
    assert_eq!(bytes, res);
}

#[test]
fn test_mndp_type_try_into() {
    use strum::IntoEnumIterator;
    for mndp_type in MndpType::iter() {
        let a = mndp_type as u16;
        let b: MndpType = a.try_into().expect(format!("TryInto<u16> not implemented for {:?}", mndp_type).as_str());
        assert_eq!(mndp_type, b);
    }
}



