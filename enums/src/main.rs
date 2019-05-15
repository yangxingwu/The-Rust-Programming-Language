struct Ipv4Addr {
    addr: u32,
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(String),
}

fn print_addr(addr: IpAddr) {
    // match addr {
    //     IpAddr::V4(ipv4_addr) => {
    //         println!("IPV4 address {}", ipv4_addr.addr);
    //     }
    //     IpAddr::V6(ipv6_addr) => {
    //         println!("IPV6 address {}", ipv6_addr);
    //     }
    // }
}

fn main() {
    let addr1 = IpAddr::V4(Ipv4Addr { addr: 1234 });
    let addr2 = IpAddr::V6(String::from("::1"));
    print_addr(addr1);
    print_addr(addr2);
}
