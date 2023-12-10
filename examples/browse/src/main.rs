use std::{io::stdin, sync::{Mutex, Arc}};

#[tokio::main]
async fn main() {
    println!("Starting browser...");

    let map = Arc::new(Mutex::new(std::collections::HashMap::new()));
    let mut accumulator = 1;

    let map_capture = map.clone();
    let _browser = qmulti::browse_services("test-service", qmulti::Protocol::Tcp, move |state| {
        let map = map_capture.clone();
        match state {
            qmulti::ServiceState::Found(service) => {
                println!("({}) Found {:?} ({:?})", accumulator, service.info().name(), service.info().domain());
                map.lock().unwrap().insert(accumulator, service);

                accumulator += 1;
            }
            qmulti::ServiceState::Lost(service) => {
                let mut map = map.lock().unwrap();
                let index = *map.iter().find(|&(_, v)| v.info() == service.info()).unwrap().0;

                println!("({}) Lost {:?} ({:?})", index, service.info().name(), service.info().domain());

                map.remove(&index);
            },
            qmulti::ServiceState::Error(code) => {
                println!("An error occured while browsing: {:?}", code);
            }
        }
    }).unwrap();
    println!("Started browsing. Hit enter to stop");

    let mut input = String::new();
    loop {
        stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            break;
        }
        if let Ok(num) = input.trim().parse::<u32>() {
            if let Some(service) = map.lock().unwrap().get(&num) {
                println!("Resolving #{}...", num);
                let resolved = service.resolve().await;
                println!("Resolved: {:?}", resolved);
            }
        }

        input.clear();
    }

    println!("Stopping...");
}