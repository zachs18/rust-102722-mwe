use std::net::{IpAddr, Ipv4Addr};

fn main() {
    let value = IpAddr::V4(Ipv4Addr::new(31, 41, 59, 26));
    assert_eq!(&value, &value);
}
