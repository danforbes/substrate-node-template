[![Built with Substrate v2.0.0](https://img.shields.io/badge/Substrate-v2.0.0-E6007A)](https://github.com/paritytech/substrate/releases/tag/v2.0.0)

# Dumbo - FRAME Origin Demonstration

The purpose of this project is to explore
[FRAME's origin primitive](https://substrate.dev/docs/en/knowledgebase/runtime/origin) in order to
learn and educate. The name is a playful reference to the
[Sudo pallet](https://github.com/paritytech/substrate/tree/master/frame/sudo) :elephant:

## FRAME's Origin Primitive

In FRAME, the term "origin" refers to the source of a runtime call. The origin primitive is one of
the most important types in the FRAME system for runtime development because it supports both
authentication and authorization capabilities. The FRAME runtime system defines three core origins:

- `Root` - This is a system-level origin that represents the concept of the runtime administrator.
  This is analogous to the Linux operating system's
  [superuser account](https://en.wikipedia.org/wiki/Superuser). The FRAME system provides mechanisms
  to allow privileged users or on-chain governance bodies to proxy a call for dispatch from the
  `Root` origin. The most evocative use of this origin is to perform
  [runtime upgrades](https://substrate.dev/rustdocs/v2.0.0/frame_system/enum.Call.html#variant.set_code)
  (it is the only origin that is allowed to do so), but there are other privileged functions that
  are only accessible from the `Root` origin.
- `Signed` - The signed origin represents a dispatch that originates from some on-chain account and
  is signed by its private key. This allows the runtime system to securely authenticate the account
  that invoked the dispatch and charge it transaction fees. The account ID is accessible from within
  runtime modules, which makes it possible to easily implement flexible authorization logic.
- `None` - This origin is used for unsigned transactions or to allow validators to insert data
  directly into a block. In either case, the runtime system is not able to charge fees for calls
  from this origin since it is not able to attribute the call to a source. In the case of unsigned
  transactions, it is the responsibility of the runtime module to validate and account for the call
  and the resources it consumes. In the case of data inserted by block authors, there must exist
  mechanisms by which other validators can come to agreement on the validity of the data.

## Custom Origins

Runtime developers may define custom origins in addition to the three core origins described above.
This project contains two pallets, one of which defines a custom origin and a dispatchable function
that makes it possible to dispatch a call from the custom origin, and another pallet that defines a
call that may only be dispatched by the custom origin.

### Key Takeaways & Gotchas

- Defining custom origins relies on help from the `construct_runtime!` macro.
- It is not trivial to define an origin _and_ a dispatch that may only be called by that origin in
  the same pallet (this is because of the reliance on help from `construct_runtime`).
- In order to use the `hex-literal` crate in the runtime it must be removed as a dependency when
  building with the `runtime-benchmarks` feature flag. In order to run the benchmarks, it will be
  necessary to reconfigure the Cargo.toml file.

### Dumbo Pallet

The Dumbo pallet defines the custom `Dumbo` origin as well as a dispatchable function,
`proxy_dumbo`, that allows an authorized account to proxy a call from `Dumbo`. The authorized
account is configured by way of the `pallet_dumbo::Trait::DumboAccountId` type. If `proxy_dumbo` is
called by an unauthorized account, a `BadOrigin` error will be emitted. This pallet also defines an
`EnsureDumbo` struct that implements the
[`EnsureOrigin` trait](https://substrate.dev/rustdocs/v2.0.0/frame_support/traits/trait.EnsureOrigin.html);
it may be used by other pallets to ensure the origin of a dispatch is the custom `Dumbo` origin.

Notice that the Dumbo pallet's `Trait` configuration trait specifies the `Call` and `Event` types.
Furthermore, notice that when the Dumbo pallet is included in the `construct_module!` macro it
includes the `Origin` component.

### Peanut Gallery Pallet

The Peanut Gallery pallet defines a privileged dispatchable function, `dumbo_wants_peanuts`, that
may only be called by the custom `Dumbo` origin defined in the Dumbo pallet. In order to provide
this access control capability, the Peanut Gallery depends on a `DumboOrigin` type and is configured
to use the `EnsureDumbo` struct to satisfy this dependency. If `dumbo_wants_peanuts` is called by
the `Dumbo` origin, a `DumboWantsPeanuts` event will be emitted; if a non-authorized account calls
this function, a `BadOrigin` error will be emitted.

## Learn More About Origins

It is possible to define even more powerful types of origins, such as those used by the FRAME
[Collective pallet](https://substrate.dev/rustdocs/v2.0.0/pallet_collective/enum.RawOrigin.html).
These origins specify the _threshold_ by which some proposal passed and allows runtime developers to
implement even more flexible and robust access control logic.

The
[Substrate Enterprise Sample](https://github.com/substrate-developer-hub/substrate-enterprise-sample)
makes extensive use of the origin primitive in order to implement a realistic supply chain use case.

## Acknowledgements

Thanks to [Shawn Tabrizi](https://github.com/shawntabrizi/) for helping me figure out some of the
intricacies of defining custom FRAME origins.

## Upstream

This project was forked from the
[Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template).

## Learn More About Substrate

Learn more about [Substrate](https://www.substrate.io/) at the
[Substrate Developer Hub](https://substrate.dev/).
