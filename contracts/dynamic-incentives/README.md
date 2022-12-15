# dynamic-incentives

# Osmosis Localchain

### clone della repository di osmosis
```bash
git clone https://github.com/osmosis-labs/osmosis.git
cd osmosis
```

### Avvio della chain locale
```bash
make localnet-build

make localnet-start
```
Se i comandi falliscono per `permission denied` al docker, è sufficiente eseguire con `sudo`.

#### Avvio della chain con state
```bash
make localnet-start-with-state
```

Questo comando permette di avviare la chain con un certo stato iniziale. In generale vengono definiti 3 pools nel modulo `Gamm` e per ognuno dei quali vengono creati 3 gauges, uno per ogni duration.
### Fermare la chain conservando lo stato attuale
```bash
make localnet-stop
```
Questo comando ferma la chain e al prossimo avvio si riparte dallo stato precendente. Mentre per un reset della chain prima di un avvio si usa:

```bash
make localnet-clean
```

# Store & Instantiate
```bash
WALLET_CREATOR="osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks"
CHAIN_ID="localosmosis"

DYNAMIC_INCENTIVES_CONTRACT="$HOME/Dynamic-Incentives/target/wasm32-unknown-unknown/release/dynamic_incentives.wasm"

osmosisd tx wasm store $DYNAMIC_INCENTIVES_CONTRACT \
--from $WALLET_CREATOR \
--chain-id=$CHAIN_ID \
--gas-prices 0.1uosmo --gas auto \
--gas-adjustment 1.3 \
-b block -y
```

Viene salvato il code_id del contratto (in questo caso è 1) e lo si usa per instanziare il contratto

```bash
DYNAMIC_INCENTIVES_CODE_ID=1

INIT='{"owner":"osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks", "osmo_base_reward":{"denom":"uosmo", "amount":"1000"}}'

osmosisd tx wasm instantiate $DYNAMIC_INCENTIVES_CODE_ID \
"$INIT"  \
--label "First Dynamic incentives Contract" \
--from $WALLET_CREATOR \
--chain-id $CHAIN_ID \
--gas-prices 0.1uosmo \
--gas auto --gas-adjustment 1.3 \
-b block \
--no-admin -y
```
Salvare l'address del contratto dal reply della instantiate.

```bash
CONTRACT_ADDRESS="osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9"
```

# Execute del contratto

## ADD TO GAUGE
```bash
ADD_TO_GAUGE='{"add_to_gauge":{"gauge_id": 10, "owner": "osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks", "reward_amount":[{"denom": "uosmo", "amount":"5000"}]}}'

osmosisd tx wasm execute $CONTRACT_ADDRESS \
"$ADD_TO_GAUGE" \
--amount 5000uosmo \
--from $WALLET_CREATOR \
--chain-id=$CHAIN_ID \
--gas-prices 0.1uosmo --gas auto \
--gas-adjustment 1.3 \
-b block -o text -y
```

Attualmente questa transazione fallisce con un `contract doesn't have permission: unauthorized`.

## UPDATE OSMO BASE REWARD
```bash
UPDATE_OSMO_BASE_REWARD='{"update_osmo_base_reward":{"new_base_reward":{"denom": "uosmo", "amount":"1500"}}}'

osmosisd tx wasm execute $CONTRACT_ADDRESS \
"$UPDATE_OSMO_BASE_REWARD" \
--from $WALLET_CREATOR \
--chain-id $CHAIN_ID \
--gas-prices 0.1uosmo --gas auto \
--gas-adjustment 1.3 \
-b block -o text -y
```

Aggiorna il reward base da distribuire ai gauges.

## UPDATE OWNER ADDRESS
```bash
UPDATE_OWNER_ADDR='{"update_owner_addr":{"addr":"osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj"}}'

osmosisd tx wasm execute $CONTRACT_ADDRESS \
"$UPDATE_OWNER_ADDR" \
--from $WALLET_CREATOR \
--chain-id $CHAIN_ID \
--gas-prices 0.1uosmo --gas auto \
--gas-adjustment 1.3 \
-b block -o text -y
```

# Queries

```bash
osmosisd query wasm contract-state smart $CONTRACT_ADDRESS '{"info":{}}'

osmosisd query wasm contract-state smart $CONTRACT_ADDRESS '{"get_owner":{}}'

osmosisd query wasm contract-state smart $CONTRACT_ADDRESS '{"get_osmo_base_reward":{}}'
```

NB: La query `info{}` non ritorna nulla in quanto non è stato ancora definito.