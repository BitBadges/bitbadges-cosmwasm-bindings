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
  TransferTokensMsg {
    collection_id: String,
    transfers: Vec<Transfer>,
  },

  #[serde(rename_all = "camelCase")]
  CreateCollectionMsg {
    default_balances: UserBalanceStore,
    valid_token_ids: Vec<UintRange>,
    collection_permissions: CollectionPermissions,
    manager: String,
    collection_metadata: CollectionMetadata,
    token_metadata: Vec<TokenMetadata>,
    custom_data: String,
    collection_approvals: Vec<CollectionApproval>,
    standards: Vec<String>,
    is_archived: bool,
    mint_escrow_coins_to_transfer: Vec<CosmosCoin>,
    cosmos_coin_wrapper_paths_to_add: Vec<CosmosCoinWrapperPathAddObject>,
    alias_paths_to_add: Vec<AliasPathAddObject>,
    invariants: CollectionInvariants,
  },


  #[serde(rename_all = "camelCase")]
  UpdateCollectionMsg {
    collection_id: String,
    update_valid_token_ids: bool,
    valid_token_ids: Vec<UintRange>,
    update_collection_permissions: bool,
    collection_permissions: CollectionPermissions,
    update_manager: bool,
    manager: String,
    update_collection_metadata: bool,
    collection_metadata: CollectionMetadata,
    update_token_metadata: bool,
    token_metadata: Vec<TokenMetadata>,
    update_custom_data: bool,
    custom_data: String,
    update_collection_approvals: bool,
    collection_approvals: Vec<CollectionApproval>,
    update_standards: bool,
    standards: Vec<String>,
    update_is_archived: bool,
    is_archived: bool,
    mint_escrow_coins_to_transfer: Vec<CosmosCoin>,
    cosmos_coin_wrapper_paths_to_add: Vec<CosmosCoinWrapperPathAddObject>,
    alias_paths_to_add: Vec<AliasPathAddObject>,
    invariants: Option<InvariantsAddObject>,
  },

  #[serde(rename_all = "camelCase")]
  UniversalUpdateCollectionMsg {
    collection_id: String,
    default_balances: UserBalanceStore,
    update_valid_token_ids: bool,
    valid_token_ids: Vec<UintRange>,
    update_collection_permissions: bool,
    collection_permissions: CollectionPermissions,
    update_manager: bool,
    manager: String,
    update_collection_metadata: bool,
    collection_metadata: CollectionMetadata,
    update_token_metadata: bool,
    token_metadata: Vec<TokenMetadata>,
    update_custom_data: bool,
    custom_data: String,
    update_collection_approvals: bool,
    collection_approvals: Vec<CollectionApproval>,
    update_standards: bool,
    standards: Vec<String>,
    update_is_archived: bool,
    is_archived: bool,
    mint_escrow_coins_to_transfer: Vec<CosmosCoin>,
    cosmos_coin_wrapper_paths_to_add: Vec<CosmosCoinWrapperPathAddObject>,
    alias_paths_to_add: Vec<AliasPathAddObject>,
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
  CreateDynamicStoreMsg {
    default_value: bool,
  },

  #[serde(rename_all = "camelCase")]
  UpdateDynamicStoreMsg {
    store_id: String,
    default_value: bool,
    global_enabled: bool,
  },

  #[serde(rename_all = "camelCase")]
  DeleteDynamicStoreMsg {
    store_id: String,
  },

  #[serde(rename_all = "camelCase")]
  SetDynamicStoreValueMsg {
    store_id: String,
    address: String,
    value: bool,
  },


  #[serde(rename_all = "camelCase")]
  SetIncomingApprovalMsg {
    creator: String,
    collection_id: String,
    approval: UserIncomingApproval,
  },

  #[serde(rename_all = "camelCase")]
  DeleteIncomingApprovalMsg {
    creator: String,
    collection_id: String,
    approval_id: String,
  },

  #[serde(rename_all = "camelCase")]
  SetOutgoingApprovalMsg {
    creator: String,
    collection_id: String,
    approval: UserOutgoingApproval,
  },

  #[serde(rename_all = "camelCase")]
  DeleteOutgoingApprovalMsg {
    creator: String,
    collection_id: String,
    approval_id: String,
  },

  #[serde(rename_all = "camelCase")]
  PurgeApprovalsMsg {
    creator: String,
    collection_id: String,
    purge_expired: bool,
    approver_address: String,
    purge_counterparty_approvals: bool,
    approvals_to_purge: Vec<ApprovalIdentifierDetails>,
  },

  #[serde(rename_all = "camelCase")]
  SetValidTokenIdsMsg {
    creator: String,
    collection_id: String,
    valid_token_ids: Vec<UintRange>,
    can_update_valid_token_ids: Vec<TokenIdsActionPermission>,
  },

  #[serde(rename_all = "camelCase")]
  SetManagerMsg {
    creator: String,
    collection_id: String,
    manager: String,
    can_update_manager: Vec<ActionPermission>,
  },

  #[serde(rename_all = "camelCase")]
  SetCollectionMetadataMsg {
    creator: String,
    collection_id: String,
    collection_metadata: CollectionMetadata,
    can_update_collection_metadata: Vec<ActionPermission>,
  },

  #[serde(rename_all = "camelCase")]
  SetTokenMetadataMsg {
    creator: String,
    collection_id: String,
    token_metadata: Vec<TokenMetadata>,
    can_update_token_metadata: Vec<TokenIdsActionPermission>,
  },

  #[serde(rename_all = "camelCase")]
  SetCustomDataMsg {
    creator: String,
    collection_id: String,
    custom_data: String,
    can_update_custom_data: Vec<ActionPermission>,
  },

  #[serde(rename_all = "camelCase")]
  SetStandardsMsg {
    creator: String,
    collection_id: String,
    standards: Vec<String>,
    can_update_standards: Vec<ActionPermission>,
  },

  #[serde(rename_all = "camelCase")]
  SetCollectionApprovalsMsg {
    creator: String,
    collection_id: String,
    collection_approvals: Vec<CollectionApproval>,
    can_update_collection_approvals: Vec<CollectionApprovalPermission>,
  },

  #[serde(rename_all = "camelCase")]
  SetIsArchivedMsg {
    creator: String,
    collection_id: String,
    is_archived: bool,
    can_archive_collection: Vec<ActionPermission>,
  },

  #[serde(rename_all = "camelCase")]
  SetReservedProtocolAddressMsg {
    authority: String,
    address: String,
    is_reserved_protocol: bool,
  },

  #[serde(rename_all = "camelCase")]
  CastVoteMsg {
    collection_id: String,
    approval_level: String,
    approver_address: String,
    approval_id: String,
    proposal_id: String,
    yes_weight: String,
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

pub fn transfer_tokens_msg(
  collection_id: String,
  transfers: Vec<Transfer>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::TransferTokensMsg {
    collection_id,
    transfers,
    }.into()
}

pub fn create_collection_msg(
  default_balances: UserBalanceStore,
  valid_token_ids: Vec<UintRange>,
  collection_permissions: CollectionPermissions,
  manager: String,
  collection_metadata: CollectionMetadata,
  token_metadata: Vec<TokenMetadata>,
  custom_data: String,
  collection_approvals: Vec<CollectionApproval>,
  standards: Vec<String>,
  is_archived: bool,
  mint_escrow_coins_to_transfer: Vec<CosmosCoin>,
  cosmos_coin_wrapper_paths_to_add: Vec<CosmosCoinWrapperPathAddObject>,
  alias_paths_to_add: Vec<AliasPathAddObject>,
  invariants: CollectionInvariants
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::CreateCollectionMsg { 
    default_balances: default_balances,
    valid_token_ids,
    collection_permissions,
    manager,
    collection_metadata,
    token_metadata,
    custom_data,
    collection_approvals,
    standards,
    is_archived,
    mint_escrow_coins_to_transfer,
    cosmos_coin_wrapper_paths_to_add,
    alias_paths_to_add,
    invariants,
  }.into()
}
  
pub fn update_collection_msg(
  collection_id: String,
  update_valid_token_ids: bool,
  valid_token_ids: Vec<UintRange>,
  update_collection_permissions: bool,
  collection_permissions: CollectionPermissions,
  update_manager: bool,
  manager: String,
  update_collection_metadata: bool,
  collection_metadata: CollectionMetadata,
  update_token_metadata: bool,
  token_metadata: Vec<TokenMetadata>,
  update_custom_data: bool,
  custom_data: String,
  update_collection_approvals: bool,
  collection_approvals: Vec<CollectionApproval>,
  update_standards: bool,
  standards: Vec<String>,
  update_is_archived: bool,
  is_archived: bool,
  mint_escrow_coins_to_transfer: Vec<CosmosCoin>,
  cosmos_coin_wrapper_paths_to_add: Vec<CosmosCoinWrapperPathAddObject>,
  alias_paths_to_add: Vec<AliasPathAddObject>,
  invariants: Option<InvariantsAddObject>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::UpdateCollectionMsg {
    collection_id,
    update_valid_token_ids,
    valid_token_ids,
    update_collection_permissions,
    collection_permissions,
    update_manager,
    manager,
    update_collection_metadata,
    collection_metadata,
    update_token_metadata,
    token_metadata,
    update_custom_data,
    custom_data,
    update_collection_approvals,
    collection_approvals,
    update_standards,
    standards,
    update_is_archived,
    is_archived,
    mint_escrow_coins_to_transfer,
    cosmos_coin_wrapper_paths_to_add,
    alias_paths_to_add,
    invariants,
  }
  .into()
}

pub fn universal_update_collection_msg(
  collection_id: String,
  default_balances: UserBalanceStore,
  update_valid_token_ids: bool,
  valid_token_ids: Vec<UintRange>,
  update_collection_permissions: bool,
  collection_permissions: CollectionPermissions,
  update_manager: bool,
  manager: String,
  update_collection_metadata: bool,
  collection_metadata: CollectionMetadata,
  update_token_metadata: bool,
  token_metadata: Vec<TokenMetadata>,
  update_custom_data: bool,
  custom_data: String,
  update_collection_approvals: bool,
  collection_approvals: Vec<CollectionApproval>,
  update_standards: bool,
  standards: Vec<String>,
  update_is_archived: bool,
  is_archived: bool,
  mint_escrow_coins_to_transfer: Vec<CosmosCoin>,
  cosmos_coin_wrapper_paths_to_add: Vec<CosmosCoinWrapperPathAddObject>,
  alias_paths_to_add: Vec<AliasPathAddObject>,
  invariants: CollectionInvariants
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::UniversalUpdateCollectionMsg {
    collection_id,
    default_balances: default_balances,
    update_valid_token_ids,
    valid_token_ids,
    update_collection_permissions,
    collection_permissions,
    update_manager,
    manager,
    update_collection_metadata,
    collection_metadata,
    update_token_metadata,
    token_metadata,
    update_custom_data,
    custom_data,
    update_collection_approvals,
    collection_approvals,
    update_standards,
    standards,
    update_is_archived,
    is_archived,
    mint_escrow_coins_to_transfer,
    cosmos_coin_wrapper_paths_to_add,
    alias_paths_to_add,
    invariants,
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

pub fn create_dynamic_store_msg(
  default_value: bool,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::CreateDynamicStoreMsg {
    default_value,
  }
  .into()
}

pub fn update_dynamic_store_msg(
  store_id: String,
  default_value: bool,
  global_enabled: bool,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::UpdateDynamicStoreMsg {
    store_id,
    default_value,
    global_enabled,
  }
  .into()
}

pub fn delete_dynamic_store_msg(
  store_id: String,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::DeleteDynamicStoreMsg {
    store_id,
  }
  .into()
}

pub fn set_dynamic_store_value_msg(
  store_id: String,
  address: String,
  value: bool,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::SetDynamicStoreValueMsg {
    store_id,
    address,
    value,
  }
  .into()
}

pub fn set_incoming_approval_msg(
  creator: String,
  collection_id: String,
  approval: UserIncomingApproval,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::SetIncomingApprovalMsg {
    creator,
    collection_id,
    approval,
  }
  .into()
}

pub fn delete_incoming_approval_msg(
  creator: String,
  collection_id: String,
  approval_id: String,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::DeleteIncomingApprovalMsg {
    creator,
    collection_id,
    approval_id,
  }
  .into()
}

pub fn set_outgoing_approval_msg(
  creator: String,
  collection_id: String,
  approval: UserOutgoingApproval,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::SetOutgoingApprovalMsg {
    creator,
    collection_id,
    approval,
  }
  .into()
}

pub fn delete_outgoing_approval_msg(
  creator: String,
  collection_id: String,
  approval_id: String,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::DeleteOutgoingApprovalMsg {
    creator,
    collection_id,
    approval_id,
  }
  .into()
}

pub fn purge_approvals_msg(
  creator: String,
  collection_id: String,
  purge_expired: bool,
  approver_address: String,
  purge_counterparty_approvals: bool,
  approvals_to_purge: Vec<ApprovalIdentifierDetails>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::PurgeApprovalsMsg {
    creator,
    collection_id,
    purge_expired,
    approver_address,
    purge_counterparty_approvals,
    approvals_to_purge,
  }
  .into()
}

pub fn set_valid_token_ids_msg(
  creator: String,
  collection_id: String,
  valid_token_ids: Vec<UintRange>,
  can_update_valid_token_ids: Vec<TokenIdsActionPermission>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::SetValidTokenIdsMsg {
    creator,
    collection_id,
    valid_token_ids,
    can_update_valid_token_ids,
  }
  .into()
}

pub fn set_manager_msg(
  creator: String,
  collection_id: String,
  manager: String,
  can_update_manager: Vec<ActionPermission>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::SetManagerMsg {
    creator,
    collection_id,
    manager,
    can_update_manager,
  }
  .into()
}

pub fn set_collection_metadata_msg(
  creator: String,
  collection_id: String,
  collection_metadata: CollectionMetadata,
  can_update_collection_metadata: Vec<ActionPermission>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::SetCollectionMetadataMsg {
    creator,
    collection_id,
    collection_metadata,
    can_update_collection_metadata,
  }
  .into()
}

pub fn set_token_metadata_msg(
  creator: String,
  collection_id: String,
  token_metadata: Vec<TokenMetadata>,
  can_update_token_metadata: Vec<TokenIdsActionPermission>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::SetTokenMetadataMsg {
    creator,
    collection_id,
    token_metadata,
    can_update_token_metadata,
  }
  .into()
}

pub fn set_custom_data_msg(
  creator: String,
  collection_id: String,
  custom_data: String,
  can_update_custom_data: Vec<ActionPermission>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::SetCustomDataMsg {
    creator,
    collection_id,
    custom_data,
    can_update_custom_data,
  }
  .into()
}

pub fn set_standards_msg(
  creator: String,
  collection_id: String,
  standards: Vec<String>,
  can_update_standards: Vec<ActionPermission>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::SetStandardsMsg {
    creator,
    collection_id,
    standards,
    can_update_standards,
  }
  .into()
}

pub fn set_collection_approvals_msg(
  creator: String,
  collection_id: String,
  collection_approvals: Vec<CollectionApproval>,
  can_update_collection_approvals: Vec<CollectionApprovalPermission>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::SetCollectionApprovalsMsg {
    creator,
    collection_id,
    collection_approvals,
    can_update_collection_approvals,
  }
  .into()
}

pub fn set_is_archived_msg(
  creator: String,
  collection_id: String,
  is_archived: bool,
  can_archive_collection: Vec<ActionPermission>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::SetIsArchivedMsg {
    creator,
    collection_id,
    is_archived,
    can_archive_collection,
  }
  .into()
}

pub fn set_reserved_protocol_address_msg(
  authority: String,
  address: String,
  is_reserved_protocol: bool,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::SetReservedProtocolAddressMsg {
    authority,
    address,
    is_reserved_protocol,
  }
  .into()
}

pub fn cast_vote_msg(
  collection_id: String,
  approval_level: String,
  approver_address: String,
  approval_id: String,
  proposal_id: String,
  yes_weight: String,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::CastVoteMsg {
    collection_id,
    approval_level,
    approver_address,
    approval_id,
    proposal_id,
    yes_weight,
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
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub from: String,
    pub to_addresses: Vec<String>,
    pub balances: Vec<Balance>,
    pub precalculate_balances_from_approval: Option<PrecalculateBalancesFromApprovalDetails>,
    pub merkle_proofs: Vec<MerkleProof>,
    pub eth_signature_proofs: Vec<ETHSignatureProof>,
    pub memo: String,
    pub prioritized_approvals: Vec<ApprovalIdentifierDetails>,
    pub only_check_prioritized_collection_approvals: bool,
    pub only_check_prioritized_incoming_approvals: bool,
    pub only_check_prioritized_outgoing_approvals: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PrecalculateBalancesFromApprovalDetails {
    pub approval_id: String,
    pub approval_level: String,
    pub approver_address: String,
    pub version: String,
    pub precalculation_options: PrecalculationOptions,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PrecalculationOptions {
    pub override_timestamp: String,
    pub token_ids_override: Vec<UintRange>,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub amount: String,
    pub ownership_times: Vec<UintRange>,
    pub token_ids: Vec<UintRange>,
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
pub struct ETHSignatureProof {
    pub nonce: String,
    pub signature: String,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenMetadata {
    pub uri: String,
    pub custom_data: String,
    pub token_ids: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CollectionMetadata {
    pub uri: String,
    pub custom_data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PathMetadata {
    pub uri: String,
    pub custom_data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionSideA {
    pub amount: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionSideAWithDenom {
    pub amount: String,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionWithoutDenom {
    pub side_a: ConversionSideA,
    pub side_b: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Conversion {
    pub side_a: ConversionSideAWithDenom,
    pub side_b: Vec<Balance>,
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
    pub increment_token_ids_by: String,
    pub increment_ownership_times_by: String,
    pub duration_from_timestamp: String,
    pub allow_override_timestamp: bool,
    pub recurring_ownership_times: RecurringOwnershipTimes,
    pub allow_override_with_any_valid_token: bool,
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
  pub must_own_tokens: Vec<MustOwnTokens>,
  pub dynamic_store_challenges: Vec<DynamicStoreChallenge>,
  pub eth_signature_challenges: Vec<ETHSignatureChallenge>,
  pub sender_checks: Option<AddressChecks>,
  pub recipient_checks: Option<AddressChecks>,
  pub initiator_checks: Option<AddressChecks>,
  pub alt_time_checks: Option<AltTimeChecks>,
  pub must_prioritize: bool,
  pub voting_challenges: Vec<VotingChallenge>,
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
  pub must_own_tokens: Vec<MustOwnTokens>,
  pub dynamic_store_challenges: Vec<DynamicStoreChallenge>,
  pub eth_signature_challenges: Vec<ETHSignatureChallenge>,
  pub recipient_checks: Option<AddressChecks>,
  pub initiator_checks: Option<AddressChecks>,
  pub alt_time_checks: Option<AltTimeChecks>,
  pub must_prioritize: bool,
  pub voting_challenges: Vec<VotingChallenge>,
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
  pub must_own_tokens: Vec<MustOwnTokens>,
  pub dynamic_store_challenges: Vec<DynamicStoreChallenge>,
  pub eth_signature_challenges: Vec<ETHSignatureChallenge>,
  pub sender_checks: Option<AddressChecks>,
  pub initiator_checks: Option<AddressChecks>,
  pub alt_time_checks: Option<AltTimeChecks>,
  pub must_prioritize: bool,
  pub voting_challenges: Vec<VotingChallenge>,
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
    pub token_ids: Vec<UintRange>,
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
    pub token_ids: Vec<UintRange>,
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
    pub token_ids: Vec<UintRange>,
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
    pub can_archive_collection: Vec<ActionPermission>,
    pub can_update_standards: Vec<ActionPermission>,
    pub can_update_custom_data: Vec<ActionPermission>,
    pub can_update_manager: Vec<ActionPermission>,
    pub can_update_collection_metadata: Vec<ActionPermission>,
    pub can_update_valid_token_ids: Vec<TokenIdsActionPermission>,
    pub can_update_token_metadata: Vec<TokenIdsActionPermission>,
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
    pub token_ids: Vec<UintRange>,
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
    pub token_ids: Vec<UintRange>,
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
    pub token_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenIdsActionPermission {
    pub token_ids: Vec<UintRange>,
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
pub struct TokenCollection {
  pub collection_id: String,
  pub collection_metadata: CollectionMetadata,
  pub token_metadata: Vec<TokenMetadata>,
  pub custom_data: String,
  pub manager: String,
  pub collection_permissions: CollectionPermissions,
  pub collection_approvals: Vec<CollectionApproval>,
  pub standards: Vec<String>,
  pub is_archived: bool,
  pub default_balances: UserBalanceStore,
  pub created_by: String,
  pub valid_token_ids: Vec<UintRange>,
  pub mint_escrow_address: String,
  pub cosmos_coin_wrapper_paths: Vec<CosmosCoinWrapperPath>,
  pub alias_paths: Vec<AliasPath>,
  pub invariants: CollectionInvariants,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CosmosCoinWrapperPath {
  pub address: String,
  pub denom: String,
  pub conversion: ConversionWithoutDenom,
  pub symbol: String,
  pub denom_units: Vec<DenomUnit>,
  pub allow_override_with_any_valid_token: bool,
  pub metadata: PathMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DenomUnit {
  pub decimals: String,
  pub symbol: String,
  pub is_default_display: bool,
  pub metadata: PathMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AutoDeletionOptions {
  pub after_one_use: bool,
  pub after_overall_max_num_transfers: bool,
  pub allow_counterparty_purge: bool,
  pub allow_purge_if_expired: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRoyalties {
  pub percentage: String,
  pub payout_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MustOwnTokens {
  pub collection_id: String,
  pub amount_range: UintRange,
  pub ownership_times: Vec<UintRange>,
  pub token_ids: Vec<UintRange>,
  pub override_with_current_time: bool,
  pub must_satisfy_for_all_assets: bool,
  pub ownership_check_party: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DynamicStoreChallenge {
  pub store_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ETHSignatureChallenge {
  pub signer: String,
  pub challenge_tracker_id: String,
  pub uri: String,
  pub custom_data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddressChecks {
  pub must_be_wasm_contract: bool,
  pub must_not_be_wasm_contract: bool,
  pub must_be_liquidity_pool: bool,
  pub must_not_be_liquidity_pool: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AltTimeChecks {
  pub offline_hours: Vec<UintRange>,
  pub offline_days: Vec<UintRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Voter {
  pub address: String,
  pub weight: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct VotingChallenge {
  pub proposal_id: String,
  pub quorum_threshold: String,
  pub voters: Vec<Voter>,
  pub uri: String,
  pub custom_data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct VoteProof {
  pub proposal_id: String,
  pub voter: String,
  pub yes_weight: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CosmosCoinWrapperPathAddObject {
  pub denom: String,
  pub conversion: ConversionWithoutDenom,
  pub symbol: String,
  pub denom_units: Vec<DenomUnit>,
  pub allow_override_with_any_valid_token: bool,
  pub metadata: PathMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AliasPath {
  pub denom: String,
  pub conversion: ConversionWithoutDenom,
  pub symbol: String,
  pub denom_units: Vec<DenomUnit>,
  pub metadata: PathMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AliasPathAddObject {
  pub denom: String,
  pub conversion: ConversionWithoutDenom,
  pub symbol: String,
  pub denom_units: Vec<DenomUnit>,
  pub metadata: PathMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CollectionInvariants {
  pub no_custom_ownership_times: bool,
  pub max_supply_per_id: String,
  pub cosmos_coin_backed_path: Option<CosmosCoinBackedPath>,
  pub no_forceful_post_mint_transfers: bool,
  pub disable_pool_creation: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CosmosCoinBackedPath {
  pub address: String,
  pub conversion: Conversion,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CosmosCoinBackedPathAddObject {
  pub conversion: Conversion,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InvariantsAddObject {
  pub no_custom_ownership_times: bool,
  pub max_supply_per_id: String,
  pub cosmos_coin_backed_path: Option<CosmosCoinBackedPathAddObject>,
  pub no_forceful_post_mint_transfers: bool,
  pub disable_pool_creation: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DynamicStore {
  pub store_id: String,
  pub created_by: String,
  pub default_value: bool,
  pub global_enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DynamicStoreValue {
  pub store_id: String,
  pub address: String,
  pub value: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Params {
  // Add fields as needed based on the proto definition
  // This is a placeholder - you'll need to add the actual fields from the params.proto file
}