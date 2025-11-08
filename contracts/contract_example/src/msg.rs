use bitbadges_cosmwasm::{
  AddressList, Transfer,
  Balance, CollectionPermissions, ManagerTimeline, CollectionMetadataTimeline, 
  TokenMetadataTimeline, CustomDataTimeline,
  CollectionApproval, StandardsTimeline, IsArchivedTimeline, UserBalanceStore, 
  UintRange, UserOutgoingApproval, UserIncomingApproval, UserPermissions, 
  CosmosCoin, CosmosCoinWrapperPathAddObject, TokenIdsActionPermission, 
  TimedUpdatePermission, TimedUpdateWithTokenIdsPermission, CollectionApprovalPermission,
  CollectionInvariants
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}


/**
 * ExecuteMsg can be considered as the API entrypoint for the contract.
 * 
 * By specifying a specific ExecuteMsg with the core contract, you can execute the contract's logic
 * for that message type.
 * 
 * Note: The contract will call into the core Msg types by specifying all the parameters.
 * However, you often do not need all parameters in the API entrypoint like seen below. 
 * 
 * For example, you might have all the CreateCollectionMsg parameters hardcoded in your contract,
 * and the entrypoint ExecuteMsg doesn't specify any parameters.
 * 
 * Alternative Note: These can be whatever you want. They do not need to match the core Msg types.
 * This is your custom logic. Define it how you want.
 */
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ExecuteMsg {
    #[serde(rename_all = "camelCase")]
    DeleteCollectionMsg {
        collection_id: String,
    },

    #[serde(rename_all = "camelCase")]
    CreateAddressListsMsg {
      address_lists: Vec<AddressList>,
    },

    #[serde(rename_all = "camelCase")]
    TransferTokensMsg {
        collection_id: String ,
        transfers: Vec<Transfer>,
    },

    #[serde(rename_all = "camelCase")]
    CreateCollectionMsg {
      default_balances: UserBalanceStore,
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
      invariants: CollectionInvariants,
    },

    #[serde(rename_all = "camelCase")]
    UpdateCollectionMsg {
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
    },

    #[serde(rename_all = "camelCase")]
    UniversalUpdateCollectionMsg {
      collection_id: String,
      default_balances: UserBalanceStore,
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
      invariants: CollectionInvariants,
    },

    #[serde(rename_all = "camelCase")]
    UpdateUserApprovalsMsg {
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
    },

    #[serde(rename_all = "camelCase")]
    SetValidTokenIdsMsg {
      collection_id: String,
      valid_token_ids: Vec<UintRange>,
      can_update_valid_token_ids: Vec<TokenIdsActionPermission>,
    },

    #[serde(rename_all = "camelCase")]
    SetManagerMsg {
      collection_id: String,
      manager_timeline: Vec<ManagerTimeline>,
      can_update_manager: Vec<TimedUpdatePermission>,
    },

    #[serde(rename_all = "camelCase")]
    SetCollectionMetadataMsg {
      collection_id: String,
      collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
      can_update_collection_metadata: Vec<TimedUpdatePermission>,
    },

    #[serde(rename_all = "camelCase")]
    SetTokenMetadataMsg {
      collection_id: String,
      token_metadata_timeline: Vec<TokenMetadataTimeline>,
      can_update_token_metadata: Vec<TimedUpdateWithTokenIdsPermission>,
    },

    #[serde(rename_all = "camelCase")]
    SetCustomDataMsg {
      collection_id: String,
      custom_data_timeline: Vec<CustomDataTimeline>,
      can_update_custom_data: Vec<TimedUpdatePermission>,
    },

    #[serde(rename_all = "camelCase")]
    SetStandardsMsg {
      collection_id: String,
      standards_timeline: Vec<StandardsTimeline>,
      can_update_standards: Vec<TimedUpdatePermission>,
    },

    #[serde(rename_all = "camelCase")]
    SetCollectionApprovalsMsg {
      collection_id: String,
      collection_approvals: Vec<CollectionApproval>,
      can_update_collection_approvals: Vec<CollectionApprovalPermission>,
    },

    #[serde(rename_all = "camelCase")]
    SetIsArchivedMsg {
      collection_id: String,
      is_archived_timeline: Vec<IsArchivedTimeline>,
      can_archive_collection: Vec<TimedUpdatePermission>,
    },

    // Add other messages here as needed
    // Same format as above
}
