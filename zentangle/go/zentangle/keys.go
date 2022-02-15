// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package zentangle

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

const (
	IdxParamDescription          = 0
	IdxParamImageId              = 1
	IdxParamInputJson            = 2
	IdxParamNumberOfImages       = 3
	IdxParamOwner                = 4
	IdxParamPlayerAddress        = 5
	IdxParamResetPlayerInfo      = 6
	IdxParamTagsRequiredPerImage = 7

	IdxResultImageId       = 8
	IdxResultInfo          = 9
	IdxResultOwner         = 10
	IdxResultPlayerBets    = 11
	IdxResultPlaysPerImage = 12
	IdxResultResults       = 13

	IdxStateBets                 = 14
	IdxStateCreator              = 15
	IdxStateDescription          = 16
	IdxStateNumberOfImages       = 17
	IdxStateOwner                = 18
	IdxStatePendingPlay          = 19
	IdxStatePlayer               = 20
	IdxStatePlaysPerImage        = 21
	IdxStateProcessedImages      = 22
	IdxStateReward               = 23
	IdxStateTaggedImages         = 24
	IdxStateTagsRequiredPerImage = 25
	IdxStateValidTags            = 26
)

const keyMapLen = 27

var keyMap = [keyMapLen]wasmlib.Key{
	ParamDescription,
	ParamImageId,
	ParamInputJson,
	ParamNumberOfImages,
	ParamOwner,
	ParamPlayerAddress,
	ParamResetPlayerInfo,
	ParamTagsRequiredPerImage,
	ResultImageId,
	ResultInfo,
	ResultOwner,
	ResultPlayerBets,
	ResultPlaysPerImage,
	ResultResults,
	StateBets,
	StateCreator,
	StateDescription,
	StateNumberOfImages,
	StateOwner,
	StatePendingPlay,
	StatePlayer,
	StatePlaysPerImage,
	StateProcessedImages,
	StateReward,
	StateTaggedImages,
	StateTagsRequiredPerImage,
	StateValidTags,
}

var idxMap [keyMapLen]wasmlib.Key32
