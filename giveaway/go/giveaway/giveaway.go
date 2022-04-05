// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package giveaway

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"

func funcInit(ctx wasmlib.ScFuncContext, f *InitContext) {
	if f.Params.Owner().Exists() {
		f.State.Owner().SetValue(f.Params.Owner().Value())
		return
	}
	f.State.Owner().SetValue(ctx.ContractCreator())
}

func funcSetOwner(ctx wasmlib.ScFuncContext, f *SetOwnerContext) {
	f.State.Owner().SetValue(f.Params.Owner().Value())
}

func viewGetOwner(ctx wasmlib.ScViewContext, f *GetOwnerContext) {
	f.Results.Owner().SetValue(f.State.Owner().Value())
}

func funcLoadAddresses(ctx wasmlib.ScFuncContext, f *LoadAddressesContext) {
}

func funcUnloadAddresses(ctx wasmlib.ScFuncContext, f *UnloadAddressesContext) {
}

func funcRuffle(ctx wasmlib.ScFuncContext, f *RuffleContext) {
}
