use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::Packet;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;

fn main() {
    let interface_name = "wlp6s0";
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
    };
    println!("ok");

    loop {
        match rx.next() {
            Ok(packet) => {
                let ethernet_packet = EthernetPacket::new(packet).unwrap();
                println!("MAC Source: {}", ethernet_packet.get_source());
                println!("MAC Destination: {}", ethernet_packet.get_destination());
                //println!("Packet: {:?}", ethernet_packet.packet());
                //Sprintln!("Payload: {:?}", ethernet_packet.payload());
                println!("EtherType: {}", ethernet_packet.get_ethertype());

                match ethernet_packet.get_ethertype() {
                    pnet::packet::ethernet::EtherTypes::Ipv6 => {
                        if let Some(ipv6_packet) = Ipv6Packet::new(ethernet_packet.payload()) {
                            println!("IPv6 Source: {}", ipv6_packet.get_source());
                            println!("IPv6 Destination: {}", ipv6_packet.get_destination());
                            // Add more IPv6 specific code here...
                        }
                    }
                    pnet::packet::ethernet::EtherTypes::Ipv4 => {
                        if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                            println!("IPv4 Source: {}", ipv4_packet.get_source());
                            println!("IPv4 Destination: {}", ipv4_packet.get_destination());
                            // Add more IPv4 specific code here...
                        }
                    }
                    _ => {
                        println!("Unknown EtherType");
                    }
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
