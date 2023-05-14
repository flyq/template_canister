# Template Canister

## build
```sh
cargo run  --features "export-api" > .artifact/template_canister.did

cargo build --target wasm32-unknown-unknown --release --features "export-api"

ic-wasm target/wasm32-unknown-unknown/release/template_canister.wasm -o .artifact/template_canister.wasm shrink
```

## deploy in local

terminal 0:
```sh
dfx start --clean
```

terminal 1:
```sh
dfx canister create --no-wallet template_canister

dfx build template_canister

dfx canister install template_canister --argument "record { owner=principal \"$(dfx identity get-principal)\"}"

dfx canister call template_canister get_owner
```