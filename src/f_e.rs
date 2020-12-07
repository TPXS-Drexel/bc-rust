use regex::Regex;
// get "mine new block" request from fontend
pub fn get_request_content(text: &str) -> &str {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(GET /convert_bc(.*) HTTP)").unwrap();
    }
    match RE.captures(text) {
        Some(caps) => {
            let end_pos = &caps[0].len() - 5;
            return &caps.get(0).unwrap().as_str()[18..end_pos];
        }
        None => {
            return text;
        }
    }
}

// get block id from fontend
pub fn get_mine_id(text: &str) -> u64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(GET /minebc\*[0-9]*__)").unwrap();
    }
    match RE.captures(text) {
        Some(caps) => {
            let end_pos = &caps[0].len() - 2;
            let digits = &caps.get(0).unwrap().as_str()[12..end_pos];
            return digits.parse::<u64>().unwrap();
        }
        None => {
            return 0;
        }
    }
}

// get block's new content from fontend
pub fn get_mine_content(text: &str) -> &str {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"__(.*) HTTP").unwrap();
    }
    match RE.captures(text) {
        Some(caps) => {
            let end_pos = &caps[0].len() - 5;
            return &caps.get(0).unwrap().as_str()[2..end_pos];
        }
        None => {
            return text;
        }
    }
}

// get max_nonce value from fontend's preference
pub fn get_max_nonce(text: &str) -> u64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(GET /setting_bc\*[0-9]*__)").unwrap();
    }
    match RE.captures(text) {
        Some(caps) => {
            let end_pos = &caps[0].len() - 2;
            let digits =  &caps.get(0).unwrap().as_str()[16..end_pos];
            return digits.parse::<u64>().unwrap();
        }
        None => {
            return 1_000_000;
        }
    }
}

// get leading value from fontend's preference
pub fn get_leading_zeros(text: &str) -> u64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"__[0-9]* HTTP").unwrap();
    }
    match RE.captures(text) {
        Some(caps) => {
            let end_pos = &caps[0].len() - 5;
            return caps.get(0).unwrap().as_str()[2..end_pos].parse::<u64>().unwrap();
        }
        None => {
            return 3;
        }
    }
}