use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::time::Duration;

use macaddr::MacAddr6;
use mndp::neighbor::{Neighbor, Unpack};


fn main() {
    println!("Hello, world!");

    let b = Neighbor::builder()
        .board("hello")
        .identity("world")
        .interface_name("ether2")
        .platform("abc123")
        .unpack(Unpack::Simple)
        .ipv6_enabled(false)
        .mac_address(MacAddr6::from_str("aa:bb:cc:dd:ee:ff").unwrap())
        .software_id("ABC-123")
        .version("3.4.5")
        .ipv6_address(Ipv6Addr::from_str("ff02:b:a::25").unwrap())
        .ipv4_address(Ipv4Addr::from_str("192.168.1.1").unwrap())
        .uptime(Duration::from_secs(450))
        .build();

    println!("{:#?}", b);

    println!("{:?}", b.platform);


}
