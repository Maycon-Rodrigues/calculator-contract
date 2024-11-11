# Soroban Project

## 1. config networking

```
stellar network add local \
    --rpc-url "http://localhost:8000/soroban/rpc" \
    --network-passphrase "Standalone Network ; February 2017"
```

## 2. config wallet

```
stellar keys generate --global bob --network local
```

## 3. build smartcontracts

```
stellar contract build
```

## 4. deploy smartcontract

```
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/<CONTRACT_NAME>.wasm \
  --source bob \
  --network local
```

## 5. interact with smartcontract

```
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source bob \
  --network local \
  -- \
  sum \
  --x 10 --y 5
```
