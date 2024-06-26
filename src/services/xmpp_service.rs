use prose_xmpp::BareJid;

#[derive(Debug, Clone, PartialEq)]
pub enum RoomId {
    User(BareJid),
    Room(BareJid),
}

pub trait XMPPService: Send + Sync {
    fn send_message(&self, to: RoomId, message: String);
}
