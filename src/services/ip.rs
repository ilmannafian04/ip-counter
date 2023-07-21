use crate::{error::Error, states::IpMap};

pub fn increment(ip_map: &mut IpMap, ip: &str) -> Result<usize, Error> {
    let hits: usize;
    if ip_map.contains_key(ip) {
        hits = ip_map.get(ip).unwrap() + 1;
        ip_map.insert(ip.to_owned(), hits);
    } else {
        hits = 1;
        ip_map.insert(ip.to_owned(), 1);
    };
    Ok(hits)
}
