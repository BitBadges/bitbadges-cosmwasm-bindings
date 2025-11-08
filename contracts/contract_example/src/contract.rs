use bitbadges_cosmwasm::{
  delete_collection_msg, BitBadgesMsg, AddressList, address_lists_msg, Transfer, transfer_tokens_msg,
  Balance, CollectionPermissions, ManagerTimeline, CollectionMetadataTimeline, TokenMetadataTimeline, 
  CustomDataTimeline, CollectionApproval, StandardsTimeline, IsArchivedTimeline,
  create_collection_msg, update_collection_msg, universal_update_collection_msg, UserBalanceStore,
  UintRange, update_user_approvals_msg, UserOutgoingApproval, UserIncomingApproval, UserPermissions,
  CosmosCoin, CosmosCoinWrapperPathAddObject, set_valid_token_ids_msg, set_manager_msg, 
  set_collection_metadata_msg, set_token_metadata_msg, set_custom_data_msg, set_standards_msg,
  set_collection_approvals_msg, set_is_archived_msg,
};

use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response, StdError, StdResult, SubMsg, Reply,
    SubMsgResult,
};

use crate::msg::{ExecuteMsg, InstantiateMsg};

// Add new constant for reply ID
const BADGES_REPLY_ID: u64 = 1;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}


#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<BitBadgesMsg>, StdError> {
    /**
     * The x/badges message will always be executed as the contract. 
     * 
     * Please keep this in mind when developing your contract. All messages will be executed as the contract address.
     * To work around this, you must manage everything as the contract address.
     *    Ex: Create as contract + set manager as info.sender, etc -> User now manages the collection
     *    Ex: User approves contract in their approvals -> contract can execute on behalf of the user.
     */
    // let contract_address = _env.contract.address.to_string();
    // let sender = info.sender.to_string();


    /**
     * Here, we execute the contract's logic for the specific message type.
     * 
     * You match your defined ExecuteMsg with their logic.
     * Within the logic of each ExecuteMsg, you can call into the core x/badges messages
     * using the add_submessage function.
     * 
     * Ok(Response::new().add_submessage(SubMsg::reply_always(msg, BADGES_REPLY_ID)))
     */
    match msg {
        ExecuteMsg::DeleteCollectionMsg { collection_id } => {
          execute_msg_delete_collection(collection_id)
        }
        ExecuteMsg::CreateAddressListsMsg { address_lists } => {
          execute_msg_create_address_lists(address_lists)
        }
        ExecuteMsg::TransferTokensMsg { collection_id, transfers } => {
          execute_msg_transfer_tokens(collection_id, transfers)
        }
        ExecuteMsg::CreateCollectionMsg { default_balances, valid_token_ids, collection_permissions, manager_timeline, collection_metadata_timeline, token_metadata_timeline, custom_data_timeline, collection_approvals, standards_timeline, is_archived_timeline, mint_escrow_coins_to_transfer, cosmos_coin_wrapper_paths_to_add, invariants } => {
          execute_msg_create_collection(default_balances, valid_token_ids, collection_permissions, manager_timeline, collection_metadata_timeline, token_metadata_timeline, custom_data_timeline, collection_approvals, standards_timeline, is_archived_timeline, mint_escrow_coins_to_transfer, cosmos_coin_wrapper_paths_to_add, invariants)
        }   
        ExecuteMsg::UpdateCollectionMsg { collection_id, update_valid_token_ids, valid_token_ids, update_collection_permissions, collection_permissions, update_manager_timeline, manager_timeline, update_collection_metadata_timeline, collection_metadata_timeline, update_token_metadata_timeline, token_metadata_timeline, update_custom_data_timeline, custom_data_timeline, update_collection_approvals, collection_approvals, update_standards_timeline, standards_timeline, update_is_archived_timeline, is_archived_timeline, mint_escrow_coins_to_transfer, cosmos_coin_wrapper_paths_to_add } => {
          execute_msg_update_collection(collection_id, update_valid_token_ids, valid_token_ids, update_collection_permissions, collection_permissions, update_manager_timeline, manager_timeline, update_collection_metadata_timeline, collection_metadata_timeline, update_token_metadata_timeline, token_metadata_timeline, update_custom_data_timeline, custom_data_timeline, update_collection_approvals, collection_approvals, update_standards_timeline, standards_timeline, update_is_archived_timeline, is_archived_timeline, mint_escrow_coins_to_transfer, cosmos_coin_wrapper_paths_to_add)
        }
        ExecuteMsg::UniversalUpdateCollectionMsg { collection_id, default_balances, update_valid_token_ids, valid_token_ids, update_collection_permissions, collection_permissions, update_manager_timeline, manager_timeline, update_collection_metadata_timeline, collection_metadata_timeline, update_token_metadata_timeline, token_metadata_timeline, update_custom_data_timeline, custom_data_timeline, update_collection_approvals, collection_approvals, update_standards_timeline, standards_timeline, update_is_archived_timeline, is_archived_timeline, mint_escrow_coins_to_transfer, cosmos_coin_wrapper_paths_to_add, invariants } => {
          execute_msg_universal_update_collection(collection_id, default_balances, update_valid_token_ids, valid_token_ids, update_collection_permissions, collection_permissions, update_manager_timeline, manager_timeline, update_collection_metadata_timeline, collection_metadata_timeline, update_token_metadata_timeline, token_metadata_timeline, update_custom_data_timeline, custom_data_timeline, update_collection_approvals, collection_approvals, update_standards_timeline, standards_timeline, update_is_archived_timeline, is_archived_timeline, mint_escrow_coins_to_transfer, cosmos_coin_wrapper_paths_to_add, invariants)
        }
        ExecuteMsg::UpdateUserApprovalsMsg { collection_id, update_outgoing_approvals, outgoing_approvals, update_incoming_approvals, incoming_approvals, update_auto_approve_self_initiated_outgoing_transfers, auto_approve_self_initiated_outgoing_transfers, update_auto_approve_self_initiated_incoming_transfers, auto_approve_self_initiated_incoming_transfers, update_auto_approve_all_incoming_transfers, auto_approve_all_incoming_transfers, update_user_permissions, user_permissions } => {
          execute_msg_update_user_approvals(collection_id, update_outgoing_approvals, outgoing_approvals, update_incoming_approvals, incoming_approvals, update_auto_approve_self_initiated_outgoing_transfers, auto_approve_self_initiated_outgoing_transfers, update_auto_approve_self_initiated_incoming_transfers, auto_approve_self_initiated_incoming_transfers, update_auto_approve_all_incoming_transfers, auto_approve_all_incoming_transfers, update_user_permissions, user_permissions)
        }
        ExecuteMsg::SetValidTokenIdsMsg { collection_id, valid_token_ids, can_update_valid_token_ids } => {
          execute_msg_set_valid_token_ids(collection_id, valid_token_ids, can_update_valid_token_ids)
        }
        ExecuteMsg::SetManagerMsg { collection_id, manager_timeline, can_update_manager } => {
          execute_msg_set_manager(collection_id, manager_timeline, can_update_manager)
        }
        ExecuteMsg::SetCollectionMetadataMsg { collection_id, collection_metadata_timeline, can_update_collection_metadata } => {
          execute_msg_set_collection_metadata(collection_id, collection_metadata_timeline, can_update_collection_metadata)
        }
        ExecuteMsg::SetTokenMetadataMsg { collection_id, token_metadata_timeline, can_update_token_metadata } => {
          execute_msg_set_token_metadata(collection_id, token_metadata_timeline, can_update_token_metadata)
        }
        ExecuteMsg::SetCustomDataMsg { collection_id, custom_data_timeline, can_update_custom_data } => {
          execute_msg_set_custom_data(collection_id, custom_data_timeline, can_update_custom_data)
        }
        ExecuteMsg::SetStandardsMsg { collection_id, standards_timeline, can_update_standards } => {
          execute_msg_set_standards(collection_id, standards_timeline, can_update_standards)
        }
        ExecuteMsg::SetCollectionApprovalsMsg { collection_id, collection_approvals, can_update_collection_approvals } => {
          execute_msg_set_collection_approvals(collection_id, collection_approvals, can_update_collection_approvals)
        }
        ExecuteMsg::SetIsArchivedMsg { collection_id, is_archived_timeline, can_archive_collection } => {
          execute_msg_set_is_archived(collection_id, is_archived_timeline, can_archive_collection)
        }
    }
}


pub fn execute_msg_delete_collection(
    collection_id: String,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = delete_collection_msg(collection_id);
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_create_address_lists(
    address_lists: Vec<AddressList>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = address_lists_msg(address_lists);
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_transfer_tokens(
    collection_id: String,
    transfers: Vec<Transfer>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = transfer_tokens_msg(collection_id, transfers);
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_create_collection(
    default_balanes: UserBalanceStore,
    valid_token_ids: Vec<UintRange>,
    collection_permissions: CollectionPermissions,
    manager_timeline: Vec<ManagerTimeline>,
    collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
    token_metadata_timeline: Vec<TokenMetadataTimeline>,
    custom_data_timeline: Vec<CustomDataTimeline>,
    collection_approvals: Vec<CollectionApproval>,
    standards_timeline: Vec<StandardsTimeline>,
    is_archived_timeline: Vec<IsArchivedTimeline>,
    mint_escrow_coins_to_transfer: Vec<CosmosCoin>,
    cosmos_coin_wrapper_paths_to_add: Vec<CosmosCoinWrapperPathAddObject>,
    invariants: bitbadges_cosmwasm::CollectionInvariants,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = create_collection_msg(
      default_balanes,
      valid_token_ids,
      collection_permissions,
      manager_timeline,
      collection_metadata_timeline,
      token_metadata_timeline,
      custom_data_timeline,
      collection_approvals,
      standards_timeline,
      is_archived_timeline,
      mint_escrow_coins_to_transfer,
      cosmos_coin_wrapper_paths_to_add,
      invariants,
    );
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_update_collection(
    collection_id: String,
    update_valid_token_ids: bool,
    valid_token_ids: Vec<UintRange>,
    update_collection_permissions: bool,
    collection_permissions: CollectionPermissions,
    update_manager_timeline: bool,
    manager_timeline: Vec<ManagerTimeline>,
    update_collection_metadata_timeline: bool,
    collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
    update_token_metadata_timeline: bool,
    token_metadata_timeline: Vec<TokenMetadataTimeline>,
    update_custom_data_timeline: bool,
    custom_data_timeline: Vec<CustomDataTimeline>,
    update_collection_approvals: bool,
    collection_approvals: Vec<CollectionApproval>,
    update_standards_timeline: bool,
    standards_timeline: Vec<StandardsTimeline>,
    update_is_archived_timeline: bool,
    is_archived_timeline: Vec<IsArchivedTimeline>,
    mint_escrow_coins_to_transfer: Vec<CosmosCoin>,
    cosmos_coin_wrapper_paths_to_add: Vec<CosmosCoinWrapperPathAddObject>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = update_collection_msg(
        collection_id,
        update_valid_token_ids,
        valid_token_ids,
        update_collection_permissions,
        collection_permissions,
        update_manager_timeline,
        manager_timeline,
        update_collection_metadata_timeline,
        collection_metadata_timeline,
        update_token_metadata_timeline,
        token_metadata_timeline,
        update_custom_data_timeline,
        custom_data_timeline,
        update_collection_approvals,
        collection_approvals,
        update_standards_timeline,
        standards_timeline,
        update_is_archived_timeline,
        is_archived_timeline,
        mint_escrow_coins_to_transfer,
        cosmos_coin_wrapper_paths_to_add,
    );
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_universal_update_collection(
    collection_id: String,
    default_balanes: UserBalanceStore,
    update_valid_token_ids: bool, 
    valid_token_ids: Vec<UintRange>,
    update_collection_permissions: bool,
    collection_permissions: CollectionPermissions,
    update_manager_timeline: bool,
    manager_timeline: Vec<ManagerTimeline>,
    update_collection_metadata_timeline: bool,
    collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
    update_token_metadata_timeline: bool,
    token_metadata_timeline: Vec<TokenMetadataTimeline>,
    update_custom_data_timeline: bool,
    custom_data_timeline: Vec<CustomDataTimeline>,
    update_collection_approvals: bool,
    collection_approvals: Vec<CollectionApproval>,
    update_standards_timeline: bool,
    standards_timeline: Vec<StandardsTimeline>,
    update_is_archived_timeline: bool,
    is_archived_timeline: Vec<IsArchivedTimeline>,
    mint_escrow_coins_to_transfer: Vec<CosmosCoin>,
    cosmos_coin_wrapper_paths_to_add: Vec<CosmosCoinWrapperPathAddObject>,
    invariants: bitbadges_cosmwasm::CollectionInvariants,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = universal_update_collection_msg(
        collection_id,
        default_balanes,
        update_valid_token_ids,
        valid_token_ids,
        update_collection_permissions,
        collection_permissions,
        update_manager_timeline,
        manager_timeline,
        update_collection_metadata_timeline,
        collection_metadata_timeline,
        update_token_metadata_timeline,
        token_metadata_timeline,
        update_custom_data_timeline,
        custom_data_timeline,
        update_collection_approvals,
        collection_approvals,
        update_standards_timeline,
        standards_timeline,
        update_is_archived_timeline,
        is_archived_timeline,
        mint_escrow_coins_to_transfer,
        cosmos_coin_wrapper_paths_to_add,
        invariants,
    );
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_update_user_approvals(
    collection_id: String,
    update_outgoing_approvals: bool,
    outgoing_approvals: Vec<UserOutgoingApproval>,
    update_incoming_approvals: bool,  
    incoming_approvals: Vec<UserIncomingApproval>,  
    update_auto_approve_self_initiated_outgoing_transfers: bool,  
    auto_approve_self_initiated_outgoing_transfers: bool,  
    update_auto_approve_self_initiated_incoming_transfers: bool,  
    auto_approve_self_initiated_incoming_transfers: bool,  
    update_auto_approve_all_incoming_transfers: bool,
    auto_approve_all_incoming_transfers: bool,
    update_user_permissions: bool,  
    user_permissions: UserPermissions,  
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = update_user_approvals_msg(
        collection_id,
        update_outgoing_approvals,
        outgoing_approvals,
        update_incoming_approvals,
        incoming_approvals,
        update_auto_approve_self_initiated_outgoing_transfers,
        auto_approve_self_initiated_outgoing_transfers,
        update_auto_approve_self_initiated_incoming_transfers,
        auto_approve_self_initiated_incoming_transfers,
        update_auto_approve_all_incoming_transfers,
        auto_approve_all_incoming_transfers,
        update_user_permissions,
        user_permissions,
    );
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_set_valid_token_ids(
    collection_id: String,
    valid_token_ids: Vec<UintRange>,
    can_update_valid_token_ids: Vec<bitbadges_cosmwasm::TokenIdsActionPermission>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = set_valid_token_ids_msg(
        "".to_string(), // creator - will be set by the contract
        collection_id,
        valid_token_ids,
        can_update_valid_token_ids,
    );
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_set_manager(
    collection_id: String,
    manager_timeline: Vec<ManagerTimeline>,
    can_update_manager: Vec<bitbadges_cosmwasm::TimedUpdatePermission>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = set_manager_msg(
        "".to_string(), // creator - will be set by the contract
        collection_id,
        manager_timeline,
        can_update_manager,
    );
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_set_collection_metadata(
    collection_id: String,
    collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
    can_update_collection_metadata: Vec<bitbadges_cosmwasm::TimedUpdatePermission>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = set_collection_metadata_msg(
        "".to_string(), // creator - will be set by the contract
        collection_id,
        collection_metadata_timeline,
        can_update_collection_metadata,
    );
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_set_token_metadata(
    collection_id: String,
    token_metadata_timeline: Vec<TokenMetadataTimeline>,
    can_update_token_metadata: Vec<bitbadges_cosmwasm::TimedUpdateWithTokenIdsPermission>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = set_token_metadata_msg(
        "".to_string(), // creator - will be set by the contract
        collection_id,
        token_metadata_timeline,
        can_update_token_metadata,
    );
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_set_custom_data(
    collection_id: String,
    custom_data_timeline: Vec<CustomDataTimeline>,
    can_update_custom_data: Vec<bitbadges_cosmwasm::TimedUpdatePermission>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = set_custom_data_msg(
        "".to_string(), // creator - will be set by the contract
        collection_id,
        custom_data_timeline,
        can_update_custom_data,
    );
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_set_standards(
    collection_id: String,
    standards_timeline: Vec<StandardsTimeline>,
    can_update_standards: Vec<bitbadges_cosmwasm::TimedUpdatePermission>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = set_standards_msg(
        "".to_string(), // creator - will be set by the contract
        collection_id,
        standards_timeline,
        can_update_standards,
    );
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_set_collection_approvals(
    collection_id: String,
    collection_approvals: Vec<CollectionApproval>,
    can_update_collection_approvals: Vec<bitbadges_cosmwasm::CollectionApprovalPermission>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = set_collection_approvals_msg(
        "".to_string(), // creator - will be set by the contract
        collection_id,
        collection_approvals,
        can_update_collection_approvals,
    );
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_set_is_archived(
    collection_id: String,
    is_archived_timeline: Vec<IsArchivedTimeline>,
    can_archive_collection: Vec<bitbadges_cosmwasm::TimedUpdatePermission>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = set_is_archived_msg(
        "".to_string(), // creator - will be set by the contract
        collection_id,
        is_archived_timeline,
        can_archive_collection,
    );
    Ok(Response::new().add_message(msg))
}
