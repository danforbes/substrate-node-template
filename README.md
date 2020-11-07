# Caliente - Substrate Hot Wallet Demonstration

The purpose of this project is to demonstrate a minimal Substrate node that can be used to implement
a secure, flexible, easy-to-use [hot wallet](https://github.com/emostov/proxy-hot-wallet#background)
architecture :hot_pepper:

## Clone

Use the following command to clone this repository and all of its submodules into a directory named
`hot-wallet-demo`:

```sh
git clone --recurse-submodules -j8 -b hot-wallet https://github.com/danforbes/substrate-node-template.git hot-wallet-demo
```

## Makefile

Refer to the included [Makefile](Makefile) to discover commands for building, testing, and
interacting with this project. Run `make init` to set-up and verify a local development
environment. Run `make run` to launch a local development node.

## Acknowledgements & References

- [Hot wallet demo](https://github.com/emostov/proxy-hot-wallet) by Zeke Mostov

## Upstream

This project was forked from the
[Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template).

## Learn More

Learn more about [Substrate](https://www.substrate.io/) at the
[Substrate Developer Hub](https://substrate.dev/).
