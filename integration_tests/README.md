# Tests:
* interactions between smart contracts and bitbadges-noded NFT module are working.
* interactions between smart contracts and native modules bank/gov/staking.
* smart contract to smart contract instantiation and execution.

# How to start tests
1) Start bitbadges-node instance by executing ```init.sh``` inside  ```/bitbadges-node/```
2) Compile the native_tester and nft_bindings_tester smart contracts.
3) From the ```bitbadges-cosmwasm-bindings``` directory execute the tests ```./integration_tests/test.sh bitbadges-local-network path_to_native_tester.wasm path_to_nft_bindings_tester.wasm path_to_bitbadges_node_root_dir```