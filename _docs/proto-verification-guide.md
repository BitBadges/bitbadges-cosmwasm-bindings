# Proto-to-Rust Type Consistency Verification Guide

This document provides systematic instructions for verifying that all proto definitions match the Rust type implementations exactly. Use this guide when updating to new blockchain versions or when checking consistency.

## Quick Start Guide

1. **Fetch Latest Proto Files**: Run `source ./scripts/get_proto.sh` from the base directory
2. **Follow Systematic Verification**: Use the checklists below to verify each category
3. **Document Findings**: Update the Findings Log section as you discover issues
4. **Fix Discrepancies**: Update Rust code to match proto exactly
5. **Verify Build**: Run `cargo build` and `cargo schema` in `packages/bitbadges-cosmwasm/`
6. **Test Contract**: Build example contract to ensure everything works

## Systematic Verification Process

### Step 1: Message Types Verification
Compare `proto/badges/tx.proto` → `packages/bitbadges-cosmwasm/src/msg.rs`

For each `Msg*` message type:
- [ ] Message name matches (e.g., `MsgCreateCollection` → `CreateCollectionMsg`)
- [ ] All proto fields exist in Rust struct
- [ ] Field names: proto camelCase → Rust snake_case
- [ ] All structs have `#[serde(rename_all = "camelCase")]`
- [ ] Field types match exactly
- [ ] Optional fields use `Option<T>` in Rust
- [ ] `creator` field is NOT in Rust (handled by CosmWasm)

### Step 2: Query Types Verification
Compare `proto/badges/query.proto` → `packages/bitbadges-cosmwasm/src/query.rs`

For each query:
- [ ] Query request variant exists in `BitBadgesQuery` enum
- [ ] Query response struct exists
- [ ] All fields match proto definition
- [ ] Field naming conventions followed

### Step 3: Supporting Types Verification
Compare all proto files → Rust structs in `msg.rs` or `query.rs`

For each type:
- [ ] Struct exists in Rust
- [ ] All fields present
- [ ] Field names follow camelCase → snake_case
- [ ] Types match exactly
- [ ] Optionality matches (proto3 optional → `Option<T>`)

## Type Mapping Reference

| Proto Type | Rust Type | Notes |
|------------|-----------|-------|
| `string` | `String` | Always |
| `repeated T` | `Vec<T>` | Always |
| `optional T` | `Option<T>` | Proto3 optional fields |
| `bool` | `bool` | Always |
| `uint64` (with customtype Uint) | `String` | Serialized as string |
| `message T` | Rust struct `T` | Nested messages |
| `cosmos.base.v1beta1.Coin` | `CosmosCoin` | Custom struct |

## Naming Convention Rules

### Proto → Rust Field Name Conversion

1. **Proto**: camelCase (e.g., `collectionId`, `allowOverrideWithAnyValidToken`)
2. **Rust**: snake_case (e.g., `collection_id`, `allow_override_with_any_valid_token`)
3. **JSON Serialization**: camelCase (via `#[serde(rename_all = "camelCase")]`)

### Message Type Naming

- **Proto**: `MsgCreateCollection`
- **Rust Enum Variant**: `CreateCollectionMsg`
- **Pattern**: `Msg*` → `*Msg`

### Required Attributes

All Rust structs must have:
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TypeName {
    // fields...
}
```

## Verification Checklists

### Message Types Checklist

- [ ] `MsgCreateCollection` → `CreateCollectionMsg`
- [ ] `MsgUpdateCollection` → `UpdateCollectionMsg`
- [ ] `MsgUniversalUpdateCollection` → `UniversalUpdateCollectionMsg`
- [ ] `MsgTransferTokens` → `TransferTokensMsg`
- [ ] `MsgDeleteCollection` → `DeleteCollectionMsg`
- [ ] `MsgCreateAddressLists` → `CreateAddressListsMsg`
- [ ] `MsgUpdateUserApprovals` → `UpdateUserApprovalsMsg`
- [ ] `MsgSetIncomingApproval` → `SetIncomingApprovalMsg`
- [ ] `MsgSetOutgoingApproval` → `SetOutgoingApprovalMsg`
- [ ] `MsgDeleteIncomingApproval` → `DeleteIncomingApprovalMsg`
- [ ] `MsgDeleteOutgoingApproval` → `DeleteOutgoingApprovalMsg`
- [ ] `MsgPurgeApprovals` → `PurgeApprovalsMsg`
- [ ] `MsgCreateDynamicStore` → `CreateDynamicStoreMsg`
- [ ] `MsgUpdateDynamicStore` → `UpdateDynamicStoreMsg`
- [ ] `MsgDeleteDynamicStore` → `DeleteDynamicStoreMsg`
- [ ] `MsgSetDynamicStoreValue` → `SetDynamicStoreValueMsg`
- [ ] `MsgSetValidTokenIds` → `SetValidTokenIdsMsg`
- [ ] `MsgSetManager` → `SetManagerMsg`
- [ ] `MsgSetCollectionMetadata` → `SetCollectionMetadataMsg`
- [ ] `MsgSetTokenMetadata` → `SetTokenMetadataMsg`
- [ ] `MsgSetCustomData` → `SetCustomDataMsg`
- [ ] `MsgSetStandards` → `SetStandardsMsg`
- [ ] `MsgSetCollectionApprovals` → `SetCollectionApprovalsMsg`
- [ ] `MsgSetIsArchived` → `SetIsArchivedMsg`
- [ ] `MsgSetReservedProtocolAddress` → `SetReservedProtocolAddressMsg`
- [ ] `MsgCastVote` → `CastVoteMsg`

### Query Types Checklist

- [ ] `QueryGetCollection` → `QueryCollection`
- [ ] `QueryGetBalance` → `QueryBalance`
- [ ] `QueryGetAddressList` → `QueryAddressList`
- [ ] `QueryGetApprovalTracker` → `QueryApprovalTracker`
- [ ] `QueryGetChallengeTracker` → `QueryChallengeTracker`
- [ ] `QueryGetDynamicStore` → `QueryDynamicStore`
- [ ] `QueryGetDynamicStoreValue` → `QueryDynamicStoreValue`
- [ ] `QueryGetETHSignatureTracker` → `QueryGetETHSignatureTracker`
- [ ] `QueryGetWrappableBalances` → `QueryGetWrappableBalances`
- [ ] `QueryIsAddressReservedProtocol` → `QueryIsAddressReservedProtocol`
- [ ] `QueryGetAllReservedProtocolAddresses` → `QueryGetAllReservedProtocolAddresses`
- [ ] `QueryGetVote` → `QueryGetVote`
- [ ] `QueryGetVotes` → `QueryGetVotes`

### Supporting Types Checklist

#### Collections
- [ ] `TokenCollection`
- [ ] `CollectionMetadata`
- [ ] `TokenMetadata`
- [ ] `CollectionInvariants`
- [ ] `InvariantsAddObject`
- [ ] `CosmosCoinWrapperPath`
- [ ] `CosmosCoinWrapperPathAddObject`
- [ ] `CosmosCoinBackedPath`
- [ ] `CosmosCoinBackedPathAddObject`
- [ ] `AliasPath`
- [ ] `AliasPathAddObject`
- [ ] `PathMetadata`
- [ ] `Conversion`
- [ ] `ConversionWithoutDenom`
- [ ] `ConversionSideA`
- [ ] `ConversionSideAWithDenom`
- [ ] `DenomUnit`

#### Transfers & Balances
- [ ] `Transfer`
- [ ] `Balance`
- [ ] `UintRange`
- [ ] `PrecalculateBalancesFromApprovalDetails`
- [ ] `PrecalculationOptions`
- [ ] `MerkleProof`
- [ ] `MerklePathItem`
- [ ] `ETHSignatureProof`
- [ ] `ApprovalIdentifierDetails`

#### Approvals
- [ ] `CollectionApproval`
- [ ] `UserOutgoingApproval`
- [ ] `UserIncomingApproval`
- [ ] `ApprovalCriteria`
- [ ] `OutgoingApprovalCriteria`
- [ ] `IncomingApprovalCriteria`

#### Approval Criteria Components
- [ ] `MerkleChallenge`
- [ ] `ETHSignatureChallenge`
- [ ] `VotingChallenge`
- [ ] `Voter`
- [ ] `VoteProof`
- [ ] `PredeterminedBalances`
- [ ] `ManualBalances`
- [ ] `IncrementedBalances`
- [ ] `RecurringOwnershipTimes`
- [ ] `PredeterminedOrderCalculationMethod`
- [ ] `ApprovalAmounts`
- [ ] `MaxNumTransfers`
- [ ] `ResetTimeIntervals`
- [ ] `ApprovalTracker`
- [ ] `CoinTransfer`
- [ ] `CosmosCoin`
- [ ] `AutoDeletionOptions`
- [ ] `UserRoyalties`
- [ ] `MustOwnTokens`
- [ ] `DynamicStoreChallenge`
- [ ] `AddressChecks`
- [ ] `AltTimeChecks`

#### Permissions
- [ ] `CollectionPermissions`
- [ ] `UserPermissions`
- [ ] `CollectionApprovalPermission`
- [ ] `UserOutgoingApprovalPermission`
- [ ] `UserIncomingApprovalPermission`
- [ ] `TokenIdsActionPermission`
- [ ] `ActionPermission`

#### Other Types
- [ ] `AddressList`
- [ ] `UserBalanceStore`
- [ ] `DynamicStore`
- [ ] `DynamicStoreValue`

## Common Patterns

### Pattern 1: Message with Creator Field
**Proto:**
```protobuf
message MsgExample {
  string creator = 1;
  string collectionId = 2;
}
```

**Rust (CORRECT):**
```rust
#[serde(rename_all = "camelCase")]
ExampleMsg {
    // creator is NOT included - handled by CosmWasm
    collection_id: String,
}
```

**Rust (INCORRECT):**
```rust
ExampleMsg {
    creator: String,  // ❌ Should NOT be here
    collection_id: String,
}
```

### Pattern 2: Optional Field
**Proto:**
```protobuf
message Example {
  string required = 1;
  optional string optional = 2;
}
```

**Rust:**
```rust
#[serde(rename_all = "camelCase")]
pub struct Example {
    pub required: String,
    pub optional: Option<String>,  // ✅ Optional field
}
```

### Pattern 3: Repeated Field
**Proto:**
```protobuf
message Example {
  repeated string items = 1;
}
```

**Rust:**
```rust
#[serde(rename_all = "camelCase")]
pub struct Example {
    pub items: Vec<String>,  // ✅ Repeated → Vec
}
```

## Findings Log

### v22 Verification Findings

*This section will be updated in real-time as findings are discovered during verification.*

#### Date: 2024-12-19

**Status**: Verification in progress

**Findings**:

1. **CRITICAL - DynamicStore Type Mismatch**:
   - `DynamicStore.default_value`: Proto has `bool`, Rust has `String` ❌
   - `DynamicStoreValue.value`: Proto has `bool`, Rust has `String` ❌
   - `CreateDynamicStoreMsg.default_value`: Proto has `bool`, Rust has `String` ❌
   - `UpdateDynamicStoreMsg.default_value`: Proto has `bool`, Rust has `String` ❌
   - `SetDynamicStoreValueMsg.value`: Proto has `bool`, Rust has `String` ❌

2. **Creator Field Issues**:
   - `CreateDynamicStoreMsg`, `UpdateDynamicStoreMsg`, `SetDynamicStoreValueMsg`, `DeleteDynamicStoreMsg` all have `creator: String` fields that should NOT be present (handled by CosmWasm) ❌
   - `IncrementStoreValueMsg`, `DecrementStoreValueMsg` have `creator` fields and don't exist in proto ❌

3. **Extra Messages Not in Proto**:
   - `IncrementStoreValueMsg` - Does not exist in proto ❌
   - `DecrementStoreValueMsg` - Does not exist in proto ❌

4. **AddressList Extra Field**:
   - `AddressList.alias_address: Option<String>` - Field does not exist in proto `AddressList` message ❌

5. **AddressListInput vs AddressList**:
   - `MsgCreateAddressLists` uses `AddressListInput` in proto, but Rust uses `AddressList`
   - This works because `AddressList` has `created_by` as optional, but not ideal ⚠️

6. **Creator Fields in Message Variants**:
   - Many message variants still have `creator: String` fields that should be removed (handled by CosmWasm)
   - This is a widespread issue affecting: SetIncomingApprovalMsg, DeleteIncomingApprovalMsg, SetOutgoingApprovalMsg, DeleteOutgoingApprovalMsg, PurgeApprovalsMsg, SetValidTokenIdsMsg, SetManagerMsg, SetCollectionMetadataMsg, SetTokenMetadataMsg, SetCustomDataMsg, SetStandardsMsg, SetCollectionApprovalsMsg, SetIsArchivedMsg
   - Note: These will still work but don't follow best practices ⚠️

**Fixed Issues**:
1. ✅ Fixed `DynamicStore.default_value`: Changed from `String` to `bool`
2. ✅ Fixed `DynamicStoreValue.value`: Changed from `String` to `bool`
3. ✅ Fixed `CreateDynamicStoreMsg.default_value`: Changed from `String` to `bool`, removed `creator` field
4. ✅ Fixed `UpdateDynamicStoreMsg.default_value`: Changed from `String` to `bool`, removed `creator` field
5. ✅ Fixed `SetDynamicStoreValueMsg.value`: Changed from `String` to `bool`, removed `creator` field
6. ✅ Fixed `DeleteDynamicStoreMsg`: Removed `creator` field
7. ✅ Removed `IncrementStoreValueMsg` and `DecrementStoreValueMsg` (not in proto)
8. ✅ Removed `AddressList.alias_address` field (not in proto)
9. ✅ Updated all helper functions to match new signatures

**Build Status**: ✅ Code compiles successfully
**Schema Generation**: ✅ Schema generation succeeds

## Troubleshooting

### Issue: Field Name Mismatch
**Symptom**: Proto has `collectionId` but Rust has `collectionId` (should be `collection_id`)

**Solution**: 
1. Rename field to snake_case: `collection_id`
2. Ensure `#[serde(rename_all = "camelCase")]` is present
3. Verify JSON serialization uses camelCase

### Issue: Missing Optional Wrapper
**Symptom**: Proto has `optional string field` but Rust has `field: String` (should be `Option<String>`)

**Solution**: Wrap type in `Option<T>` for optional fields

### Issue: Creator Field Present
**Symptom**: Rust struct includes `creator` field that shouldn't be there

**Solution**: Remove `creator` field - it's handled by CosmWasm framework automatically

### Issue: Type Mismatch
**Symptom**: Proto has `uint64` with customtype Uint, but Rust has `u64`

**Solution**: Use `String` type for Uint fields (serialized as string)

## Examples

### Correct Implementation Example

**Proto:**
```protobuf
message MsgCreateCollection {
  string creator = 1;
  UserBalanceStore defaultBalances = 2;
  repeated UintRange validTokenIds = 3;
  bool isArchived = 4;
  optional string customData = 5;
}
```

**Rust:**
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum BitBadgesMsg {
    #[serde(rename_all = "camelCase")]
    CreateCollectionMsg {
        // creator NOT included
        default_balances: UserBalanceStore,
        valid_token_ids: Vec<UintRange>,
        is_archived: bool,
        custom_data: Option<String>,  // Optional field
    },
}
```

### Incorrect Implementation Example

**Issues:**
- ❌ `creator` field included
- ❌ `custom_data` not wrapped in `Option`
- ❌ Missing `#[serde(rename_all = "camelCase")]` on enum variant

## Future Updates

When updating for a new version:

1. **Update Version Number**: Change version references in this document
2. **Review Changelog**: Check `_docs/update-to-new-version.md` for version-specific changes
3. **Re-run Verification**: Use this guide to verify all types again
4. **Document Changes**: Add new findings to Findings Log with version number
5. **Update Checklists**: Add any new message/query/types to checklists

### Version-Specific Notes

#### v22 Changes
- Timeline functionality removed (fields converted to direct values)
- Permission updates (TimedUpdatePermission → ActionPermission)
- Precalculation options moved to PrecalculateBalancesFromApprovalDetails
- New CastVoteMsg message type
- Alias path support added
- Path redesign with conversion and metadata fields
- Voting challenge support added

---

*Last Updated: 2024-12-19*

## Verification Summary

**Status**: Initial verification complete for v22

**Critical Issues Fixed**: 9 major type mismatches and structural issues
**Remaining Issues**: Creator field removal (non-critical, works but not ideal)
**Build Status**: ✅ Passing
**Schema Generation**: ✅ Passing

**Next Steps for Complete Verification**:
1. Remove all `creator` fields from message enum variants (handled by CosmWasm)
2. Consider adding `AddressListInput` type for better accuracy
3. Continue systematic field-by-field verification of all remaining types
4. Test with example contract to ensure end-to-end functionality

