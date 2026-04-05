# Deployment

In order to deploy a Polytone connection between 2 chains (e.g. archway and osmosis), you should follow the following steps.

> **NOTE**: Keep in min that Polytone connections go only one way. This tutorial allows you to control an account on chain `dst_chain` from `src_chain` but not the other way around!

## Prerequesistes

In order to be able to deploy polytone and create a connection, the two chains you want to link together should both include polytone contracts uploaded.

### Building the contracts

If the artifacts are not present in the repository, run the following command to build them and optimize them :

```bash
./devtools/optimize.sh
```

### Uploading all the contracts on the chain

We offer the possibility to upload all contracts to a specific chain in [`scripts/src/bin/upload_contracts.rs`](scripts/src/bin/upload_contracts.rs). Simply change the chain variable to the chain on which you want to deploy the contracts.

## Contract instantiation

We will instantiate a `note` contract on `src_chain` and a `voice` on `dst_chain`.

To do so, use [`scripts/src/bin/instantiate_chains.rs`](scripts/src/bin/instantiate_chains.rs). Change the `src_chain` and `dst_chain` variables accordingly.

You can specify optional addresses that will own the contracts migration rights with the `src_admin_addr` and `dst_admin_addr` variables.

## Channel creation

Finally you need to create a channel between the two contracts.

### Hermes setup

1. [Install hermes following the official tutorial](https://hermes.informal.systems/quick-start/installation.html)
2. If you're using mainnets, you can setup the relayer using the following command:

    ```bash
    hermes config auto --chains archway osmosis
    ```

3. You need to add your mnemonics or account keys to the relayer on both chains. [Follow this tutorial to learn how to add keys](https://hermes.informal.systems/tutorials/production/setup-hermes.html#setup-accounts).
4. You might need to adjust the `key_name` variables in the `$HOME/.hermes/config.toml` file to the name of the key you created.
5. You may need to config the `max_gas` variables in the `$HOME/.hermes/config.toml` file because they are not set correctly by default (some errors in the following steps will guid you in that direct). We advise you to setup the `max_gas` paramter to `1000000` on both chains for a correct channel creation process.

### Channel command

You may now create a channel with:

```bash
hermes create channel --a-chain <src-chain-id> --a-connection <connection-id> --a-port wasm.<src-note-contract-address> --b-port wasm.<dst-voice-contract-address> --channel-version polytone-1
```

e.g.

```bash
hermes create channel --a-chain archway-1 --a-connection connection-1 --a-port wasm.archway1zla0cm4sjytmktj4skrdm00hlars6q4jgkeqz5wy0tlftzu494tsqpkhuj --b-port wasm.osmo1kju3qsgcfwfuhqdrwm623xrf7hn3lmp2scaznxn75r3mlv36x4xsc7lnz2 --channel-version polytone-1
```

In return you should get all the channel ids you need (on the `src_chain` and on the `dst_chain`).

### Testing

You can make sure everything is setup correctly by using [`scripts/src/bin/verify_deployment.rs`](scripts/src/bin/verify_deployment.rs).
This creates a transaction with empty sent messages across polytone. If the relayer is setup correctly, it should relay the packets succesfully with an `Ok` acknowledgment sent back

### Setting up hermes to relay only packets on this channel

Use the following configuration for each chain you want to relay on to limit the channel to the channel you just created.

```toml
[chains.packet_filter]
policy = "allow"
list = [
    [
    "wasm.<contract-address>",
    "<created-channel-id>",
],
]
```

## Deploy Matrix

| From\To  | Osmosis | Archway | Terra | Juno | Neutron | Stargaze | Kujira | Migaloo |
|---       |----|----|----|----|----|----|----|----|
| Osmosis  | ❌ | ✅ | 🟢 | 🟢 | 🟢 | 🟢 | ✅ | 🟢 |
| Archway  | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |    |
| Terra    | ✅ | ✅ | ❌ | ✅ | ✅ |    | ✅ |    |
| Juno     | 🟢 | ✅ | 🟢 | ❌ |    | 🟢 | ✅ | 🟢 |
| Neutron  | 🟢 | ✅ | 🟢 |    | ❌ | 🟢 | ✅ |    |
| Stargaze | 🟢 | ✅ |    | 🟢 | 🟢 | ❌ | ✅ | 🟢 |
| Kujira   | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |    |
| Migaloo  | 🟢 |    |    | 🟢 |    | 🟢 |    | ❌ |

✅: Deployed by Abstract

❌: No reason to deploy

🟢: Deployed by DA0DA0
