deploy: 
    near deploy --wasmFile ./contract/target/wasm32-unknown-unknown/release/contract.wasm --accountId testooken.devpham.testnet

near call:
    near call testooken.devpham.testnet mint '{"account_id": "devpham.testnet", "amount": "1000000000000000000000000"}' --accountId devpham.testnet

    near call dev-1689996463321-29455003036828 get_balance '{"account_id":"devpham.testnet"}' --accountId devpham.testnet
