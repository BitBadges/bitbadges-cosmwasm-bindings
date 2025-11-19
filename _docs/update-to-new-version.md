# Updating to New Version Types

This document outlines the process for updating the BitBadges CosmWasm bindings to support a new blockchain version.

## Steps

### 1. Fetch Latest Proto Files

Run the `get_proto.sh` script from the base directory to fetch the current proto files:

```bash
source ./scripts/get_proto.sh
```

This will:

-   Copy proto files from `../bitbadgeschain/proto/badges/` to `./proto/badges/`
-   Clean up version folders and unnecessary files

### 2. Read All Proto Definitions

Review all `*.proto` files in `./proto/badges/` to identify:

-   New message types in `tx.proto` (for msg definitions)
-   New query types in `query.proto` (for query types)
-   New types in other proto files (collections.proto, balances.proto, permissions.proto, etc.)

### 3. Update Missing Types

Update any missing types in `packages/bitbadges-cosmwasm/src/`:

-   **msg.rs**: Add new message variants to `BitBadgesMsg` enum and corresponding helper functions
-   **query.rs**: Add new query variants to `BitBadgesQuery` enum and corresponding response types
-   **Other types**: Update struct definitions for any new types used in messages/queries

**Important Notes:**

-   The `creator` field is already handled properly - don't duplicate it
-   Pay attention to naming conventions (camelCase in JSON, snake_case in Rust)
-   Use `tx.proto` for message definitions
-   Use `query.proto` for query types
-   Use all other proto files for everything else
-   Iterate carefully on each type to catch everything, including small naming conventions

### 4. Build and Generate Schema

In the `packages/bitbadges-cosmwasm` folder:

```bash
cd packages/bitbadges-cosmwasm
cargo build
cargo schema
```

Both commands must succeed before proceeding.

### 5. Test Contract Build

Return to the base directory and test the contract build:

```bash
cd ../..
cd contracts/contract_example
source ./build.sh
```

This should complete successfully.

### 6. Update README

Update the README.md with the newest version support:

-   Add the new version to the compatibility table
-   Update any relevant documentation

## Version History

-   v19: Added support for:

    -   `SetReservedProtocolAddressMsg` message type
    -   `QueryGetWrappableBalances` query type
    -   `QueryIsAddressReservedProtocol` query type
    -   `QueryGetAllReservedProtocolAddresses` query type
    -   `CosmosCoinBackedPath` and `CosmosCoinBackedPathAddObject` types
    -   `InvariantsAddObject` type
    -   `AddressChecks` type for checking address types (WASM contract, liquidity pool, etc.)
    -   Updated `CollectionInvariants` with additional fields: `cosmos_coin_backed_path`, `no_forceful_post_mint_transfers`, `disable_pool_creation`
    -   Updated `CosmosCoinWrapperPath` with `allow_override_with_any_valid_token` and `allow_cosmos_wrapping` fields
    -   Added `invariants` field to `UpdateCollectionMsg` (optional, as invariants cannot be modified after creation)
    -   Added address checks to approval criteria:
        -   `ApprovalCriteria`: `sender_checks`, `recipient_checks`, `initiator_checks` (all optional `AddressChecks`)
        -   `OutgoingApprovalCriteria`: `recipient_checks`, `initiator_checks` (both optional `AddressChecks`)
        -   `IncomingApprovalCriteria`: `sender_checks`, `initiator_checks` (both optional `AddressChecks`)

-   v20: Changes:
    -   Removed `affiliate_address` field from `Transfer` struct (removed from `MsgTransferTokens`)
