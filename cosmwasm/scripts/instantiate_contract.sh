wasmd tx wasm instantiate $1 $2 --label $3 --admin alice --from alice --gas auto --gas-adjustment 1.15 --yes

address=$(wasmd query wasm list-contract-by-code $1 --output json | jq -r '.contracts[-1]' )

echo "Deployed contract address: $address"