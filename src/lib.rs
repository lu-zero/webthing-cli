use anyhow::{anyhow, Result};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use reqwest::blocking;
use serde_json::Value;

#[derive(Debug)]
pub struct Thing {
    pub title: String,
    pub attype: String,
}

fn get_thing(info: ServiceInfo) -> Result<Thing> {
    let host = info.get_addresses().iter().next().unwrap();
    let port = info.get_port();

    let v: Value = blocking::get(format!("http://{}:{}", host, port))?.json()?;

    let title = v
        .get("title")
        .ok_or_else(|| anyhow!("Cannot find name"))?
        .to_string();
    let attype = v
        .get("@type")
        .ok_or_else(|| anyhow!("Cannot find @type"))?
        .to_string();

    Ok(Thing { title, attype })
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
