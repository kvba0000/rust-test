use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
#[allow(dead_code)]
enum MyIpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}


pub fn ip_init() {
    let my_v4 = MyIpAddr::V4(127, 0, 0, 1);
    let my_v6 = MyIpAddr::V6(String::from("::1"));

    println!("Own implementation:");
    println!("V4 Address: {:#?}", my_v4);
    println!("V6 Address: {:#?}", my_v6);

    let built_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let built_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    println!("Built-in implementation:");
    println!("V4 Address: {:#?}", built_v4);
    println!("V6 Address: {:#?}", built_v6);
}