// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package rstemplates

var cargoToml = map[string]string{
	// *******************************
	"Cargo.toml": `
# Copyright 2020 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

[package]
name = "$package"
description = "$scDesc"
license = "Apache-2.0"
version = "0.1.0"
authors = ["John Doe <john@doe.org>"]
edition = "2018"
repository = "https://github.com/iotaledger/wasp"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = ["console_error_panic_hook"]

[dependencies]
wasmlib = { git = "https://github.com/iotaledger/wasp", branch = "develop" }
console_error_panic_hook = { version = "0.1.6", optional = true }
wee_alloc = { version = "0.4.5", optional = true }

[dev-dependencies]
wasm-bindgen-test = "0.3.13"
`,
}
