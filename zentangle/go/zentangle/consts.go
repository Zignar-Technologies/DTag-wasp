// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package zentangle

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib/wasmtypes"

const (
	ScName        = "zentangle"
	ScDescription = "Incentivized AI Training Casino"
	HScName       = wasmtypes.ScHname(0xf707a9c6)
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
	StateBets                  = "bets"
	StateCompleteImages        = "completeImages"
	StateCreator               = "creator"
	StateDescription           = "description"
	StateNumberOfImages        = "numberOfImages"
	StateOwner                 = "owner"
	StatePendingPlay           = "pendingPlay"
	StatePendingPlays          = "pendingPlays"
	StatePlayerBoost           = "playerBoost"
	StatePlayersBoost          = "playersBoost"
	StatePlaysPerImage         = "playsPerImage"
	StatePlaysRequiredPerImage = "playsRequiredPerImage"
	StateProcessedImages       = "processedImages"
	StateReward                = "reward"
	StateTaggedImages          = "taggedImages"
	StateTotalPlayerTags       = "totalPlayerTags"
	StateValidTags             = "validTags"
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
	HFuncCreateGame       = wasmtypes.ScHname(0x585dcce2)
	HFuncEndGame          = wasmtypes.ScHname(0xb2303ef2)
	HFuncInit             = wasmtypes.ScHname(0x1f44d644)
	HFuncRequestPlay      = wasmtypes.ScHname(0x74f0bf82)
	HFuncSendTags         = wasmtypes.ScHname(0xc31816cb)
	HFuncSetOwner         = wasmtypes.ScHname(0x2a15fe7b)
	HFuncWithdraw         = wasmtypes.ScHname(0x9dcc0f41)
	HViewGetOwner         = wasmtypes.ScHname(0x137107a6)
	HViewGetPlayerBets    = wasmtypes.ScHname(0x842b0ef5)
	HViewGetPlayerInfo    = wasmtypes.ScHname(0x504151da)
	HViewGetPlaysPerImage = wasmtypes.ScHname(0x749519e8)
	HViewGetResults       = wasmtypes.ScHname(0xc2ed9edb)
)
