use cosmwasm_std::{QuerierWrapper, StdResult};
use crate::query::BitBadgesQuery;

pub struct BitBadgesQuerier<'a> {
    querier: &'a QuerierWrapper<'a, BitBadgesQuery>,
}


impl<'a> BitBadgesQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, BitBadgesQuery>) -> Self {
        BitBadgesQuerier { querier }
    }
    
    pub fn query_collection<T: Into<String>>(&self, collection_id: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryCollection { 
            collection_id
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_balance<T: Into<String>>(&self, collection_id: String, address: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryBalance { 
            collection_id,
            address
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_address_list<T: Into<String>>(&self, list_id: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryAddressList { 
            list_id
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_approvals_tracker<T: Into<String>>(&self, collection_id: String, approval_level: String, approver_address: String, approval_id: String, amount_tracker_id: String, tracker_type: String, approved_address: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryApprovalTracker { 
            collection_id,
            approval_level,
            approval_id,
            approver_address,
            amount_tracker_id,
            tracker_type,
            approved_address
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_num_used_for_merkle_challenge<T: Into<String>>(&self, collection_id: String, approval_level: String, approver_address: String, approval_id: String, challenge_tracker_id: String, leaf_index: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryChallengeTracker { 
            collection_id,
            approval_id,
            approval_level,
            approver_address,
            challenge_tracker_id,
            leaf_index
        }
        .into();
        self.querier.query(&request)
    }

    pub fn query_wrappable_balances<T: Into<String>>(&self, denom: String, address: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryGetWrappableBalances { 
            denom,
            address
        }
        .into();
        self.querier.query(&request)
    }

    pub fn query_is_address_reserved_protocol<T: Into<String>>(&self, address: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryIsAddressReservedProtocol { 
            address
        }
        .into();
        self.querier.query(&request)
    }

    pub fn query_all_reserved_protocol_addresses(&self) -> StdResult<String> {
        let request = BitBadgesQuery::QueryGetAllReservedProtocolAddresses {}
        .into();
        self.querier.query(&request)
    }

    pub fn query_get_vote<T: Into<String>>(&self, collection_id: String, approval_level: String, approver_address: String, approval_id: String, proposal_id: String, voter_address: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryGetVote { 
            collection_id,
            approval_level,
            approver_address,
            approval_id,
            proposal_id,
            voter_address
        }
        .into();
        self.querier.query(&request)
    }

    pub fn query_get_votes<T: Into<String>>(&self, collection_id: String, approval_level: String, approver_address: String, approval_id: String, proposal_id: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryGetVotes { 
            collection_id,
            approval_level,
            approver_address,
            approval_id,
            proposal_id
        }
        .into();
        self.querier.query(&request)
    }
}