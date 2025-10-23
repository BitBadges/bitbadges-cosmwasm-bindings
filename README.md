# BitBadges CosmWasm Bindings

This crate provides custom bindings for communicating with the BitBadges network's custom modules (x/badges) from CosmWasm smart contracts.

## Installation

Add the crate to your smart contract's `Cargo.toml`:

```toml
[dependencies]
bitbadges-cosmwasm = { version = "X.X.X" }
```

## Version Compatibility

| Blockchain Version | Bindings Version                         | Repository Reference                                                                                                  |
| ------------------ | ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| v12                | a3dad84d0ed7b8fdcb789a5cc8fbd04562e8b068 | [v12 Release](https://github.com/BitBadges/bitbadges-cosmwasm-bindings/tree/a3dad84d0ed7b8fdcb789a5cc8fbd04562e8b068) |
| v13                | a3dad84d0ed7b8fdcb789a5cc8fbd04562e8b068 | [v13 Release](https://github.com/BitBadges/bitbadges-cosmwasm-bindings/tree/a3dad84d0ed7b8fdcb789a5cc8fbd04562e8b068) |
| v14                | a3dad84d0ed7b8fdcb789a5cc8fbd04562e8b068 | [v14 Release](https://github.com/BitBadges/bitbadges-cosmwasm-bindings/tree/a3dad84d0ed7b8fdcb789a5cc8fbd04562e8b068) |
| v16                | current                                  | [Current](https://github.com/BitBadges/bitbadges-cosmwasm-bindings)                                                   |

**Note:** The v12/v13/v14/v16 refer to our blockchain's versions. Make sure to use the appropriate bindings version for your target blockchain version.

## Features

Currently, this crate exports bindings for the x/badges module. Additional custom bindings will be added in the future.

### Creating Messages

**Note:** The BitBadges bindings do not cover messages that have already been implemented by the CosmWasm team or other libraries, such as staking-related messages and fundamental ones like `MsgSend`.

To perform x/badges message operations at the end of your contract's execution, create a message using the predefined functions:

-   `create_register_addresses_msg`
-   ...

Then add it to your response as shown below:

```rust
use cosmwasm_std::CosmosMsg;
use bitbadges_cosmwasm::create_register_addresses_msg;

pub fn execute_msg_register_addresses(
    _deps: DepsMut<BitBadgesQuery>,
    _env: Env,
    _info: MessageInfo,
    addresses_to_register: Vec<String>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = create_register_addresses_msg(addresses_to_register);
    Ok(Response::new().add_message(msg))
}
```

### Querying

To use the query functions enabled by the bindings, create a `BitBadgesQuerier` instance within your contract logic - either in `init()` or `query()` entrypoints. You can access all enabled queries through this object:

```rust
use bitbadges_cosmwasm::BitBadgesQuerier;

pub fn query_collection(deps: Deps, id: u64) -> StdResult<BadgeCollection> {
    let querier = BitBadgesQuerier::new(&deps.querier);
    let res: BadgeCollection = querier.query_collection(id)?;
    Ok(res)
}
```

## Example Usage

Example smart contracts can be found in the `/contracts` directory. These demonstrate how to issue transactions and make queries from a smart contract to the custom module.

To upload and interact with the example contract (assuming you have a local BitBadges chain running):

```bash
# Store the contract
clonedDir='path/to/the/test/smart/contract/binary'
bitbadgeschaind tx wasm store $clonedDir/bindings_tester.wasm \
    --from=<address> \
    --chain-id=<chain-id> \
    --gas=auto -y

# Initialize the contract
INIT='{}'
bitbadgeschaind tx wasm instantiate your_code_id $INIT \
    --from=<address> \
    --label="your_label" \
    --chain-id=<chain-id> \
    --gas=auto -y

# Get the contract address
TESTER=$(bitbadgeschaind query wasm list-contract-by-code $CODE --output json | jq -r '.contracts[-1]')
echo $TESTER

# Execute a message
msgDetails='{
    ....
}'
bitbadgeschaind tx wasm execute $TESTER $msgDetails \
    --from=<address> \
    --chain-id=<chain-id> \
    --gas=auto -y
```

**Note:** The `sender` field in queries should be your contract's address (in this case, `$TESTER`).

## Acknowledgements

This repository was forked from the Cudos cosmwasm bindings. We would like to thank the Cudos team for their work on this project.
