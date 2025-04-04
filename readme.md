# VM2 @ CEP-78 Enhanced NFT
An implementation of the [CEP-78 Enhanced NFT](https://github.com/casper-network/ceps/blob/master/text/0078-enhanced-nft-standard.md) standard for the proposed [VM2](https://github.com/casper-network/casper-node/pull/4806) of casper-node.

> [!WARNING]
> This is a work in progress. The code, APIs, and tooling may change at any time without notice.

This project is under active development, with some features still in progress. Specifically, metadata validation and owner-reverse-lookup-mode are not yet implemented. Despite these gaps, this implementation offers valuable insights into the potential structure of VM2 contracts and areas within the VM that may require improvement.

# Tools

You need custom tools installed from a git repo.

```sh
# cargo-casper
cargo install --git https://github.com/casper-node/casper-node --branch dev cargo-casper
```

# Building the contract

```sh
cargo-casper build
```

This builds both .wasm and .json schema files.

# Contract schema

To obtain schema for this contract you can run:

```
cargo-casper build-schema
```
