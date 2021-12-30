// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use dtag::*;
use wasmlib::*;
use wasmlib::host::*;

use crate::consts::*;
use crate::keys::*;
use crate::params::*;
use crate::results::*;
use crate::state::*;

mod consts;
mod contract;
mod keys;
mod params;
mod results;
mod state;
mod structs;
mod dtag;

#[no_mangle]
fn on_load() {
    let exports = ScExports::new();
    exports.add_func(FUNC_CREATE_GAME,         func_create_game_thunk);
    exports.add_func(FUNC_END_GAME,            func_end_game_thunk);
    exports.add_func(FUNC_INIT,                func_init_thunk);
    exports.add_func(FUNC_REQUEST_PLAY,        func_request_play_thunk);
    exports.add_func(FUNC_SEND_TAGS,           func_send_tags_thunk);
    exports.add_func(FUNC_SET_OWNER,           func_set_owner_thunk);
    exports.add_func(FUNC_WITHDRAW,            func_withdraw_thunk);
    exports.add_view(VIEW_GET_OWNER,           view_get_owner_thunk);
    exports.add_view(VIEW_GET_PLAYER_BETS,     view_get_player_bets_thunk);
    exports.add_view(VIEW_GET_PLAYS_PER_IMAGE, view_get_plays_per_image_thunk);
    exports.add_view(VIEW_GET_RESULTS,         view_get_results_thunk);

    unsafe {
        for i in 0..KEY_MAP_LEN {
            IDX_MAP[i] = get_key_id_from_string(KEY_MAP[i]);
        }
    }
}

pub struct CreateGameContext {
	params: ImmutableCreateGameParams,
	state: MutabledtagState,
}

fn func_create_game_thunk(ctx: &ScFuncContext) {
	ctx.log("dtag.funcCreateGame");
	let f = CreateGameContext {
		params: ImmutableCreateGameParams {
			id: OBJ_ID_PARAMS,
		},
		state: MutabledtagState {
			id: OBJ_ID_STATE,
		},
	};
	ctx.require(f.params.description().exists(), "missing mandatory description");
	ctx.require(f.params.number_of_images().exists(), "missing mandatory numberOfImages");
	func_create_game(ctx, &f);
	ctx.log("dtag.funcCreateGame ok");
}

pub struct EndGameContext {
	state: MutabledtagState,
}

fn func_end_game_thunk(ctx: &ScFuncContext) {
	ctx.log("dtag.funcEndGame");
	let f = EndGameContext {
		state: MutabledtagState {
			id: OBJ_ID_STATE,
		},
	};
	func_end_game(ctx, &f);
	ctx.log("dtag.funcEndGame ok");
}

pub struct InitContext {
	params: ImmutableInitParams,
	state: MutabledtagState,
}

fn func_init_thunk(ctx: &ScFuncContext) {
	ctx.log("dtag.funcInit");
	let f = InitContext {
		params: ImmutableInitParams {
			id: OBJ_ID_PARAMS,
		},
		state: MutabledtagState {
			id: OBJ_ID_STATE,
		},
	};
	func_init(ctx, &f);
	ctx.log("dtag.funcInit ok");
}

pub struct RequestPlayContext {
	results: MutableRequestPlayResults,
	state: MutabledtagState,
}

fn func_request_play_thunk(ctx: &ScFuncContext) {
	ctx.log("dtag.funcRequestPlay");
	let f = RequestPlayContext {
		results: MutableRequestPlayResults {
			id: OBJ_ID_RESULTS,
		},
		state: MutabledtagState {
			id: OBJ_ID_STATE,
		},
	};
	func_request_play(ctx, &f);
	ctx.log("dtag.funcRequestPlay ok");
}

pub struct SendTagsContext {
	params: ImmutableSendTagsParams,
	state: MutabledtagState,
}

fn func_send_tags_thunk(ctx: &ScFuncContext) {
	ctx.log("dtag.funcSendTags");
	let f = SendTagsContext {
		params: ImmutableSendTagsParams {
			id: OBJ_ID_PARAMS,
		},
		state: MutabledtagState {
			id: OBJ_ID_STATE,
		},
	};
	ctx.require(f.params.boost().exists(), "missing mandatory boost");
	ctx.require(f.params.h().exists(), "missing mandatory h");
	ctx.require(f.params.w().exists(), "missing mandatory w");
	ctx.require(f.params.x().exists(), "missing mandatory x");
	ctx.require(f.params.y().exists(), "missing mandatory y");
	func_send_tags(ctx, &f);
	ctx.log("dtag.funcSendTags ok");
}

pub struct SetOwnerContext {
	params: ImmutableSetOwnerParams,
	state: MutabledtagState,
}

fn func_set_owner_thunk(ctx: &ScFuncContext) {
	ctx.log("dtag.funcSetOwner");

	// current owner of this smart contract
	let access = ctx.state().get_agent_id("owner");
	ctx.require(access.exists(), "access not set: owner");
	ctx.require(ctx.caller() == access.value(), "no permission");

	let f = SetOwnerContext {
		params: ImmutableSetOwnerParams {
			id: OBJ_ID_PARAMS,
		},
		state: MutabledtagState {
			id: OBJ_ID_STATE,
		},
	};
	ctx.require(f.params.owner().exists(), "missing mandatory owner");
	func_set_owner(ctx, &f);
	ctx.log("dtag.funcSetOwner ok");
}

pub struct WithdrawContext {
	state: MutabledtagState,
}

fn func_withdraw_thunk(ctx: &ScFuncContext) {
	ctx.log("dtag.funcWithdraw");

	// current owner of this smart contract
	let access = ctx.state().get_agent_id("owner");
	ctx.require(access.exists(), "access not set: owner");
	ctx.require(ctx.caller() == access.value(), "no permission");

	let f = WithdrawContext {
		state: MutabledtagState {
			id: OBJ_ID_STATE,
		},
	};
	func_withdraw(ctx, &f);
	ctx.log("dtag.funcWithdraw ok");
}

pub struct GetOwnerContext {
	results: MutableGetOwnerResults,
	state: ImmutabledtagState,
}

fn view_get_owner_thunk(ctx: &ScViewContext) {
	ctx.log("dtag.viewGetOwner");
	let f = GetOwnerContext {
		results: MutableGetOwnerResults {
			id: OBJ_ID_RESULTS,
		},
		state: ImmutabledtagState {
			id: OBJ_ID_STATE,
		},
	};
	view_get_owner(ctx, &f);
	ctx.log("dtag.viewGetOwner ok");
}

pub struct GetPlayerBetsContext {
	results: MutableGetPlayerBetsResults,
	state: ImmutabledtagState,
}

fn view_get_player_bets_thunk(ctx: &ScViewContext) {
	ctx.log("dtag.viewGetPlayerBets");
	let f = GetPlayerBetsContext {
		results: MutableGetPlayerBetsResults {
			id: OBJ_ID_RESULTS,
		},
		state: ImmutabledtagState {
			id: OBJ_ID_STATE,
		},
	};
	view_get_player_bets(ctx, &f);
	ctx.log("dtag.viewGetPlayerBets ok");
}

pub struct GetPlaysPerImageContext {
	params: ImmutableGetPlaysPerImageParams,
	results: MutableGetPlaysPerImageResults,
	state: ImmutabledtagState,
}

fn view_get_plays_per_image_thunk(ctx: &ScViewContext) {
	ctx.log("dtag.viewGetPlaysPerImage");
	let f = GetPlaysPerImageContext {
		params: ImmutableGetPlaysPerImageParams {
			id: OBJ_ID_PARAMS,
		},
		results: MutableGetPlaysPerImageResults {
			id: OBJ_ID_RESULTS,
		},
		state: ImmutabledtagState {
			id: OBJ_ID_STATE,
		},
	};
	ctx.require(f.params.image_id().exists(), "missing mandatory imageId");
	view_get_plays_per_image(ctx, &f);
	ctx.log("dtag.viewGetPlaysPerImage ok");
}

pub struct GetResultsContext {
	params: ImmutableGetResultsParams,
	results: MutableGetResultsResults,
	state: ImmutabledtagState,
}

fn view_get_results_thunk(ctx: &ScViewContext) {
	ctx.log("dtag.viewGetResults");
	let f = GetResultsContext {
		params: ImmutableGetResultsParams {
			id: OBJ_ID_PARAMS,
		},
		results: MutableGetResultsResults {
			id: OBJ_ID_RESULTS,
		},
		state: ImmutabledtagState {
			id: OBJ_ID_STATE,
		},
	};
	ctx.require(f.params.image_id().exists(), "missing mandatory imageId");
	view_get_results(ctx, &f);
	ctx.log("dtag.viewGetResults ok");
}
