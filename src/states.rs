use std::{collections::HashMap, sync::Mutex};

use actix_web::web;

pub type IpMap = HashMap<String, usize>;

pub struct IpCounter {
    pub maps: Mutex<IpMap>,
}

impl IpCounter {
    pub fn new() -> web::Data<IpCounter> {
        web::Data::new(IpCounter {
            maps: Mutex::new(HashMap::new()),
        })
    }
}
