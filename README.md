# ICP Rust Bootcamp - Proposal/Vote System Canister

This project implements a simple proposal system smart contract on the Internet Computer blockchain. Users can create, edit, end, and vote on proposals, with a query function to determine the status of a proposal based on community votes.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Query Functions](#query-functions)
- [Update Functions](#update-functions)
- [Proposal Status](#proposal-status)
- [Contributing](#contributing)
- [Run-deploy-test](#run-deploy-test)

## Overview

The smart contract is written in Rust and uses the Candid interface description language for interacting with the Internet Computer. It utilizes the `ic_stable_structures` library for stable data structures and memory management.

## Features

- Create, edit, end, and vote on proposals.
- Query the details of a proposal.
- Get the total count of proposals.
- Determine the status of a proposal based on community votes.

## Getting Started

To deploy and interact with the smart contract, follow these steps:

1. Clone the repository.
2. Install the necessary dependencies.
3. Build and deploy the smart contract.

## Usage

Describe how users can deploy the smart contract and interact with its functions.

## Query Functions

### `get_proposal(key: u64) -> Option<Proposal>`

Retrieve the details of a proposal based on the provided key.

### `get_proposal_count() -> u64`

Get the total count of proposals.

### `get_proposal_status(key: u64) -> String`

Determine the status of a proposal based on community votes.

## Update Functions

### `create_proposal(key: u64, proposal: CreateProposal) -> Option<Proposal>`

Create a new proposal.

### `edit_proposal(key: u64, proposal: CreateProposal) -> Result<(), VoteError>`

Edit an existing proposal.

### `end_proposal(key: u64) -> Result<(), VoteError>`

End an active proposal.

### `vote(key: u64, choice: VoteTypes) -> Result<(), VoteError>`

Vote on a proposal.

## Proposal Status

The `get_proposal_status` query function calculates the status of a proposal. It declares the proposal as 'Approved,' 'Rejected,' 'Passed,' or 'Undecided' based on the votes.

- A proposal must have at least 5 votes to be eligible for evaluation.
- A status is only assigned if it earns at least 50% of the votes.

## Contributing

Describe how others can contribute to the project.

## Run-deploy-test

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd vote/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
