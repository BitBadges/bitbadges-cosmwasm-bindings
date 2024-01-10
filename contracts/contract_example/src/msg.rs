use bitbadges_cosmwasm::{
  AddressList, Transfer,
  Balance, CollectionPermissions, ManagerTimeline, CollectionMetadataTimeline, BadgeMetadataTimeline, 
  OffChainBalancesMetadataTimeline, CustomDataTimeline, CollectionApproval, StandardsTimeline, IsArchivedTimeline, UserBalanceStore
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use cosmwasm_std::{Coin, VoteOption};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

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
    TransferBadgesMsg {
        collection_id: String,
        transfers: Vec<Transfer>,
    },

    #[serde(rename_all = "camelCase")]
    CreateCollectionMsg {
      balances_type: String,
      default_balances: UserBalanceStore,
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
    },

    #[serde(rename_all = "camelCase")]
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

    #[serde(rename_all = "camelCase")]
    UniversalUpdateCollectionMsg {
      collection_id: String,
      balances_type: String,
      default_balances: UserBalanceStore,
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

    // Add other messages here as needed
    // Same format as above
}
