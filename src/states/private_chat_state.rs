use std::net::Ipv4Addr;

pub struct PrivateChatState {
    pub connected_user_ip: Option<Ipv4Addr>,
}

impl PrivateChatState {
    pub fn new() -> Self {
        Self {
            connected_user_ip: None,
        }
    }
}
