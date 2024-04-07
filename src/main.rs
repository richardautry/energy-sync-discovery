use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Error, Record, RecordKind};
use std::{net::IpAddr, time::Duration};
use reqwest;

fn main() {
    // register_service();

    scan_for_service();
}

fn scan_for_service() -> String {
    // current target server ip 192.168.1.197
    // for 0 - 255
    // ping 192.168.1.(i):3000
    // if msg is "welcome to energy sync"
    // return ip address

    let ip_address_start: &str = "http://192.168.1.";
    let port_num: &str = ":3000";
    let mut server_ip = String::new();

    for i in 197..255 {
        let ip_address = format!("{ip_address_start}{i}{port_num}");
        println!("{}", ip_address);
        // TODO: Timeout on reqwest get. I may have to use ClientBuilder to ge this functionality
        match reqwest::blocking::get(&ip_address) {
            Ok(body) => {
                println!("{}", ip_address);
                let response_text = body.text().unwrap();
                println!("{}", response_text);
                if response_text == "Welcome to EnergySync" {
                    server_ip = ip_address;
                    break;
                }
            }
            Err(_) => {}
        }
    }
    
    server_ip
}

fn discover_services() {
    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Browse for a service type.
    let service_type = "_rust._tcp.local.";
    let receiver = mdns.browse(service_type).expect("Couldn't browse");

    // Receive the browse events in sync or async. Here is
    // an example of using a thread. Users can call `receiver.recv_async().await`
    // if running in async environment.
    // std::thread::spawn(move || {
    println!("Starting loop");
    // while let Ok(event) = receiver.recv() {
    //     println!("OK");
    //     match event {
    //         ServiceEvent::ServiceResolved(info) => {
    //             println!("Resolved a new service: {} {} {}", info.get_fullname(), info.get_hostname(), info.get_port());

    //             println!("{:?}", info.get_addresses());
    //         }
    //         other_event => {
    //             println!("Received other event: {:?}", &other_event);
    //         }
    //     }
    // };
    std::thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    println!("Resolved a new service: {} at {:?}", info.get_fullname(), info.get_addresses());
                }
                other_event => {
                    println!("Received other event: {:?}", &other_event);
                }
            }
        }
    });

    std::thread::sleep(std::time::Duration::from_secs(1));
    mdns.shutdown().unwrap();
}

fn register_service() {

    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Create a service info.
    let service_type = "_energy-sync._tcp.local.";
    let instance_name = "my-instance";
    // let host_ipv4 = "192.168.1.12";
    let host_ipv4 = "0.0.0.0";
    // let host_name = "192.168.1.12.local.";
    let host_name = "0.0.0.0.local.";
    // TODO: This basically works, but how to discover service on wifi?
    // It doesn't seem like this is broadcasting on wifi but instead looking at localhost (this computer only)
    let port = 5200;
    let properties = [("property_1", "test"), ("property_2", "1234")];

    let my_service = ServiceInfo::new(
        service_type,
        instance_name,
        host_name,
        host_ipv4,
        port,
        &properties[..],
    ).unwrap();

    // Register with the daemon, which publishes the service.
    mdns.register(my_service).unwrap();
    println!("Finished registering");
}

/// The hostname of the devices we are searching for.
/// Every Chromecast will respond to the service name in this example.
// const SERVICE_NAME: &'static str = "_googlecast._tcp.local";
const SERVICE_NAME: &'static str = "_energy_sync._udp.local.";

// #[async_std::main]
// async fn main() -> Result<(), Error> {
//     // Iterate through responses from each Cast device, asking for new devices every 15s
//     let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15))?.listen();
//     println!("Pinning stream");
//     pin_mut!(stream);

//     println!("Starting loop");
//     while let Some(Ok(response)) = stream.next().await {
//         println!("Starting iteration");
//         let addr = response.records()
//                            .filter_map(self::to_ip_addr)
//                            .next();
        
//         println!("have addr");
//         if let Some(addr) = addr {
//             println!("found cast device at {}", addr);
//         } else {
//             println!("cast device does not advertise address");
//         }
//     }

//     Ok(())
// }

    


// fn to_ip_addr(record: &Record) -> Option<IpAddr> {
//     match record.kind {
//         RecordKind::A(addr) => Some(addr.into()),
//         RecordKind::AAAA(addr) => Some(addr.into()),
//         _ => None,
//     }
// }