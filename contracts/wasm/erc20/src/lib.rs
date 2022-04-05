// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use erc20::*;
use wasmlib::*;

use crate::consts::*;
use crate::events::*;
use crate::params::*;
use crate::results::*;
use crate::state::*;
use crate::typedefs::*;

mod consts;
mod contract;
mod events;
mod params;
mod results;
mod state;
mod typedefs;

mod erc20;

const EXPORT_MAP: ScExportMap = ScExportMap {
    names: &[
    	FUNC_APPROVE,
    	FUNC_INIT,
    	FUNC_TRANSFER,
    	FUNC_TRANSFER_FROM,
    	VIEW_ALLOWANCE,
    	VIEW_BALANCE_OF,
    	VIEW_TOTAL_SUPPLY,
	],
    funcs: &[
    	func_approve_thunk,
    	func_init_thunk,
    	func_transfer_thunk,
    	func_transfer_from_thunk,
	],
    views: &[
    	view_allowance_thunk,
    	view_balance_of_thunk,
    	view_total_supply_thunk,
	],
};

#[no_mangle]
fn on_call(index: i32) {
	ScExports::call(index, &EXPORT_MAP);
}

#[no_mangle]
fn on_load() {
    ScExports::export(&EXPORT_MAP);
}

pub struct ApproveContext {
	events:  Erc20Events,
	params: ImmutableApproveParams,
	state: MutableErc20State,
}

fn func_approve_thunk(ctx: &ScFuncContext) {
	ctx.log("erc20.funcApprove");
	let f = ApproveContext {
		events:  Erc20Events {},
		params: ImmutableApproveParams { proxy: params_proxy() },
		state: MutableErc20State { proxy: state_proxy() },
	};
	ctx.require(f.params.amount().exists(), "missing mandatory amount");
	ctx.require(f.params.delegation().exists(), "missing mandatory delegation");
	func_approve(ctx, &f);
	ctx.log("erc20.funcApprove ok");
}

pub struct InitContext {
	events:  Erc20Events,
	params: ImmutableInitParams,
	state: MutableErc20State,
}

fn func_init_thunk(ctx: &ScFuncContext) {
	ctx.log("erc20.funcInit");
	let f = InitContext {
		events:  Erc20Events {},
		params: ImmutableInitParams { proxy: params_proxy() },
		state: MutableErc20State { proxy: state_proxy() },
	};
	ctx.require(f.params.creator().exists(), "missing mandatory creator");
	ctx.require(f.params.supply().exists(), "missing mandatory supply");
	func_init(ctx, &f);
	ctx.log("erc20.funcInit ok");
}

pub struct TransferContext {
	events:  Erc20Events,
	params: ImmutableTransferParams,
	state: MutableErc20State,
}

fn func_transfer_thunk(ctx: &ScFuncContext) {
	ctx.log("erc20.funcTransfer");
	let f = TransferContext {
		events:  Erc20Events {},
		params: ImmutableTransferParams { proxy: params_proxy() },
		state: MutableErc20State { proxy: state_proxy() },
	};
	ctx.require(f.params.account().exists(), "missing mandatory account");
	ctx.require(f.params.amount().exists(), "missing mandatory amount");
	func_transfer(ctx, &f);
	ctx.log("erc20.funcTransfer ok");
}

pub struct TransferFromContext {
	events:  Erc20Events,
	params: ImmutableTransferFromParams,
	state: MutableErc20State,
}

fn func_transfer_from_thunk(ctx: &ScFuncContext) {
	ctx.log("erc20.funcTransferFrom");
	let f = TransferFromContext {
		events:  Erc20Events {},
		params: ImmutableTransferFromParams { proxy: params_proxy() },
		state: MutableErc20State { proxy: state_proxy() },
	};
	ctx.require(f.params.account().exists(), "missing mandatory account");
	ctx.require(f.params.amount().exists(), "missing mandatory amount");
	ctx.require(f.params.recipient().exists(), "missing mandatory recipient");
	func_transfer_from(ctx, &f);
	ctx.log("erc20.funcTransferFrom ok");
}

pub struct AllowanceContext {
	params: ImmutableAllowanceParams,
	results: MutableAllowanceResults,
	state: ImmutableErc20State,
}

fn view_allowance_thunk(ctx: &ScViewContext) {
	ctx.log("erc20.viewAllowance");
	let f = AllowanceContext {
		params: ImmutableAllowanceParams { proxy: params_proxy() },
		results: MutableAllowanceResults { proxy: results_proxy() },
		state: ImmutableErc20State { proxy: state_proxy() },
	};
	ctx.require(f.params.account().exists(), "missing mandatory account");
	ctx.require(f.params.delegation().exists(), "missing mandatory delegation");
	view_allowance(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("erc20.viewAllowance ok");
}

pub struct BalanceOfContext {
	params: ImmutableBalanceOfParams,
	results: MutableBalanceOfResults,
	state: ImmutableErc20State,
}

fn view_balance_of_thunk(ctx: &ScViewContext) {
	ctx.log("erc20.viewBalanceOf");
	let f = BalanceOfContext {
		params: ImmutableBalanceOfParams { proxy: params_proxy() },
		results: MutableBalanceOfResults { proxy: results_proxy() },
		state: ImmutableErc20State { proxy: state_proxy() },
	};
	ctx.require(f.params.account().exists(), "missing mandatory account");
	view_balance_of(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("erc20.viewBalanceOf ok");
}

pub struct TotalSupplyContext {
	results: MutableTotalSupplyResults,
	state: ImmutableErc20State,
}

fn view_total_supply_thunk(ctx: &ScViewContext) {
	ctx.log("erc20.viewTotalSupply");
	let f = TotalSupplyContext {
		results: MutableTotalSupplyResults { proxy: results_proxy() },
		state: ImmutableErc20State { proxy: state_proxy() },
	};
	view_total_supply(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("erc20.viewTotalSupply ok");
}
