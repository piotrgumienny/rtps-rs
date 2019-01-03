use crate::structure::locator::LocatorList_t;

/// This message is sent from an RTPS Reader to an RTPS Writer.
/// It contains explicit information on where to send a reply
/// to the Submessages that follow it within the same message.
#[derive(Debug, PartialEq)]
pub struct InfoReply {
    pub unicast_locator_list: LocatorList_t,
    pub multicast_locator_list: LocatorList_t,
}