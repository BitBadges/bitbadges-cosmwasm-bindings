use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CosmosMsg;
use cosmwasm_std::CustomMsg;

// implement custom msg
impl CustomMsg for BitBadgesMsg {}

// this is a helper to be able to return these as CosmosMsg easier
impl From<BitBadgesMsg> for CosmosMsg<BitBadgesMsg> {
  fn from(original: BitBadgesMsg) -> Self {
      CosmosMsg::Custom(original)
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum BitBadgesMsg {
  #[serde(rename_all = "camelCase")]
  DeleteCollectionMsg {
      //As a string
      collection_id: String
  },

  #[serde(rename_all = "camelCase")]
  CreateAddressListsMsg {
    address_lists: Vec<AddressList>,
  },

  #[serde(rename_all = "camelCase")]
  TransferBadgesMsg {
    collection_id: String,
    transfers: Vec<Transfer>,
  },

  #[serde(rename_all = "camelCase")]
  CreateCollectionMsg {
    balances_type: String,
    default_balances: UserBalanceStore,
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
  },


  #[serde(rename_all = "camelCase")]
  UpdateCollectionMsg {
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
  },

  #[serde(rename_all = "camelCase")]
  UniversalUpdateCollectionMsg {
    collection_id: String,
    balances_type: String,
    default_balances: UserBalanceStore,
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
  }
}

pub fn delete_collection_msg(
    collection_id: String,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::DeleteCollectionMsg {
        collection_id,
    }.into()
}

pub fn address_lists_msg(
  address_lists: Vec<AddressList>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::CreateAddressListsMsg {
    address_lists,
  }.into()
}

pub fn transfer_badges_msg(
  collection_id: String,
  transfers: Vec<Transfer>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::TransferBadgesMsg {
    collection_id,
    transfers,
    }.into()
}

pub fn create_collection_msg(
  balances_type: String,
  default_balances: UserBalanceStore,
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
  cosmos_coin_wrapper_paths_to_add: Vec<CosmosCoinWrapperPathAddObject>
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::CreateCollectionMsg { 
    balances_type,
    default_balances: default_balances,
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
  }.into()
}
  
pub fn update_collection_msg(
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
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::UpdateCollectionMsg {
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
  }
  .into()
}

pub fn universal_update_collection_msg(
  collection_id: String,
  balances_type: String,
  default_balances: UserBalanceStore,
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
  cosmos_coin_wrapper_paths_to_add: Vec<CosmosCoinWrapperPathAddObject>
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::UniversalUpdateCollectionMsg {
    collection_id,
    balances_type,
    default_balances: default_balances,
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
      cosmos_coin_wrapper_paths_to_add
  }
  .into()
}

pub fn update_user_approvals_msg(
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
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::UpdateUserApprovalsMsg {
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
  }
  .into()
}




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddressList {
    pub list_id: String,
    pub addresses: Vec<String>,
    pub whitelist: bool,
    pub uri: String,
    pub custom_data: String,
    pub created_by: Option<String>,
    pub alias_address: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub from: String,
    pub to_addresses: Vec<String>,
    pub balances: Vec<Balance>,
    pub precalculate_balances_from_approval: Option<ApprovalIdentifierDetails>,
    pub merkle_proofs: Vec<MerkleProof>,
    pub memo: String,
    pub prioritized_approvals: Vec<ApprovalIdentifierDetails>,
    pub only_check_prioritized_collection_approvals: bool,
    pub only_check_prioritized_incoming_approvals: bool,
    pub only_check_prioritized_outgoing_approvals: bool,
    pub precalculation_options: PrecalculationOptions,
    pub affiliate_address: String,
    pub num_attempts: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PrecalculationOptions {
    pub override_timestamp: String,
    pub badge_ids_override: Vec<UintRange>,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub amount: String,
    pub ownership_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UintRange {
    pub start: String,
    pub end: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalIdentifierDetails {
    pub approval_id: String,
    pub approval_level: String,
    pub approver_address: String,
    pub version: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MerkleProof {
    pub leaf: String,
    pub aunts: Vec<MerklePathItem>,
    pub leaf_signature: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MerklePathItem {
    pub aunt: String,
    pub on_right: bool,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BadgeMetadata {
    pub uri: String,
    pub custom_data: String,
    pub badge_ids: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CollectionMetadata {
    pub uri: String,
    pub custom_data: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OffChainBalancesMetadata {
    pub uri: String,
    pub custom_data: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CollectionMetadataTimeline {
    pub collection_metadata: CollectionMetadata,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BadgeMetadataTimeline {
    pub badge_metadata: Vec<BadgeMetadata>,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OffChainBalancesMetadataTimeline {
    pub off_chain_balances_metadata: OffChainBalancesMetadata,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CustomDataTimeline {
    pub custom_data: String,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ManagerTimeline {
    pub manager: String,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IsArchivedTimeline {
    pub is_archived: bool,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContractAddressTimeline {
    pub contract_address: String,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StandardsTimeline {
    pub standards: Vec<String>,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ManualBalances {
    pub balances: Vec<Balance>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IncrementedBalances {
    pub start_balances: Vec<Balance>,
    pub increment_badge_ids_by: String,
    pub increment_ownership_times_by: String,
    pub duration_from_timestamp: String,
    pub allow_override_timestamp: bool,
    pub recurring_ownership_times: RecurringOwnershipTimes,
    pub allow_override_with_any_valid_badge: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RecurringOwnershipTimes {
    pub start_time: String,
    pub interval_length: String,
    pub charge_period_length: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PredeterminedOrderCalculationMethod {
    pub use_overall_num_transfers: bool,
    pub use_per_to_address_num_transfers: bool,
    pub use_per_from_address_num_transfers: bool,
    pub use_per_initiated_by_address_num_transfers: bool,
    pub use_merkle_challenge_leaf_index: bool,
    pub challenge_tracker_id: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PredeterminedBalances {
    pub manual_balances: Vec<ManualBalances>,
    pub incremented_balances: IncrementedBalances,
    pub order_calculation_method: PredeterminedOrderCalculationMethod,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalAmounts {
    pub overall_approval_amount: String,
    pub per_to_address_approval_amount: String,
    pub per_from_address_approval_amount: String,
    pub per_initiated_by_address_approval_amount: String,
    pub amount_tracker_id: String,
    pub reset_time_intervals: ResetTimeIntervals,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MaxNumTransfers {
  pub overall_max_num_transfers: String,
  pub per_to_address_max_num_transfers: String,
  pub per_from_address_max_num_transfers: String,
  pub per_initiated_by_address_max_num_transfers: String,
  pub amount_tracker_id: String,
  pub reset_time_intervals: ResetTimeIntervals,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResetTimeIntervals {
  pub start_time: String,
  pub interval_length: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalTracker {
    pub num_transfers: String,
    pub amounts: Vec<Balance>,
    pub last_updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CosmosCoin {
    pub amount: String,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CoinTransfer {
  pub to: String,
  pub coins: Vec<CosmosCoin>,
  pub override_from_with_approver_address: bool,
  pub override_to_with_initiator: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalCriteria {
  pub merkle_challenges: Vec<MerkleChallenge>,
  pub predetermined_balances: PredeterminedBalances,
  pub approval_amounts: ApprovalAmounts,
  pub max_num_transfers: MaxNumTransfers,
  pub coin_transfers: Vec<CoinTransfer>,
  pub require_to_equals_initiated_by: bool,
  pub require_from_equals_initiated_by: bool,
  pub require_to_does_not_equal_initiated_by: bool,
  pub require_from_does_not_equal_initiated_by: bool,
  pub overrides_from_outgoing_approvals: bool,
  pub overrides_to_incoming_approvals: bool,
  pub auto_deletion_options: AutoDeletionOptions,
  pub user_royalties: UserRoyalties,
  pub must_own_badges: Vec<MustOwnBadges>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OutgoingApprovalCriteria {
  pub merkle_challenges: Vec<MerkleChallenge>,
  pub predetermined_balances: PredeterminedBalances,
  pub approval_amounts: ApprovalAmounts,
  pub max_num_transfers: MaxNumTransfers,
  pub coin_transfers: Vec<CoinTransfer>,
  pub require_to_equals_initiated_by: bool,
  pub require_to_does_not_equal_initiated_by: bool,
  pub auto_deletion_options: AutoDeletionOptions,
  pub must_own_badges: Vec<MustOwnBadges>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IncomingApprovalCriteria {
  pub merkle_challenges: Vec<MerkleChallenge>,
  pub predetermined_balances: PredeterminedBalances,
  pub approval_amounts: ApprovalAmounts,
  pub max_num_transfers: MaxNumTransfers,
  pub coin_transfers: Vec<CoinTransfer>,
  pub require_from_equals_initiated_by: bool,
  pub require_from_does_not_equal_initiated_by: bool,
  pub auto_deletion_options: AutoDeletionOptions,
  pub must_own_badges: Vec<MustOwnBadges>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MerkleChallenge {
    pub root: String,
    pub expected_proof_length: String,
    pub use_creator_address_as_leaf: bool,
    pub max_uses_per_leaf: String,
    pub uri: String,
    pub custom_data: String,
    pub challenge_tracker_id: String,
    pub leaf_signer: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserOutgoingApproval {
    pub to_list_id: String,
    pub initiated_by_list_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub uri: String,
    pub custom_data: String,
    pub approval_id: String,
    pub approval_criteria: OutgoingApprovalCriteria,
    pub version: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserIncomingApproval {
    pub from_list_id: String,
    pub initiated_by_list_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub uri: String,
    pub custom_data: String,
    pub approval_id: String,
    pub approval_criteria: IncomingApprovalCriteria,
    pub version: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CollectionApproval {
    pub from_list_id: String,
    pub to_list_id: String,
    pub initiated_by_list_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub uri: String,
    pub custom_data: String,
    pub approval_id: String,
    pub approval_criteria: ApprovalCriteria,
    pub version: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPermissions {
    pub can_delete_collection: Vec<ActionPermission>,
    pub can_archive_collection: Vec<TimedUpdatePermission>,
    pub can_update_off_chain_balances_metadata: Vec<TimedUpdatePermission>,
    pub can_update_standards: Vec<TimedUpdatePermission>,
    pub can_update_custom_data: Vec<TimedUpdatePermission>,
    pub can_update_manager: Vec<TimedUpdatePermission>,
    pub can_update_collection_metadata: Vec<TimedUpdatePermission>,
    pub can_update_valid_badge_ids: Vec<BadgeIdsActionPermission>,
    pub can_update_badge_metadata: Vec<TimedUpdateWithBadgeIdsPermission>,
    pub can_update_collection_approvals: Vec<CollectionApprovalPermission>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserPermissions {
    pub can_update_outgoing_approvals: Vec<UserOutgoingApprovalPermission>,
    pub can_update_incoming_approvals: Vec<UserIncomingApprovalPermission>,
    pub can_update_auto_approve_self_initiated_outgoing_transfers: Vec<ActionPermission>,
    pub can_update_auto_approve_self_initiated_incoming_transfers: Vec<ActionPermission>,
    pub can_update_auto_approve_all_incoming_transfers: Vec<ActionPermission>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CollectionApprovalPermission {
    pub from_list_id: String,
    pub to_list_id: String,
    pub initiated_by_list_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserOutgoingApprovalPermission {
    pub to_list_id: String,
    pub initiated_by_list_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserIncomingApprovalPermission {
    pub from_list_id: String,
    pub initiated_by_list_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BadgeIdsActionPermission {
    pub badge_ids: Vec<UintRange>,
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ActionPermission {
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TimedUpdatePermission {
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TimedUpdateWithBadgeIdsPermission {
    pub badge_ids: Vec<UintRange>,
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserBalanceStore {
    pub balances: Vec<Balance>,
    pub outgoing_approvals: Vec<UserOutgoingApproval>,
    pub incoming_approvals: Vec<UserIncomingApproval>,
    pub auto_approve_self_initiated_outgoing_transfers: bool,
    pub auto_approve_self_initiated_incoming_transfers: bool,
    pub auto_approve_all_incoming_transfers: bool,
    pub user_permissions: UserPermissions,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BadgeCollection {
  pub collection_id: String,
  pub collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
  pub badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
  pub balances_type: String,
  pub off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
  pub custom_data_timeline: Vec<CustomDataTimeline>,
  pub manager_timeline: Vec<ManagerTimeline>,
  pub collection_permissions: CollectionPermissions,
  pub collection_approvals: Vec<CollectionApproval>,
  pub standards_timeline: Vec<StandardsTimeline>,
  pub is_archived_timeline: Vec<IsArchivedTimeline>,
  pub default_balances: UserBalanceStore,
  pub created_by: String,
  pub valid_badge_ids: Vec<UintRange>,
  pub mint_escrow_address: String,
  pub cosmos_coin_wrapper_paths: Vec<CosmosCoinWrapperPath>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CosmosCoinWrapperPath {
  pub address: String,
  pub denom: String,
  pub balances: Vec<Balance>,
  pub symbol: String,
  pub denom_units: Vec<DenomUnit>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DenomUnit {
  pub decimals: String,
  pub symbol: String,
  pub is_default_display: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AutoDeletionOptions {
  pub after_one_use: bool,
  pub after_overall_max_num_transfers: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRoyalties {
  pub percentage: String,
  pub payout_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MustOwnBadges {
  pub collection_id: String,
  pub amount_range: UintRange,
  pub ownership_times: Vec<UintRange>,
  pub badge_ids: Vec<UintRange>,
  pub override_with_current_time: bool,
  pub must_satisfy_for_all_assets: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CosmosCoinWrapperPathAddObject {
  pub denom: String,
  pub balances: Vec<Balance>,
  pub symbol: String,
  pub denom_units: Vec<DenomUnit>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Params {
  // Add fields as needed based on the proto definition
  // This is a placeholder - you'll need to add the actual fields from the params.proto file
}