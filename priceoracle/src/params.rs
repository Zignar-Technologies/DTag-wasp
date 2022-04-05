// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;
use crate::*;

#[derive(Clone)]
pub struct ImmutableApproveOracleParams {
	pub(crate) proxy: Proxy,
}

impl ImmutableApproveOracleParams {
    pub fn agentid(&self) -> ScImmutableAgentID {
		ScImmutableAgentID::new(self.proxy.root(PARAM_AGENTID))
	}
}

#[derive(Clone)]
pub struct MutableApproveOracleParams {
	pub(crate) proxy: Proxy,
}

impl MutableApproveOracleParams {
    pub fn agentid(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.proxy.root(PARAM_AGENTID))
	}
}

#[derive(Clone)]
pub struct ImmutableInitParams {
	pub(crate) proxy: Proxy,
}

impl ImmutableInitParams {
    pub fn owner(&self) -> ScImmutableAgentID {
		ScImmutableAgentID::new(self.proxy.root(PARAM_OWNER))
	}
}

#[derive(Clone)]
pub struct MutableInitParams {
	pub(crate) proxy: Proxy,
}

impl MutableInitParams {
    pub fn owner(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.proxy.root(PARAM_OWNER))
	}
}

#[derive(Clone)]
pub struct ImmutableSetMiotaPriceParams {
	pub(crate) proxy: Proxy,
}

impl ImmutableSetMiotaPriceParams {
    pub fn price(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.proxy.root(PARAM_PRICE))
	}
}

#[derive(Clone)]
pub struct MutableSetMiotaPriceParams {
	pub(crate) proxy: Proxy,
}

impl MutableSetMiotaPriceParams {
    pub fn price(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.proxy.root(PARAM_PRICE))
	}
}

#[derive(Clone)]
pub struct ImmutableSetOwnerParams {
	pub(crate) proxy: Proxy,
}

impl ImmutableSetOwnerParams {
    pub fn owner(&self) -> ScImmutableAgentID {
		ScImmutableAgentID::new(self.proxy.root(PARAM_OWNER))
	}
}

#[derive(Clone)]
pub struct MutableSetOwnerParams {
	pub(crate) proxy: Proxy,
}

impl MutableSetOwnerParams {
    pub fn owner(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.proxy.root(PARAM_OWNER))
	}
}
