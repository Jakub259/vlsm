use std::cmp::Reverse;
use std::env;
use std::net::Ipv4Addr;

fn main() {
    let mut args = env::args();
    // skip program name
    args.next();
    // read ip from argv[1]
    let ip = u32::from(
        args.next()
            .expect("No net address given")
            .parse::<Ipv4Addr>()
            .expect("Invalid ip address"),
    );

    // read prefix from argv[2]
    let prefix = args
        .next()
        .expect("No prefix given")
        .parse::<u8>()
        .expect("Invalid prefix");
    // check if network address is valid
    if ip != ip & u32::MAX << (32 - prefix) {
        panic!("Invalid network address");
    }
    if prefix > 30 {
        panic!("Invalid prefix");
    }

    // read number of hosts from argv[3..]
    let mut subnets_hosts: Vec<u32> = Vec::new();
    for count in args {
        subnets_hosts.push(count.parse::<u32>().unwrap());
    }

    // sort subnets_hosts in descending order
    subnets_hosts.sort_by_key(|&num| Reverse(num));

    print_subnets(&subnets_hosts, ip, prefix);
}

fn calculate_subnet_capacity(mut subnets_hosts: u32) -> u32 {
    // add 2 for broadcast and network address
    subnets_hosts = subnets_hosts + 2;
    // find smallest power of 2 which is greater or equal to num
    subnets_hosts = subnets_hosts - 1;
    subnets_hosts = subnets_hosts.leading_zeros();
    subnets_hosts = 32 - subnets_hosts;
    subnets_hosts = 1u32
        .checked_shl(subnets_hosts)
        .expect("Integer overflow probably too many hosts");
    subnets_hosts
}
fn print_subnets(subnets_hosts: &[u32], ip: u32, prefix: u8) {
    let net_capacity = 2u32
        .checked_pow((32 - prefix).into())
        .expect("Integer overflow probably too many hosts");
    let mut current_capacity = 0u32;

    let mut ip = ip;
    for hosts in subnets_hosts {
        let capacity = calculate_subnet_capacity(*hosts);
        current_capacity += capacity;
        if current_capacity > net_capacity {
            panic!("Too many hosts");
        }
        
        let network_address = Ipv4Addr::from(ip);
        let first_usable_address = Ipv4Addr::from(ip + 1);
        let last_usable_address = Ipv4Addr::from(ip + capacity - 2);
        let broadcast = Ipv4Addr::from(ip + capacity - 1);
        let mask = Ipv4Addr::from(
            u32::MAX
                .checked_shl(capacity.trailing_zeros())
                .expect("Probably too many hosts"),
        );

        println!("SUBNET:");
        print!("mask: {mask} ");
        print!("hosts: {hosts} ");
        println!("capacity: {capacity} ");
        println!("network address: {network_address}");
        if u32::from(first_usable_address) >= u32::from(last_usable_address) {
            println!("NO USABLE ADDRESSES");
        } else {
            println!("first usable address: {}", first_usable_address);
            println!("last usable address: {}", last_usable_address);
        }
        println!("broadcast address: {}\n", broadcast);
        ip = ip + capacity;
    }
}