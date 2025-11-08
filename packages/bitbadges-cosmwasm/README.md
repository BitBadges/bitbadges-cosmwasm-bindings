# BitBadges bindings for CosmWasm

This crate provides the custom bindings that are used to communicate with the custom modules (x/badges) on the BitBadges network from a CosmWasm smart contract.

# Installation

Add the crate to your smart contract's' Cargo.toml

```toml
[dependencies]
bitbadges-cosmwasm = { version = "X.X.X" }
```

# Exposed bindings

This crate, as of now, exports binding only for the x/badges module. In the future, more custom bindings will be added.

## Creating Messages

​
**NOTE:** The BitBadges bindings do not cover messages that have already been implemented by the CosmWasm team, such as staking-related messages and fundamental ones like `MsgSend`.
​
You may want your contract to perform messages such as `DeleteCollection` and `TransferTokens` operations at the end of its execution. To do this, create a message using the predefined functions:
​

-   `transfer_tokens_msg`
-   ...

And add it to your response, like below
​

```rust
use cosmwasm_std::CosmosMsg;
use bitbadges_cosmwasm::{transfer_tokens_msg};
​
...
​
pub fn execute_msg_transfer_tokens(
    collection_id: String,
    transfers: Vec<Transfer>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = transfer_tokens_msg(
        collection_id,
        transfers,
    );

    Ok(Response::new().add_message(msg))
}
```

### Querying

​
In order to use the query functions enabled by the bindings, create a `BitBadgesQuerier` instance within your contract logic -- in either `init()` or `query()` entrypoints. You can access all the enabled queries through this object.
​

```rust
// src/contract.rs
use bitbadges_cosmwasm::{ BitBadgesQuerier };
​
...
​pub fn query_collection(deps: Deps, collection_id: String) -> StdResult<TokenCollection> {
    let querier = BitBadgesQuerier::new(&deps.querier);
    let res: TokenCollection = querier.query_collection(collection_id)?;

    Ok(res)
}

```

# Example

Please consult the example smart contracts in /contracts - there you can see an example how to issue a transaction or make a query from the smart contract to the custom module.
You can upload it and interact with it (and through it - with the bitbadges chain) with the following steps.
This assumes you have a local bitbadges chain running.

```bash
clonedDir='path/to/the/test/smart/contract/binary'
bitbadgeschaind tx wasm store $clonedDir/bindings_tester.wasm --from=<address> --chain-id=<chain-id> --gas=auto -y
INIT='{}'

# Find the code_id of your stored contract (in the raw_log of output of the previous command or can query with bitbadgeschaind query wasm list-code)

bitbadgeschaind tx wasm instantiate your_code_id $INIT --from=<address> --label="your_label" --chain-id=<chain-id> --gas=auto -y
TESTER=$(bitbadgeschaind query wasm list-contract-by-code $CODE --output json | jq -r '.contracts[-1]')
echo $TESTER

# NOTE: sender field in the queries should be the address of your contract, in this case - $TESTER
# issueDenom
# NOTE: schema is optional field

# NOTE: we convert to camelCase which should be used here
msgDetails='{
    "transferBadgesMsg": {
        "collectionId": "test",
        "transfers": [
            {
              ....
            }
        ]
    }
}'
bitbadgeschaind tx wasm execute $TESTER $msgDetails --from=<address> --chain-id=<chain-id> --gas=auto -y
```

## Acknowledgements

This repository was forked form the Cudos cosmwasm bindings.
We would like to thank the Cudos team for their work on this project.
