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

### Fermare la chain conservando lo stato attuale
```bash
make localnet-stop
```
Questo comando ferma la chain e al prossimo avvio si riparte dallo stato precendente. Mentre per un reset della chain prima di un avvio si usa:

```bash
make localnet-remove
```

