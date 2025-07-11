use bitbadges_cosmwasm::{
  delete_collection_msg, BitBadgesMsg, AddressList, address_lists_msg, Transfer, transfer_badges_msg,
  Balance, CollectionPermissions, ManagerTimeline, CollectionMetadataTimeline, BadgeMetadataTimeline, 
  OffChainBalancesMetadataTimeline, CustomDataTimeline, CollectionApproval, StandardsTimeline, IsArchivedTimeline,
  create_collection_msg, update_collection_msg, universal_update_collection_msg, UserBalanceStore,
  UintRange, update_user_approvals_msg, UserOutgoingApproval, UserIncomingApproval, UserPermissions,
  CosmosCoin, CosmosCoinWrapperPathAddObject,
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
        ExecuteMsg::TransferBadgesMsg { collection_id, transfers } => {
          execute_msg_transfer_badges(collection_id, transfers)
        }
        ExecuteMsg::CreateCollectionMsg { balances_type, default_balances, valid_badge_ids, collection_permissions, manager_timeline, collection_metadata_timeline, badge_metadata_timeline, off_chain_balances_metadata_timeline, custom_data_timeline, collection_approvals, standards_timeline, is_archived_timeline, mint_escrow_coins_to_transfer, cosmos_coin_wrapper_paths_to_add } => {
          execute_msg_create_collection(balances_type, default_balances, valid_badge_ids, collection_permissions, manager_timeline, collection_metadata_timeline, badge_metadata_timeline, off_chain_balances_metadata_timeline, custom_data_timeline, collection_approvals, standards_timeline, is_archived_timeline, mint_escrow_coins_to_transfer, cosmos_coin_wrapper_paths_to_add)
        }   
        ExecuteMsg::UpdateCollectionMsg { collection_id, update_valid_badge_ids, valid_badge_ids, update_collection_permissions, collection_permissions, update_manager_timeline, manager_timeline, update_collection_metadata_timeline, collection_metadata_timeline, update_badge_metadata_timeline, badge_metadata_timeline, update_off_chain_balances_metadata_timeline, off_chain_balances_metadata_timeline, update_custom_data_timeline, custom_data_timeline, update_collection_approvals, collection_approvals, update_standards_timeline, standards_timeline, update_is_archived_timeline, is_archived_timeline, mint_escrow_coins_to_transfer, cosmos_coin_wrapper_paths_to_add } => {
          execute_msg_update_collection(collection_id, update_valid_badge_ids, valid_badge_ids, update_collection_permissions, collection_permissions, update_manager_timeline, manager_timeline, update_collection_metadata_timeline, collection_metadata_timeline, update_badge_metadata_timeline, badge_metadata_timeline, update_off_chain_balances_metadata_timeline, off_chain_balances_metadata_timeline, update_custom_data_timeline, custom_data_timeline, update_collection_approvals, collection_approvals, update_standards_timeline, standards_timeline, update_is_archived_timeline, is_archived_timeline, mint_escrow_coins_to_transfer, cosmos_coin_wrapper_paths_to_add)
        }
        ExecuteMsg::UniversalUpdateCollectionMsg { collection_id, balances_type, default_balances, update_valid_badge_ids, valid_badge_ids, update_collection_permissions, collection_permissions, update_manager_timeline, manager_timeline, update_collection_metadata_timeline, collection_metadata_timeline, update_badge_metadata_timeline, badge_metadata_timeline, update_off_chain_balances_metadata_timeline, off_chain_balances_metadata_timeline, update_custom_data_timeline, custom_data_timeline, update_collection_approvals, collection_approvals, update_standards_timeline, standards_timeline, update_is_archived_timeline, is_archived_timeline, mint_escrow_coins_to_transfer, cosmos_coin_wrapper_paths_to_add } => {
          execute_msg_universal_update_collection(collection_id, balances_type, default_balances, update_valid_badge_ids, valid_badge_ids, update_collection_permissions, collection_permissions, update_manager_timeline, manager_timeline, update_collection_metadata_timeline, collection_metadata_timeline, update_badge_metadata_timeline, badge_metadata_timeline, update_off_chain_balances_metadata_timeline, off_chain_balances_metadata_timeline, update_custom_data_timeline, custom_data_timeline, update_collection_approvals, collection_approvals, update_standards_timeline, standards_timeline, update_is_archived_timeline, is_archived_timeline, mint_escrow_coins_to_transfer, cosmos_coin_wrapper_paths_to_add)
        }
        ExecuteMsg::UpdateUserApprovalsMsg { collection_id, update_outgoing_approvals, outgoing_approvals, update_incoming_approvals, incoming_approvals, update_auto_approve_self_initiated_outgoing_transfers, auto_approve_self_initiated_outgoing_transfers, update_auto_approve_self_initiated_incoming_transfers, auto_approve_self_initiated_incoming_transfers, update_auto_approve_all_incoming_transfers, auto_approve_all_incoming_transfers, update_user_permissions, user_permissions } => {
          execute_msg_update_user_approvals(collection_id, update_outgoing_approvals, outgoing_approvals, update_incoming_approvals, incoming_approvals, update_auto_approve_self_initiated_outgoing_transfers, auto_approve_self_initiated_outgoing_transfers, update_auto_approve_self_initiated_incoming_transfers, auto_approve_self_initiated_incoming_transfers, update_auto_approve_all_incoming_transfers, auto_approve_all_incoming_transfers, update_user_permissions, user_permissions)
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

pub fn execute_msg_transfer_badges(
    collection_id: String,
    transfers: Vec<Transfer>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = transfer_badges_msg(collection_id, transfers);
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_create_collection(
    balances_type: String,
    default_balanes: UserBalanceStore,
    valid_badge_ids: Vec<UintRange>,
    collection_permissions: CollectionPermissions,
    manager_timeline: Vec<ManagerTimeline>,
    collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
    badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
    off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
    custom_data_timeline: Vec<CustomDataTimeline>,
    collection_approvals: Vec<CollectionApproval>,
    standards_timeline: Vec<StandardsTimeline>,
    is_archived_timeline: Vec<IsArchivedTimeline>,
    mint_escrow_coins_to_transfer: Vec<CosmosCoin>,
    cosmos_coin_wrapper_paths_to_add: Vec<CosmosCoinWrapperPathAddObject>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = create_collection_msg(
      balances_type,
      default_balanes,
      valid_badge_ids,
      collection_permissions,
      manager_timeline,
      collection_metadata_timeline,
      badge_metadata_timeline,
      off_chain_balances_metadata_timeline,
      custom_data_timeline,
      collection_approvals,
      standards_timeline,
      is_archived_timeline,
      mint_escrow_coins_to_transfer,
      cosmos_coin_wrapper_paths_to_add,
    );
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_update_collection(
    collection_id: String,
    update_valid_badge_ids: bool,
    valid_badge_ids: Vec<UintRange>,
    update_collection_permissions: bool,
    collection_permissions: CollectionPermissions,
    update_manager_timeline: bool,
    manager_timeline: Vec<ManagerTimeline>,
    update_collection_metadata_timeline: bool,
    collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
    update_badge_metadata_timeline: bool,
    badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
    update_off_chain_balances_metadata_timeline: bool,
    off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
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
        update_valid_badge_ids,
        valid_badge_ids,
        update_collection_permissions,
        collection_permissions,
        update_manager_timeline,
        manager_timeline,
        update_collection_metadata_timeline,
        collection_metadata_timeline,
        update_badge_metadata_timeline,
        badge_metadata_timeline,
        update_off_chain_balances_metadata_timeline,
        off_chain_balances_metadata_timeline,
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
    balances_type: String,
    default_balanes: UserBalanceStore,
    update_valid_badge_ids: bool, 
    valid_badge_ids: Vec<UintRange>,
    update_collection_permissions: bool,
    collection_permissions: CollectionPermissions,
    update_manager_timeline: bool,
    manager_timeline: Vec<ManagerTimeline>,
    update_collection_metadata_timeline: bool,
    collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
    update_badge_metadata_timeline: bool,
    badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
    update_off_chain_balances_metadata_timeline: bool,
    off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
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
    let msg = universal_update_collection_msg(
        collection_id,
        balances_type,
        default_balanes,
        update_valid_badge_ids,
        valid_badge_ids,
        update_collection_permissions,
        collection_permissions,
        update_manager_timeline,
        manager_timeline,
        update_collection_metadata_timeline,
        collection_metadata_timeline,
        update_badge_metadata_timeline,
        badge_metadata_timeline,
        update_off_chain_balances_metadata_timeline,
        off_chain_balances_metadata_timeline,
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
