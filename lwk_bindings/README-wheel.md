# Liquid Wallet Kit

A Python package to build on the [Liquid](https://blockstream.com/liquid/) network.

```python
import lwk
network = lwk.Network.mainnet()
assert(str(network) == "Liquid")
```

## Main Features

* **Watch-Only** wallet support: using Liquid descriptors, better known as
  [CT descriptors](https://github.com/ElementsProject/ELIPs/blob/main/elip-0150.mediawiki).
* **PSET** based: transactions are shared and processed using the
  [Partially Signed Elements Transaction](https://github.com/ElementsProject/elements/blob/1fcf0cf2323b7feaff5d1fc4c506fff5ec09132e/doc/pset.mediawiki) format.
* **Electrum** and **Esplora** [backends](https://github.com/Blockstream/electrs):
  no need to run and sync a full Liquid node or rely on closed source servers.
* **Asset issuance**, **reissuance** and **burn** support: manage the lifecycle
  of your Issued Assets with a lightweight client.
* **Generic multisig** wallets: create a wallet controlled by
  any combination of hardware or software signers, with a user
  specified quorum.

## Examples

* [List transactions](https://github.com/Blockstream/lwk/tree/master/lwk_bindings/tests/bindings/list_transactions.py) of a wpkh/slip77 wallet
* [Send transaction](https://github.com/Blockstream/lwk/tree/master/lwk_bindings/tests/bindings/send_transaction.py) of a wpkh/slip77 wallet in a regtest environment
* [Send asset](https://github.com/Blockstream/lwk/tree/master/lwk_bindings/tests/bindings/send_asset.py) of a wpkh/slip77 wallet in a regtest environment
* [Custom persister](https://github.com/Blockstream/lwk/tree/master/lwk_bindings/tests/bindings/custom_persister.py), the caller code provide how the wallet updates are persisted


