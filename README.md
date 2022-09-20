# NSwap AMM

This mono repo contains the source code for the smart contracts of AMM built for [NEAR](https://near.org).

## Contracts

| Contract                            | Description                                                                   |
| ----------------------------------- | ----------------------------------------------------------------------------- |
| [test-token](test-token/src/lib.rs) | Test token contract                                                           |
| [nswap](nswap/src/lib.rs)           | Main exchange contract, that allows to deposit and withdraw tokens, swap them |

## Development

### Building

1. Install `rustup` via https://rustup.rs/
2. Run the following

```bash
$ make rust-setup
$ make build
```

### Testing

Contracts have unit tests and also integration tests using NEAR Simulation framework. All together can be run:

```bash
$ make test
```

## Deploy

### Near TestNet

```bash
$ npm install
$ npm run deploy
```

Every smart contract in NEAR has its [own associated account][near accounts].
When you run `npm run deploy`, your smart contract gets deployed to the live NEAR TestNet with a temporary dev account.

### Near MainNet

When you're ready to make it permanent, here's how:

## Step 0: Install near-cli (optional)

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `npm install`, but for best ergonomics you may want to install it globally:

```bash
$ npm install --global near-cli
```

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)

## Step 1: Create an account for the contract

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `near-blank-project.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `near-blank-project.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

   near login

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

   near create-account near-blank-project.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet

## Step 2: deploy the contract

Use the CLI to deploy the contract to TestNet with your account ID.
Replace `PATH_TO_WASM_FILE` with the `wasm` that was generated in `contract` build directory.

    near deploy --accountId near-blank-project.YOUR-NAME.testnet --wasmFile PATH_TO_WASM_FILE

# Troubleshooting

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.

[create-near-app]: https://github.com/near/create-near-app
[node.js]: https://nodejs.org/en/download/package-manager/
[jest]: https://jestjs.io/
[near accounts]: https://docs.near.org/concepts/basics/account
[near wallet]: https://wallet.testnet.near.org/
[near-cli]: https://github.com/near/near-cli
[gh-pages]: https://github.com/tschaub/gh-pages
