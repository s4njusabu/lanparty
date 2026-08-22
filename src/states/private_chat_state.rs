use std::net::IpAddr;

pub struct PrivateChatState {
    pub connected_user_ip: Option<IpAddr>,
}

impl PrivateChatState {
    pub fn new() -> Self {
        Self {
            connected_user_ip: None,
        }
    }
}
