use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use reqwest::blocking;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Thing {
    pub title: String,
    #[serde(rename = "@type")]
    pub attype: Vec<String>,
}

fn get_thing(info: ServiceInfo) -> Result<Thing> {
    let host = info.get_addresses().iter().next().unwrap();
    let port = info.get_port();

    let r = blocking::get(format!("http://{}:{}", host, port))?;

    let t = r.json()?;

    Ok(t)
}

pub fn get_things(mut limit: usize) -> Result<Vec<Thing>> {
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    let service_type = "_webthing._tcp.local.";
    let receiver = mdns.browse(&service_type).expect("Failed to browse");

    let mut things = Vec::with_capacity(limit);

    while let Ok(event) = receiver.recv() {
        if let ServiceEvent::ServiceResolved(info) = event {
            let thing = get_thing(info)?;
            things.push(thing);
            if limit <= 1 {
                break;
            } else {
                limit -= 1;
            }
        }
    }
    Ok(things)
}
