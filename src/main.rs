use mdns_sd::{ServiceDaemon, ServiceEvent};
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Error, Record, RecordKind};
use std::{net::IpAddr, time::Duration};

fn main() {
    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Browse for a service type.
    // TODO: How to get this to browse wifi network and not local network?
    let service_type = "_energy-sync._udp.local.";
    let receiver = mdns.browse(service_type).expect("Couldn't browse");

    // Receive the browse events in sync or async. Here is
    // an example of using a thread. Users can call `receiver.recv_async().await`
    // if running in async environment.
    // std::thread::spawn(move || {
    println!("Starting loop");
    while let Ok(event) = receiver.recv() {
        println!("OK");
        match event {
            ServiceEvent::ServiceResolved(info) => {
                println!("Resolved a new service: {} {} {}", info.get_fullname(), info.get_hostname(), info.get_port());

                println!("{:?}", info.get_addresses());
            }
            other_event => {
                println!("Received other event: {:?}", &other_event);
            }
        }
    };
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