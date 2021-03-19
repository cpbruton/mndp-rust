use std::net::{Ipv6Addr, Ipv4Addr};
use std::time::Duration;

use macaddr::MacAddr6;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Unpack {
    No,
    Simple,
    // UncompressedHeaders, // Protocol research needed
    // UncompressedAll // Protocol research needed
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Neighbor {
    pub board: Option<String>,
    pub identity: Option<String>,
    pub interface_name: Option<String>,
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_address: Option<Ipv6Addr>,
    pub ipv6_enabled: Option<bool>,
    pub mac_address: Option<MacAddr6>,
    pub platform: Option<String>,
    pub software_id: Option<String>,
    pub unpack: Option<Unpack>,
    pub uptime: Option<Duration>,
    pub version: Option<String>,
    _private: ()
}

impl Neighbor {
    pub fn new() -> Neighbor {
        Default::default()
    }

    pub fn builder() -> Builder {
        Builder::new()
    }

}

/// Builder structure for a `Neighbor`.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Builder {
    inner: Neighbor
}

impl Builder {
    /// Create a new `Builder` to start building a `Neighbor` instance.
    pub fn new() -> Self {
        Builder {
            inner: Neighbor::new()
        }
    }

    /// Set the board for this instance.
    pub fn board<S: Into<String>>(mut self, value: S) -> Self {
        self.inner.board = Some(value.into());
        self
    }

    /// Set the identity for this instance.
    pub fn identity<S: Into<String>>(mut self, value: S) -> Self {
        self.inner.identity = Some(value.into());
        self
    }

    /// Set the interface name for this instance.
    pub fn interface_name<S: Into<String>>(mut self, value: S) -> Self {
        self.inner.interface_name = Some(value.into());
        self
    }

    /// Set the IPv4 address for this instance.
    pub fn ipv4_address<A: Into<Ipv4Addr>>(mut self, value: A) -> Self {
        self.inner.ipv4_address = Some(value.into());
        self
    }

    /// Set the IPv6 address for this instance.
    pub fn ipv6_address<A: Into<Ipv6Addr>>(mut self, value: A) -> Self {
        self.inner.ipv6_address = Some(value.into());
        self
    }

    /// Set the IPv6 enabled flag for this instance.
    pub fn ipv6_enabled<B: Into<bool>>(mut self, value: B) -> Self {
        self.inner.ipv6_enabled = Some(value.into());
        self
    }
    /// Set the MAC address for this instance.
    pub fn mac_address<M: Into<MacAddr6>>(mut self, value: M) -> Self {
        self.inner.mac_address = Some(value.into());
        self
    }

    /// Set the platform name for this instance.
    pub fn platform<S: Into<String>>(mut self, value: S) -> Self {
        self.inner.platform = Some(value.into());
        self
    }

    /// Set the software ID for this instance.
    pub fn software_id<S: Into<String>>(mut self, value: S) -> Self {
        self.inner.software_id = Some(value.into());
        self
    }

    /// Set the unpack (compression type) for this instance.
    pub fn unpack(mut self, value: Unpack) -> Self {
        self.inner.unpack = Some(value);
        self
    }

    /// Set the uptime for this instance.
    pub fn uptime<D: Into<Duration>>(mut self, value: D) -> Self {
        self.inner.uptime = Some(value.into());
        self
    }

    /// Set the version string for this instance.
    pub fn version<S: Into<String>>(mut self, value: S) -> Self {
        self.inner.version = Some(value.into());
        self
    }

    /// Return the finished `Neighbor` instance.
    pub fn build(self) -> Neighbor {
        self.inner
    }
}
