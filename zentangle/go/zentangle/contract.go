// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package zentangle

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

type CreateGameCall struct {
	Func    *wasmlib.ScFunc
	Params  MutableCreateGameParams
}

type EndGameCall struct {
	Func    *wasmlib.ScFunc
	Params  MutableEndGameParams
}

type InitCall struct {
	Func    *wasmlib.ScInitFunc
	Params  MutableInitParams
}

type RequestPlayCall struct {
	Func    *wasmlib.ScFunc
	Results ImmutableRequestPlayResults
}

type SendTagsCall struct {
	Func    *wasmlib.ScFunc
	Params  MutableSendTagsParams
}

type SetOwnerCall struct {
	Func    *wasmlib.ScFunc
	Params  MutableSetOwnerParams
}

type WithdrawCall struct {
	Func    *wasmlib.ScFunc
}

type GetOwnerCall struct {
	Func    *wasmlib.ScView
	Results ImmutableGetOwnerResults
}

type GetPlayerBetsCall struct {
	Func    *wasmlib.ScView
	Results ImmutableGetPlayerBetsResults
}

type GetPlayerInfoCall struct {
	Func    *wasmlib.ScView
	Params  MutableGetPlayerInfoParams
	Results ImmutableGetPlayerInfoResults
}

type GetPlaysPerImageCall struct {
	Func    *wasmlib.ScView
	Params  MutableGetPlaysPerImageParams
	Results ImmutableGetPlaysPerImageResults
}

type GetResultsCall struct {
	Func    *wasmlib.ScView
	Params  MutableGetResultsParams
	Results ImmutableGetResultsResults
}

type Funcs struct{}

var ScFuncs Funcs

func (sc Funcs) CreateGame(ctx wasmlib.ScFuncCallContext) *CreateGameCall {
	f := &CreateGameCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncCreateGame)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) EndGame(ctx wasmlib.ScFuncCallContext) *EndGameCall {
	f := &EndGameCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncEndGame)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) Init(ctx wasmlib.ScFuncCallContext) *InitCall {
	f := &InitCall{Func: wasmlib.NewScInitFunc(ctx, HScName, HFuncInit, keyMap[:], idxMap[:])}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) RequestPlay(ctx wasmlib.ScFuncCallContext) *RequestPlayCall {
	f := &RequestPlayCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncRequestPlay)}
	f.Func.SetPtrs(nil, &f.Results.id)
	return f
}

func (sc Funcs) SendTags(ctx wasmlib.ScFuncCallContext) *SendTagsCall {
	f := &SendTagsCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncSendTags)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) SetOwner(ctx wasmlib.ScFuncCallContext) *SetOwnerCall {
	f := &SetOwnerCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncSetOwner)}
	f.Func.SetPtrs(&f.Params.id, nil)
	return f
}

func (sc Funcs) Withdraw(ctx wasmlib.ScFuncCallContext) *WithdrawCall {
	return &WithdrawCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncWithdraw)}
}

func (sc Funcs) GetOwner(ctx wasmlib.ScViewCallContext) *GetOwnerCall {
	f := &GetOwnerCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetOwner)}
	f.Func.SetPtrs(nil, &f.Results.id)
	return f
}

func (sc Funcs) GetPlayerBets(ctx wasmlib.ScViewCallContext) *GetPlayerBetsCall {
	f := &GetPlayerBetsCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetPlayerBets)}
	f.Func.SetPtrs(nil, &f.Results.id)
	return f
}

func (sc Funcs) GetPlayerInfo(ctx wasmlib.ScViewCallContext) *GetPlayerInfoCall {
	f := &GetPlayerInfoCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetPlayerInfo)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}

func (sc Funcs) GetPlaysPerImage(ctx wasmlib.ScViewCallContext) *GetPlaysPerImageCall {
	f := &GetPlaysPerImageCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetPlaysPerImage)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}

func (sc Funcs) GetResults(ctx wasmlib.ScViewCallContext) *GetResultsCall {
	f := &GetResultsCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetResults)}
	f.Func.SetPtrs(&f.Params.id, &f.Results.id)
	return f
}
