// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package zentangle

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"

func funcCreateGame(ctx wasmlib.ScFuncContext, f *CreateGameContext) {
}

func funcEndGame(ctx wasmlib.ScFuncContext, f *EndGameContext) {
}

func funcInit(ctx wasmlib.ScFuncContext, f *InitContext) {
    if f.Params.Owner().Exists() {
        f.State.Owner().SetValue(f.Params.Owner().Value())
        return
    }
    f.State.Owner().SetValue(ctx.ContractCreator())
}

func funcRequestPlay(ctx wasmlib.ScFuncContext, f *RequestPlayContext) {
}

func funcSendTags(ctx wasmlib.ScFuncContext, f *SendTagsContext) {
}

func funcSetOwner(ctx wasmlib.ScFuncContext, f *SetOwnerContext) {
	f.State.Owner().SetValue(f.Params.Owner().Value())
}

func funcWithdraw(ctx wasmlib.ScFuncContext, f *WithdrawContext) {
}

func viewGetOwner(ctx wasmlib.ScViewContext, f *GetOwnerContext) {
	f.Results.Owner().SetValue(f.State.Owner().Value())
}

func viewGetPlayerBets(ctx wasmlib.ScViewContext, f *GetPlayerBetsContext) {
}

func viewGetPlayerInfo(ctx wasmlib.ScViewContext, f *GetPlayerInfoContext) {
}

func viewGetPlaysPerImage(ctx wasmlib.ScViewContext, f *GetPlaysPerImageContext) {
}

func viewGetResults(ctx wasmlib.ScViewContext, f *GetResultsContext) {
}
