# Solana Zebec Silver Card Substreams modules

This package was initialized via `substreams init`, using the `sol-minimal` template.

## Usage

### Requirement and Setup

- Rust toolchain
- Substreams cli
- Node.js

If you want to use devcontainer then open this project in dev container when vscode suggest you to do so.
If you missed it, press `Ctrl + Shift + P` and type `Dev Containers: Open Folder in Container...`, 
it will show the matching command. Select and run this command.

Else you can run `setup-local.sh` file located in project root. It will install all necessities required for
running the substreams and subgraph project

### Substreams Authentication
If you don't have `.substreams.env` file in the root directory. Then you should create a api token
by running following command.

```bash
substreams auth
```

### Build Substreams Package

Then you have to build the project. 

```bash
substreams build
```

It will create <package name-package version>.spkg file in root directory. In our case it will be `solana_zebec_silver_card-v0.1.0`.

### Stream on a remote endpoint

Then if you want to test the substreams package you do so using following command:

```bash
substreams gui
```

To build the subgraph go to subgraph folder and follow the README present there accordingly.

## Modules

### `map_silver_card_data`

This module extract activities from transaction of siver card program such as deposits, withdraws, card purchases, etc.
and returns thier collection in a single output message

```protobuf
message Output {
  repeated Deposit deposits = 1;
  repeated Withdraw withdraws = 2;
  repeated CardPurchase card_purchases = 3;
  repeated GenerateYield generate_yields = 4;
  repeated WithdrawYield withdraw_yields = 5;
}
```
