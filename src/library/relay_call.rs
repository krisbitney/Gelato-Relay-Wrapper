use std::fmt;
use crate::library::constants::GELATO_RELAY_URL;

pub enum RelayCall {
    CallWithSyncFee,
    CallWithSyncFeeERC2771,
    SponsoredCall,
    SponsoredCallERC2771,
}

impl fmt::Display for RelayCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            RelayCall::CallWithSyncFee => "CallWithSyncFee",
            RelayCall::CallWithSyncFeeERC2771 => "CallWithSyncFeeERC2771",
            RelayCall::SponsoredCall => "SponsoredCall",
            RelayCall::SponsoredCallERC2771 => "SponsoredCallERC2771",
        };
        write!(f, "{}", name)
    }
}

pub fn relay_call_path(relay_call: &RelayCall) -> String {
    match relay_call {
        RelayCall::CallWithSyncFee => format!("{}/relays/v2/call-with-sync-fee", GELATO_RELAY_URL),
        RelayCall::CallWithSyncFeeERC2771 => format!("{}/relays/v2/call-with-sync-fee-erc2771", GELATO_RELAY_URL),
        RelayCall::SponsoredCall => format!("{}/relays/v2/sponsored-call", GELATO_RELAY_URL),
        RelayCall::SponsoredCallERC2771 => format!("{}/relays/v2/sponsored-call-erc2771", GELATO_RELAY_URL),
    }
}