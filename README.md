[![Built with Substrate v2.0.0](https://img.shields.io/badge/Substrate-v2.0.0-E6007A)](https://github.com/paritytech/substrate/releases/tag/v2.0.0)

# Caliente - Substrate Hot Wallet Demonstration

The purpose of this project is to demonstrate a minimal Substrate node that can be used to implement
a secure, flexible, easy-to-use [hot wallet](https://github.com/emostov/proxy-hot-wallet#background)
architecture :hot_pepper:

The root of this repository is a project based off the
[Substrate Developer Hub Node Template](#upstream). The repository also contains two submodules:

- [`hot-wallet`](https://github.com/emostov/proxy-hot-wallet): Hot wallet demonstration code.
- [`sidecar`](https://github.com/paritytech/substrate-api-sidecar): Custom build of the Sidecar REST
  API service, which is a dependency of the hot wallet demonstration.

## Clone

Use the following command to clone this repository and all of its submodules into a directory named
`hot-wallet-demo`:

```sh
git clone --recurse-submodules -j8 -b hot-wallet https://github.com/danforbes/substrate-node-template.git hot-wallet-demo
```

## Makefile

Refer to the included [Makefile](Makefile) to discover commands for building, testing, and
interacting with this project. After cloning this repository, execute the following commands to
build all dependencies and see an end-to-end hot wallet workflow in action:

```sh
# Execute each command in a separate terminal.
make init-node
make run-sidecar
make run-demo
```

## Acknowledgements & References

- [Hot wallet demo](https://github.com/emostov/proxy-hot-wallet) by Zeke Mostov

## Upstream

This project was forked from the
[Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template).

## Learn More

Learn more about [Substrate](https://www.substrate.io/) at the
[Substrate Developer Hub](https://substrate.dev/).
