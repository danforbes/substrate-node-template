# Use Substrate to Build a Blockchain the Easy Way

The code in this repository is intended for the
[Coding Earth Global Meetup #4](https://coding.earth/meetup/ckbuzizag046l0151tivgzf0x).

## Slides

There is
[a Google Slides presentation](https://docs.google.com/presentation/d/e/2PACX-1vQqSPTCMAXrXIpaVsManYMY-k1qEFbHNcI_ydtg9mqyoyD8OlnspqYMo7XCpt66Et8xML4fPhZ8AP27/pub)
that is meant to accompany the code in this repository.

## What is Substrate?

[Substrate](https://substrate.dev/) is a [Rust](https://www.rust-lang.org/) framework for writing blockchains.

### What is a Blockchain?

A blockchain is a distributed data structure that allows a decentralized network of participants to come to consensus
about the state of a system as it evolves over time. The logic for calculating and validating the changes to this system
is referred to as the
[_state transition function_](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#stf-state-transition-function).

### Why Substrate?

Substrate is built on top of the world-class cryptographic research developed by the
[Web3 Foundation](https://web3.foundation/). The goal of Substrate is to define useful blockchain primitives and expose
a flexible, modular API for composing them in a way that is secure and scalable. A defining feature of Substrate is the
concept of "forkless upgrades", which means that a blockchain's _state transition function_ can evolve over time.

## Project Structure

A Substrate project such as this consists of a number of components that are spread across a few directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network. Substrate-based blockchain
nodes expose a number of capabilities:

- Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the nodes in the network
  to communicate with one another.
- Consensus: Blockchains must have a way to come to
  [consensus](https://substrate.dev/docs/en/knowledgebase/advanced/consensus) on the state of the network. Substrate
  makes it possible to supply custom consensus engines and also ships with several consensus mechanisms that have been
  built on top of Web3 Foundation research.
- RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory - take special note of the following:

- [`chain_spec.rs`](./node/src/chain_spec.rs): A
  [chain specification](https://substrate.dev/docs/en/knowledgebase/integrate/chain-spec) is a source code file that
  defines a Substrate chain's initial (genesis) state. Chain specifications are useful for development and testing, and
  critical when architecting the launch of a production chain. Take note of the `development_config` and
  `testnet_genesis` functions, which are used to define the genesis state for the local development chain configuration.
  These functions identify some
  [well-known accounts](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#well-known-keys) and use them to
  configure the blockchain's initial state.
- [`service.rs`](./node/src/service.rs): This file defines the node implementation. This can be considered boilerplate
  code for the purposes of this workshop, but it is useful to review nonetheless. In order to get a sense of the
  capabilities it encapsulates, take note of the libraries that this file imports and the names of the functions it
  invokes. In particular, there are references to consensus-related topics, such as the
  [longest chain rule](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#longest-chain-rule), the
  [Aura](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#aura) block authoring mechanism and the
  [GRANDPA](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#grandpa) finality gadget.

The node is the executable that is used by the [workshop](#workshop). Once the node has been [built](#build--launch),
refer to embedded documentation to learn more about the capabilities and configuration parameters that it exposes:

```shell
./target/release/node-template --help
```

### Runtime

In Substrate, the [runtime](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#runtime) is essentially
equivalent to the _state transition function_. The Substrate project in this repository uses the
[FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame) framework to construct a blockchain runtime. FRAME
allows runtime developers to declare domain-specific logic in modules called "pallets". At the heart of FRAME is a
helpful [macro language](https://substrate.dev/docs/en/knowledgebase/runtime/macros) that makes it easy to create
pallets and flexibly compose them to create blockchains that can address a variety of needs.

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this project. At this time, it is sufficient
to note the following:

- This file configures several pallets to include in the runtime. Each pallet configuration is defined by a code block
  that begins with `impl $PALLET_NAME::Trait for Runtime`.
- The pallets are composed into a single runtime by way of the
  [`construct_runtime!`](https://substrate.dev/rustdocs/v2.0.0-rc5/frame_support/macro.construct_runtime.html) macro,
  which is part of the core [FRAME Support](https://substrate.dev/docs/en/knowledgebase/runtime/frame#support-library)
  library.

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with the
[core Substrate repository](https://github.com/paritytech/substrate/tree/v2.0.0-rc5/frame) and a template pallet that is
[defined in the `pallets`](./pallets/template/src/lib.rs) directory.

A FRAME pallet is compromised of a number of blockchain primitives:

- Storage: FRAME defines a rich set of powerful
  [storage abstractions](https://substrate.dev/docs/en/knowledgebase/runtime/storage) that makes it easy to use
  Substrate's efficient key-value database to manage the evolving state of a blockchain.
- Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched) from outside of the
  runtime in order to update its state.
- Events: Substrate uses [events](https://substrate.dev/docs/en/knowledgebase/runtime/events) to notify users of
  important changes in the runtime.
- Errors: When a dispatchable fails, it returns an error.
- Trait: The `Trait` configuration interface is used to define the types and parameters upon which a FRAME pallet
  depends.

## Workshop

This workshop will guide participants through the process of modifying the provided template pallet to add a new error,
event and dispatchable, and two new storage items. Finally, the [Polkadot JS Apps UI](https://polkadot.js.org/apps), an
important part of the Substrate ecosystem, will be used to interact with these new capabilities.

The example use case this workshop encapsulates is a decentralized registry of unique pseudonymous identifiers. Users
are free to associate a pseudonymous identifier (nickname) with any
[account](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#generating-keys) they control, on the condition
that the nickname is not already being used by another account.

Before starting this workshop, complete the
[instructions for local development](https://github.com/substrate-developer-hub/substrate-node-template/tree/v2.0.0-rc4#local-development).

Use [the `cargo check` command](https://doc.rust-lang.org/cargo/commands/cargo-check.html) to check the code between
steps.

### Add a Storage Item

Two storage items will be used to implement the decentralized registry of unique pseudonymous identifiers: one will be
used to determine whether or not a given nickname has been claimed and the other to associate an account with its
nickname. Both of the new storage items will be
[_storage maps_](https://substrate.dev/rustdocs/v2.0.0-rc4/frame_support/storage/trait.StorageMap.html), which is one of
FRAME's primitive storage types (the others being
[_storage values_](https://substrate.dev/rustdocs/v2.0.0-rc4/frame_support/storage/trait.StorageValue.html) and
[_storage double maps_](https://substrate.dev/rustdocs/v2.0.0-rc4/frame_support/storage/trait.StorageDoubleMap.html)).

In this workshop, byte arrays (`Vec<u8>` in Rust) are used as nicknames. In order to provide this capability, it's
necessary to depend on the [`sp_std`](https://substrate.dev/rustdocs/v2.0.0-rc4/sp_std/index.html) module, which exposes
the [`Vec`](https://substrate.dev/rustdocs/v2.0.0-rc4/sp_std/vec/struct.Vec.html) data type, and import it into the
pallet implementation. For the purposes of this workshop, the code in this repository already includes those changes.
With that in mind, update the
[`decl_storage!`](https://substrate.dev/rustdocs/v2.0.0-rc4/frame_support/macro.decl_storage.html) block to include the
following storage item:

```rust
// Maps a nickname to whether or not it has been claimed.
Names get(fn names): map hasher(blake2_128_concat) Vec<u8> => bool;
```

Refer to the
[documentation on runtime storage](https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items)
in Substrate to learn more about the syntax of this declaration. For now, take note of the fact that this statement
declares a storage map named `Nicknames`; the type that will be used for the map's _keys_ is a byte vector (the nickname
type), the type that will be used for the map's _values_ is a boolean and will indicate whether or not a given nickname
has been claimed.

Next, add another storage map that will be used to associate account IDs with the nickname, if any, claimed by that
account holder:

```rust
// Maps an account ID to the nickname that account has claimed.
AccountNames get(fn account_names): map hasher(blake2_128_concat) T::AccountId => Vec<u8>;
```

The template pallet now has the storage capabilities that are required to support the example use case. The next step is
to add an event and an error - these will be used to communicate to users of the example application whether or not
their request to register a nickname was successful.

### Add an Event & Error

Users of decentralized systems, such as the example application, need ways to monitor the state of the system. Substrate
chains use a familiar event pattern to provide this capability. In FRAME, events are defined as simple enums. These
enums encapsulate data that is emitted by runtime processes and become part of the blockchain's state. Blockchain users
can subscribe to events by way of a Substrate node's RPC server.

Update the [`decl_event!`](https://substrate.dev/rustdocs/v2.0.0-rc4/frame_support/macro.decl_event.html) block to
include the following event:

```rust
/// A nickname has been claimed. [nickname, claimant]
NameClaimed(Vec<u8>, AccountId),
```

This event will signify that a nickname has been claimed - it encapsulates two pieces of data: the nickname that has
been claimed and the ID of the account that claimed it.

Whereas events are used to indicate a successful state transition and describe the changes that were made, errors are
used to indicate that a request to modify the blockchain's state has failed and provide details about the reason for the
failure.

Update the [`decl_error!`](https://substrate.dev/rustdocs/v2.0.0-rc4/frame_support/macro.decl_error.html) block to
include the following error:

```rust
/// A nickname is not available because it has already been claimed.
NameNotAvailable,
```

The purpose of this error is to indicate that a request to claim a nickname has failed because that nickname has already
been claimed by another account.

The next step will demonstrate how to add a dispatchable function to the template pallet. This function will be used to
interact with the components defined in the previous steps.

### Add a Dispatchable

Dispatchable functions are special types of functions defined within a pallet - they encapsulate logic that can be
invoked by blockchain users and can be used to update the blockchain's state. Dispatchable functions are defined in the
`Module` struct within the
[`decl_module!`](https://substrate.dev/rustdocs/v2.0.0-rc4/frame_support/macro.decl_module.html#declaration) macro; they
must have a [`weight`](https://substrate.dev/docs/en/knowledgebase/runtime/fees#default-weight-annotations) annotation
and return a
[`DispatchResult`](https://substrate.dev/rustdocs/v2.0.0-rc4/frame_support/dispatch/type.DispatchResult.html).

Add the following dispatchable function to the template pallet:

```rust
/// Register a nickname. Will return a NameNotAvailable error if the nickname has already
/// been registered. Will emit a NameClaimed event on success.
///
/// The dispatch origin for this call must be Signed.
///
/// # <weight>
/// - `O(1)`
/// - 1 storage read.
/// - 2 storage writes.
/// - DB Weight:
///     - Read: Names
///     - Writes: Names, Account Names
/// # </weight>
#[weight = 10_000 + T::DbWeight::get().reads_writes(1,2)]
pub fn register_name(origin, name: Vec<u8>) -> dispatch::DispatchResult {
  // Check that the dispatch was signed and get the signer.
  let who = ensure_signed(origin)?;
  // Check that the nickname has not already been claimed.
  // Emit an error if the nickname has been claimed.
  ensure!(!Names::contains_key(&name), Error::<T>::NameNotAvailable);

  // Register the nickname by updating storage.
  Names::insert(&name, true);
  AccountNames::<T>::insert(&who, &name);

  // Emit an event to report the details of the state transition.
  Self::deposit_event(RawEvent::NameClaimed(name, who));
  Ok(())
}
```

In order to use the helpful [`ensure!`](https://substrate.dev/rustdocs/v2.0.0-rc5/frame_support/macro.ensure.html)
macro, it's necessary to import it from the `frame_support` module. For the purposes of this workshop, it has already
been imported.

Refer to the inline comments for a description of the function's logic. Note that this dispatchable function adheres to
the important
[verify-first-write-last](https://substrate.dev/docs/en/knowledgebase/runtime/storage#verify-first-write-last) maxim.

### Build & Launch

Use the following command to build a blockchain node to host the custom runtime logic:

```shell
cargo build --release
```

Purge any existing chain state (accept the prompt by entering `y`):

```shell
./target/release/node-template purge-chain --dev
```

Launch the chain in the development configuration with this command:

```shell
./target/release/node-template --dev
```

### Interact with the Custom Chain

Use the Polkadot JS Apps UI to connect to the blockchain node started in the previous step; supply the `rpc` URL
parameter to instruct the UI to connect to the local node:
`https://polkadot.js.org/apps/#/extrinsics?rpc=ws://127.0.0.1:9944`. This link points to the "Extrinsics" app, which can
be used to invoke dispatchable functions.

Complete the Extrinsics app form as follows:

- Account: Alice
- Module: `templateModule`
- Dispatchable: `registerName`
- Name: `0x1983`

Any valid hexadecimal value that represents a byte array can be used for the "Name" value.

Click the "Submit Transaction" button and then the "Sign and Submit" button on the popup; the transaction should
succeed.

Switch to Bob's account and sign and submit the transaction without modifying anything else. The transaction should fail
and report a `NameNotAvailable` error. Change the value of the Name parameter and sign and submit the transaction a
final time; the transaction should succeed.

Use the "Chain State" app to read the storage values of the template module that were set when the nicknames were
registered.

## Learn More

The scope of this workshop has been fairly limited: add a simple capability to an existing FRAME pallet. Here are some
suggestions for next steps:

- [Add a Pallet Tutorial](https://substrate.dev/docs/en/tutorials/add-a-pallet-to-your-runtime/) - This tutorial
  demonstrates how to extend the capabilities of a Substrate blockchain by adding a pallet from the library of
  [core FRAME pallets](https://substrate.dev/docs/en/knowledgebase/runtime/frame#prebuilt-pallets).
- [Create a Pallet Tutorial](https://substrate.dev/docs/en/tutorials/create-a-pallet/) - This tutorial guides
  participants through the process of writing a new custom FRAME pallet.
- Extend this tutorial - Here are some suggestions for how this tutorial can be extended:
  - Enforce a [limit](https://substrate.dev/docs/en/knowledgebase/runtime/storage#create-bounds) on nickname length.
  - Add new dispatchable functions to allow users to relinquish and transfer nicknames.
  - Use the [front-end template](https://github.com/substrate-developer-hub/substrate-front-end-template) to create a
    custom UI component.

## Upstream

This project was forked from the
[Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template/tree/v2.0.0-rc4).
