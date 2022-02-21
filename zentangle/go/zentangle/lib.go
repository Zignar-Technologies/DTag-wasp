// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

//nolint:dupl
package zentangle

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

func OnLoad() {
	exports := wasmlib.NewScExports()
	exports.AddFunc(FuncCreateGame,       funcCreateGameThunk)
	exports.AddFunc(FuncEndGame,          funcEndGameThunk)
	exports.AddFunc(FuncInit,             funcInitThunk)
	exports.AddFunc(FuncRequestPlay,      funcRequestPlayThunk)
	exports.AddFunc(FuncSendTags,         funcSendTagsThunk)
	exports.AddFunc(FuncSetOwner,         funcSetOwnerThunk)
	exports.AddFunc(FuncWithdraw,         funcWithdrawThunk)
	exports.AddView(ViewGetOwner,         viewGetOwnerThunk)
	exports.AddView(ViewGetPlayerBets,    viewGetPlayerBetsThunk)
	exports.AddView(ViewGetPlayerInfo,    viewGetPlayerInfoThunk)
	exports.AddView(ViewGetPlaysPerImage, viewGetPlaysPerImageThunk)
	exports.AddView(ViewGetResults,       viewGetResultsThunk)

	for i, key := range keyMap {
		idxMap[i] = key.KeyID()
	}
}

type CreateGameContext struct {
	Params  ImmutableCreateGameParams
	State   MutablezentangleState
}

func funcCreateGameThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("zentangle.funcCreateGame")
	f := &CreateGameContext{
		Params: ImmutableCreateGameParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		State: MutablezentangleState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	ctx.Require(f.Params.Description().Exists(), "missing mandatory description")
	ctx.Require(f.Params.NumberOfImages().Exists(), "missing mandatory numberOfImages")
	funcCreateGame(ctx, f)
	ctx.Log("zentangle.funcCreateGame ok")
}

type EndGameContext struct {
	Params  ImmutableEndGameParams
	State   MutablezentangleState
}

func funcEndGameThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("zentangle.funcEndGame")
	f := &EndGameContext{
		Params: ImmutableEndGameParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		State: MutablezentangleState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	funcEndGame(ctx, f)
	ctx.Log("zentangle.funcEndGame ok")
}

type InitContext struct {
	Params  ImmutableInitParams
	State   MutablezentangleState
}

func funcInitThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("zentangle.funcInit")
	f := &InitContext{
		Params: ImmutableInitParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		State: MutablezentangleState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	funcInit(ctx, f)
	ctx.Log("zentangle.funcInit ok")
}

type RequestPlayContext struct {
	Results MutableRequestPlayResults
	State   MutablezentangleState
}

func funcRequestPlayThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("zentangle.funcRequestPlay")
	f := &RequestPlayContext{
		Results: MutableRequestPlayResults{
			id: wasmlib.OBJ_ID_RESULTS,
		},
		State: MutablezentangleState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	funcRequestPlay(ctx, f)
	ctx.Log("zentangle.funcRequestPlay ok")
}

type SendTagsContext struct {
	Params  ImmutableSendTagsParams
	State   MutablezentangleState
}

func funcSendTagsThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("zentangle.funcSendTags")
	f := &SendTagsContext{
		Params: ImmutableSendTagsParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		State: MutablezentangleState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	ctx.Require(f.Params.InputJson().Exists(), "missing mandatory inputJson")
	funcSendTags(ctx, f)
	ctx.Log("zentangle.funcSendTags ok")
}

type SetOwnerContext struct {
	Params  ImmutableSetOwnerParams
	State   MutablezentangleState
}

func funcSetOwnerThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("zentangle.funcSetOwner")

	// current owner of this smart contract
	access := ctx.State().GetAgentID(wasmlib.Key("owner"))
	ctx.Require(access.Exists(), "access not set: owner")
	ctx.Require(ctx.Caller() == access.Value(), "no permission")

	f := &SetOwnerContext{
		Params: ImmutableSetOwnerParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		State: MutablezentangleState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	ctx.Require(f.Params.Owner().Exists(), "missing mandatory owner")
	funcSetOwner(ctx, f)
	ctx.Log("zentangle.funcSetOwner ok")
}

type WithdrawContext struct {
	State   MutablezentangleState
}

func funcWithdrawThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("zentangle.funcWithdraw")

	// current owner of this smart contract
	access := ctx.State().GetAgentID(wasmlib.Key("owner"))
	ctx.Require(access.Exists(), "access not set: owner")
	ctx.Require(ctx.Caller() == access.Value(), "no permission")

	f := &WithdrawContext{
		State: MutablezentangleState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	funcWithdraw(ctx, f)
	ctx.Log("zentangle.funcWithdraw ok")
}

type GetOwnerContext struct {
	Results MutableGetOwnerResults
	State   ImmutablezentangleState
}

func viewGetOwnerThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("zentangle.viewGetOwner")
	f := &GetOwnerContext{
		Results: MutableGetOwnerResults{
			id: wasmlib.OBJ_ID_RESULTS,
		},
		State: ImmutablezentangleState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	viewGetOwner(ctx, f)
	ctx.Log("zentangle.viewGetOwner ok")
}

type GetPlayerBetsContext struct {
	Results MutableGetPlayerBetsResults
	State   ImmutablezentangleState
}

func viewGetPlayerBetsThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("zentangle.viewGetPlayerBets")
	f := &GetPlayerBetsContext{
		Results: MutableGetPlayerBetsResults{
			id: wasmlib.OBJ_ID_RESULTS,
		},
		State: ImmutablezentangleState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	viewGetPlayerBets(ctx, f)
	ctx.Log("zentangle.viewGetPlayerBets ok")
}

type GetPlayerInfoContext struct {
	Params  ImmutableGetPlayerInfoParams
	Results MutableGetPlayerInfoResults
	State   ImmutablezentangleState
}

func viewGetPlayerInfoThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("zentangle.viewGetPlayerInfo")
	f := &GetPlayerInfoContext{
		Params: ImmutableGetPlayerInfoParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		Results: MutableGetPlayerInfoResults{
			id: wasmlib.OBJ_ID_RESULTS,
		},
		State: ImmutablezentangleState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	ctx.Require(f.Params.PlayerAddress().Exists(), "missing mandatory playerAddress")
	viewGetPlayerInfo(ctx, f)
	ctx.Log("zentangle.viewGetPlayerInfo ok")
}

type GetPlaysPerImageContext struct {
	Params  ImmutableGetPlaysPerImageParams
	Results MutableGetPlaysPerImageResults
	State   ImmutablezentangleState
}

func viewGetPlaysPerImageThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("zentangle.viewGetPlaysPerImage")
	f := &GetPlaysPerImageContext{
		Params: ImmutableGetPlaysPerImageParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		Results: MutableGetPlaysPerImageResults{
			id: wasmlib.OBJ_ID_RESULTS,
		},
		State: ImmutablezentangleState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	ctx.Require(f.Params.ImageId().Exists(), "missing mandatory imageId")
	viewGetPlaysPerImage(ctx, f)
	ctx.Log("zentangle.viewGetPlaysPerImage ok")
}

type GetResultsContext struct {
	Params  ImmutableGetResultsParams
	Results MutableGetResultsResults
	State   ImmutablezentangleState
}

func viewGetResultsThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("zentangle.viewGetResults")
	f := &GetResultsContext{
		Params: ImmutableGetResultsParams{
			id: wasmlib.OBJ_ID_PARAMS,
		},
		Results: MutableGetResultsResults{
			id: wasmlib.OBJ_ID_RESULTS,
		},
		State: ImmutablezentangleState{
			id: wasmlib.OBJ_ID_STATE,
		},
	}
	ctx.Require(f.Params.ImageId().Exists(), "missing mandatory imageId")
	viewGetResults(ctx, f)
	ctx.Log("zentangle.viewGetResults ok")
}
