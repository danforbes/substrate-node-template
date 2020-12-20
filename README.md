[![Built with Substrate v2.0.0](https://img.shields.io/badge/Substrate-v2.0.0-E6007A)](https://github.com/paritytech/substrate/releases/tag/v2.0.0)

# Sydney - Decentralized / Self-Sovereign Identity with Substrate

The purpose of this project is to explore decentralized identity (DID) on Substrate-based chains in
order to learn and educate. The name is a reference to the acronym SSID, which is used to refer to
the concept of self-sovereign identity 🐨

Sydney is built on the Substrate Developer Hub
[DID pallet](https://github.com/substrate-developer-hub/pallet-did).

## Background

The [Decentralized Identifier specification](https://w3c.github.io/did-core/) is maintained by the
[World Wide Web Consortium](https://www.w3.org/). It describes a new way for people to interact with
each other and with centers of trust (like government agencies, educational institutes, and social
networks). In order to achieve this goal, the DID specification depends on the permissionless,
decentralized storage capabilities of a blockchain. According to the charter of the
[Decentralized Identifier Working Group](https://www.w3.org/2019/09/did-wg-charter.html), DID
protocols enable the next-generation of digital identity management, like
[Verifiable Credentials](https://www.w3.org/TR/vc-data-model/) and [WebAuthn](https://webauthn.io/).
Even large enterprises like Microsoft are
[investigating](https://query.prod.cms.rt.microsoft.com/cms/api/am/binary/RE2DjfY) applications of
decentralized identity.

This project is just to learn and experiment. If you want to see what production-grade decentralized
identity on Substrate looks like, check out [Dock](https://www.dock.io/) and
[KILT Protocol](https://www.kilt.io/).

## Sydney DID Method Specification

A [DID method](https://w3c.github.io/did-core/#dfn-did-methods) is a definition of how a specific
[DID scheme](https://w3c.github.io/did-core/#dfn-did-schemes), or specification, must be implemented
to work with a specific
[verifiable data registry](https://w3c.github.io/did-core/#dfn-verifiable-data-registry), which is a
service that enables interactions involving DIDs.

### Method Name

The Sydney DID scheme is identified by the method name `did:sydny`.

### Method Specific ID

Substrate
[account ID](https://substrate.dev/rustdocs/v2.0.0/frame_system/trait.Trait.html#associatedtype.AccountId)s
are used to uniquely identify Sydney identities.

This means that the Sydney ID for the
[well-known](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#well-known-keys) Alice
identity is `did:sydny:5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY`.

### Method Operations

This section describes the [operations](https://w3c.github.io/did-core/#method-operations) that can
be performed on a Sydney DID document.

#### Create

In order to [create](https://w3c.github.io/did-core/#create) a Sydney a ID, create an attribute for
that ID with the name `Claimed`.

## Upstream

This project was forked from the
[Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template).

## Learn More About Substrate

Learn more about [Substrate](https://www.substrate.io/) at the
[Substrate Developer Hub](https://substrate.dev/).
