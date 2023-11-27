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
      collection_id: String, 
  },
  CreateAddressMappingsMsg {
    address_mappings: Vec<AddressMapping>,
  },
  TransferBadgeMsg {
    collection_id: String,
    transfers: Vec<Transfer>,
  },
  CreateCollectionMsg {
    balances_type: String,
    default_outgoing_approvals: Vec<UserOutgoingApproval>,
    default_incoming_approvals: Vec<UserIncomingApproval>,
    badges_to_create: Vec<Balance>,
    collection_permissions: CollectionPermissions,
    manager_timeline: Vec<ManagerTimeline>,
    collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
    badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
    off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
    custom_data_timeline: Vec<CustomDataTimeline>,
    collection_approvals: Vec<CollectionApproval>,
    standards_timeline: Vec<StandardsTimeline>,
    is_archived_timeline: Vec<IsArchivedTimeline>,
    default_auto_approve_self_initiated_outgoing_transfers: bool,
    default_auto_approve_self_initiated_incoming_transfers: bool,
    default_user_permissions: UserPermissions,
  },
  UpdateCollectionMsg {
    collection_id: String,
    badges_to_create: Vec<Balance>,
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
  },
  UniversalUpdateCollectionMsg {
    collection_id: String,
    balances_type: String,
    default_outgoing_approvals: Vec<UserOutgoingApproval>,
    default_incoming_approvals: Vec<UserIncomingApproval>,
    badges_to_create: Vec<Balance>,
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
    default_auto_approve_self_initiated_outgoing_transfers: bool,
    default_auto_approve_self_initiated_incoming_transfers: bool,
    default_user_permissions: UserPermissions,
  },
}

pub fn delete_collection_msg(
    collection_id: String,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::DeleteCollectionMsg {
        collection_id,
    }.into()

}

pub fn address_mappings_msg(
  address_mappings: Vec<AddressMapping>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::CreateAddressMappingsMsg {
    address_mappings,
  }.into()
}

pub fn transfer_badges_msg(
  collection_id: String,
  transfers: Vec<Transfer>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::TransferBadgeMsg {
    collection_id,
    transfers,
  }.into()
}

pub fn create_collection_msg(
  balances_type: String,
  default_outgoing_approvals: Vec<UserOutgoingApproval>,
  default_incoming_approvals: Vec<UserIncomingApproval>,
  badges_to_create: Vec<Balance>,
  collection_permissions: CollectionPermissions,
  manager_timeline: Vec<ManagerTimeline>,
  collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
  badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
  off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
  custom_data_timeline: Vec<CustomDataTimeline>,
  collection_approvals: Vec<CollectionApproval>,
  standards_timeline: Vec<StandardsTimeline>,
  is_archived_timeline: Vec<IsArchivedTimeline>,
  default_auto_approve_self_initiated_outgoing_transfers: bool,
  default_auto_approve_self_initiated_incoming_transfers: bool,
  default_user_permissions: UserPermissions,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::CreateCollectionMsg { 
    balances_type,
    default_outgoing_approvals,
    default_incoming_approvals,
    badges_to_create,
    collection_permissions,
    manager_timeline,
    collection_metadata_timeline,
    badge_metadata_timeline,
    off_chain_balances_metadata_timeline,
    custom_data_timeline,
    collection_approvals,
    standards_timeline,
    is_archived_timeline,
    default_auto_approve_self_initiated_outgoing_transfers,
    default_auto_approve_self_initiated_incoming_transfers,
    default_user_permissions,
  }.into()
}
  
pub fn update_collection_msg(
  collection_id: String,
  badges_to_create: Vec<Balance>,
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
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::UpdateCollectionMsg {
      collection_id,
      badges_to_create,
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
  }
  .into()
}

pub fn universal_update_collection_msg(
  collection_id: String,
  balances_type: String,
  default_outgoing_approvals: Vec<UserOutgoingApproval>,
  default_incoming_approvals: Vec<UserIncomingApproval>,
  badges_to_create: Vec<Balance>,
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
  default_auto_approve_self_initiated_outgoing_transfers: bool,
  default_auto_approve_self_initiated_incoming_transfers: bool,
  default_user_permissions: UserPermissions,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::UniversalUpdateCollectionMsg {
      collection_id,
      balances_type,
      default_outgoing_approvals,
      default_incoming_approvals,
      badges_to_create,
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
      default_auto_approve_self_initiated_outgoing_transfers,
      default_auto_approve_self_initiated_incoming_transfers,
      default_user_permissions,
  }
  .into()
}




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AddressMapping {
    pub mapping_id: String,
    pub addresses: Vec<String>,
    pub include_addresses: bool,
    pub uri: String,
    pub custom_data: String,
    pub created_by: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Transfer {
    pub from: String,
    pub to_addresses: Vec<String>,
    pub balances: Vec<Balance>,
    pub precalculate_balances_from_approval: Option<ApprovalIdentifierDetails>,
    pub merkle_proofs: Vec<MerkleProof>,
    pub memo: String,
    pub prioritized_approvals: Vec<ApprovalIdentifierDetails>,
    pub only_check_prioritized_approvals: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Balance {
    pub amount: String,
    pub ownership_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UintRange {
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ApprovalIdentifierDetails {
    pub approval_id: String,
    pub approval_level: String,
    pub approver_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MerkleProof {
    pub leaf: String,
    pub aunts: Vec<MerklePathItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MerklePathItem {
    pub aunt: String,
    pub on_right: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MustOwnBadges {
    pub collection_id: String,
    pub amount_range: UintRange,
    pub ownership_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub override_with_current_time: bool,
    pub must_own_all: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BadgeMetadata {
    pub uri: String,
    pub custom_data: String,
    pub badge_ids: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionMetadata {
    pub uri: String,
    pub custom_data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OffChainBalancesMetadata {
    pub uri: String,
    pub custom_data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionMetadataTimeline {
    pub collection_metadata: CollectionMetadata,
    pub timeline_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BadgeMetadataTimeline {
    pub badge_metadata: Vec<BadgeMetadata>,
    pub timeline_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OffChainBalancesMetadataTimeline {
    pub off_chain_balances_metadata: OffChainBalancesMetadata,
    pub timeline_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CustomDataTimeline {
    pub custom_data: String,
    pub timeline_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ManagerTimeline {
    pub manager: String,
    pub timeline_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IsArchivedTimeline {
    pub is_archived: bool,
    pub timeline_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractAddressTimeline {
    pub contract_address: String,
    pub timeline_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StandardsTimeline {
    pub standards: Vec<String>,
    pub timeline_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ManualBalances {
    pub balances: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IncrementedBalances {
    pub start_balances: Vec<Balance>,
    pub increment_badge_ids_by: String,
    pub increment_ownership_times_by: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PredeterminedOrderCalculationMethod {
    pub use_overall_num_transfers: bool,
    pub use_per_to_address_num_transfers: bool,
    pub use_per_from_address_num_transfers: bool,
    pub use_per_initiated_by_address_num_transfers: bool,
    pub use_merkle_challenge_leaf_index: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PredeterminedBalances {
    pub manual_balances: Vec<ManualBalances>,
    pub incremented_balances: IncrementedBalances,
    pub order_calculation_method: PredeterminedOrderCalculationMethod,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ApprovalAmounts {
    pub overall_approval_amount: String,
    pub per_to_address_approval_amount: String,
    pub per_from_address_approval_amount: String,
    pub per_initiated_by_address_approval_amount: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MaxNumTransfers {
  pub overall_max_num_transfers: String,
  pub per_to_address_max_num_transfers: String,
  pub per_from_address_max_num_transfers: String,
  pub per_initiated_by_address_max_num_transfers: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ApprovalsTracker {
    pub num_transfers: String,
    pub amounts: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ApprovalCriteria {
  must_own_badges: Vec<MustOwnBadges>,
  merkle_challenge: MerkleChallenge,
  predetermined_balances: PredeterminedBalances,
  approval_amounts: ApprovalAmounts,
  max_num_transfers: MaxNumTransfers,
  require_to_equals_initiated_by: bool,
  require_from_equals_initiated_by: bool,
  require_to_does_not_equal_initiated_by: bool,
  require_from_does_not_equal_initiated_by: bool,
  overrides_from_outgoing_approvals: bool,
  overrides_to_incoming_approvals: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OutgoingApprovalCriteria {
  must_own_badges: Vec<MustOwnBadges>,
  merkle_challenge: MerkleChallenge,
  predetermined_balances: PredeterminedBalances,
  approval_amounts: ApprovalAmounts,
  max_num_transfers: MaxNumTransfers,
  require_to_equals_initiated_by: bool,
  require_to_does_not_equal_initiated_by: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IncomingApprovalCriteria {
  must_own_badges: Vec<MustOwnBadges>,
  merkle_challenge: MerkleChallenge,
  predetermined_balances: PredeterminedBalances,
  approval_amounts: ApprovalAmounts,
  max_num_transfers: MaxNumTransfers,
  require_from_equals_initiated_by: bool,
  require_from_does_not_equal_initiated_by: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MerkleChallenge {
    pub root: String,
    pub expected_proof_length: String,
    pub use_creator_address_as_leaf: bool,
    pub max_uses_per_leaf: String,
    pub uri: String,
    pub custom_data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserOutgoingApproval {
    pub to_mapping_id: String,
    pub initiated_by_mapping_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub amount_tracker_id: String,
    pub challenge_tracker_id: String,
    pub uri: String,
    pub custom_data: String,
    pub approval_id: String,
    pub approval_criteria: OutgoingApprovalCriteria,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserIncomingApproval {
    pub from_mapping_id: String,
    pub initiated_by_mapping_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub amount_tracker_id: String,
    pub challenge_tracker_id: String,
    pub uri: String,
    pub custom_data: String,
    pub approval_id: String,
    pub approval_criteria: IncomingApprovalCriteria,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionApproval {
    pub from_mapping_id: String,
    pub to_mapping_id: String,
    pub initiated_by_mapping_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub amount_tracker_id: String,
    pub challenge_tracker_id: String,
    pub uri: String,
    pub custom_data: String,
    pub approval_id: String,
    pub approval_criteria: ApprovalCriteria,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionPermissions {
    pub can_delete_collection: Vec<ActionPermission>,
    pub can_archive_collection: Vec<TimedUpdatePermission>,
    pub can_update_off_chain_balances_metadata: Vec<TimedUpdatePermission>,
    pub can_update_standards: Vec<TimedUpdatePermission>,
    pub can_update_custom_data: Vec<TimedUpdatePermission>,
    pub can_update_manager: Vec<TimedUpdatePermission>,
    pub can_update_collection_metadata: Vec<TimedUpdatePermission>,
    pub can_create_more_badges: Vec<BalancesActionPermission>,
    pub can_update_badge_metadata: Vec<TimedUpdateWithBadgeIdsPermission>,
    pub can_update_collection_approvals: Vec<CollectionApprovalPermission>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserPermissions {
    pub can_update_outgoing_approvals: Vec<UserOutgoingApprovalPermission>,
    pub can_update_incoming_approvals: Vec<UserIncomingApprovalPermission>,
    pub can_update_auto_approve_self_initiated_outgoing_transfers: Vec<ActionPermission>,
    pub can_update_auto_approve_self_initiated_incoming_transfers: Vec<ActionPermission>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionApprovalPermission {
    pub from_mapping_id: String,
    pub to_mapping_id: String,
    pub initiated_by_mapping_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub amount_tracker_id: String,
    pub challenge_tracker_id: String,
    pub permitted_times: Vec<UintRange>,
    pub forbidden_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserOutgoingApprovalPermission {
    pub to_mapping_id: String,
    pub initiated_by_mapping_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub amount_tracker_id: String,
    pub challenge_tracker_id: String,
    pub permitted_times: Vec<UintRange>,
    pub forbidden_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserIncomingApprovalPermission {
    pub from_mapping_id: String,
    pub initiated_by_mapping_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub amount_tracker_id: String,
    pub challenge_tracker_id: String,
    pub permitted_times: Vec<UintRange>,
    pub forbidden_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalancesActionPermission {
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub permitted_times: Vec<UintRange>,
    pub forbidden_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ActionPermission {
    pub permitted_times: Vec<UintRange>,
    pub forbidden_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TimedUpdatePermission {
    pub permitted_times: Vec<UintRange>,
    pub forbidden_times: Vec<UintRange>,
    pub timeline_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TimedUpdateWithBadgeIdsPermission {
    pub badge_ids: Vec<UintRange>,
    pub permitted_times: Vec<UintRange>,
    pub forbidden_times: Vec<UintRange>,
    pub timeline_times: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserBalanceStore {
    pub balances: Vec<Balance>,
    pub outgoing_approvals: Vec<UserOutgoingApproval>,
    pub incoming_approvals: Vec<UserIncomingApproval>,
    pub auto_approve_self_initiated_outgoing_transfers: bool,
    pub auto_approve_self_initiated_incoming_transfers: bool,
    pub user_permissions: UserPermissions,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BadgeCollection {
  collection_id: String,
  collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
  badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
  balances_type: String,
  off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
  custom_data_timeline: Vec<CustomDataTimeline>,
  manager_timeline: Vec<ManagerTimeline>,
  collection_permissions: CollectionPermissions,
  collection_approvals: Vec<CollectionApproval>,
  standards_timeline: Vec<StandardsTimeline>,
  is_archived_timeline: Vec<IsArchivedTimeline>,
  default_outgoing_approvals: Vec<UserOutgoingApproval>,
  default_incoming_approvals: Vec<UserIncomingApproval>,
  default_auto_approve_self_initiated_outgoing_transfers: bool,
  default_auto_approve_self_initiated_incoming_transfers: bool,
  default_user_permissions: UserPermissions,
  created_by: String,
}