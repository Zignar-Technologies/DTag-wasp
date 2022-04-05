// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_mut)]

use wasmlib::*;

pub struct giveawayEvents {
}

impl giveawayEvents {

	pub fn winner(&self, evm_address: &str) {
		let mut evt = EventEncoder::new("giveaway.winner");
		evt.encode(&string_to_string(&evm_address));
		evt.emit();
	}
}
