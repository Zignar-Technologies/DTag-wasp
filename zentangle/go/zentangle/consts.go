// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package zentangle

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

const (
	ScName        = "zentangle"
	ScDescription = "Incentivized AI Training Casino"
	HScName       = wasmlib.ScHname(0xf707a9c6)
)

const (
	ParamDescription          = "description"
	ParamImageId              = "imageId"
	ParamInputJson            = "inputJson"
	ParamNumberOfImages       = "numberOfImages"
	ParamOwner                = "owner"
	ParamPlayerAddress        = "playerAddress"
	ParamResetPlayerInfo      = "resetPlayerInfo"
	ParamTagsRequiredPerImage = "tagsRequiredPerImage"
)

const (
	ResultImageId       = "imageId"
	ResultInfo          = "info"
	ResultOwner         = "owner"
	ResultPlayerBets    = "playerBets"
	ResultPlaysPerImage = "playsPerImage"
	ResultResults       = "results"
)

const (
	StateBets                 = "bets"
	StateCreator              = "creator"
	StateDescription          = "description"
	StateNumberOfImages       = "numberOfImages"
	StateOwner                = "owner"
	StatePendingPlay          = "pendingPlay"
	StatePlayer               = "player"
	StatePlaysPerImage        = "playsPerImage"
	StateProcessedImages      = "processedImages"
	StateReward               = "reward"
	StateTaggedImages         = "taggedImages"
	StateTagsRequiredPerImage = "tagsRequiredPerImage"
	StateValidTags            = "validTags"
)

const (
	FuncCreateGame       = "createGame"
	FuncEndGame          = "endGame"
	FuncInit             = "init"
	FuncRequestPlay      = "requestPlay"
	FuncSendTags         = "sendTags"
	FuncSetOwner         = "setOwner"
	FuncWithdraw         = "withdraw"
	ViewGetOwner         = "getOwner"
	ViewGetPlayerBets    = "getPlayerBets"
	ViewGetPlayerInfo    = "getPlayerInfo"
	ViewGetPlaysPerImage = "getPlaysPerImage"
	ViewGetResults       = "getResults"
)

const (
	HFuncCreateGame       = wasmlib.ScHname(0x585dcce2)
	HFuncEndGame          = wasmlib.ScHname(0xb2303ef2)
	HFuncInit             = wasmlib.ScHname(0x1f44d644)
	HFuncRequestPlay      = wasmlib.ScHname(0x74f0bf82)
	HFuncSendTags         = wasmlib.ScHname(0xc31816cb)
	HFuncSetOwner         = wasmlib.ScHname(0x2a15fe7b)
	HFuncWithdraw         = wasmlib.ScHname(0x9dcc0f41)
	HViewGetOwner         = wasmlib.ScHname(0x137107a6)
	HViewGetPlayerBets    = wasmlib.ScHname(0x842b0ef5)
	HViewGetPlayerInfo    = wasmlib.ScHname(0x504151da)
	HViewGetPlaysPerImage = wasmlib.ScHname(0x749519e8)
	HViewGetResults       = wasmlib.ScHname(0xc2ed9edb)
)
