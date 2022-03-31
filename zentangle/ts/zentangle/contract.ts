// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib";
import * as sc from "./index";

export class CreateGameCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncCreateGame);
	params: sc.MutableCreateGameParams = new sc.MutableCreateGameParams();
}

export class CreateGameContext {
	events:  sc.zentangleEvents = new sc.zentangleEvents();
	params: sc.ImmutableCreateGameParams = new sc.ImmutableCreateGameParams();
	state: sc.MutablezentangleState = new sc.MutablezentangleState();
}

export class EndGameCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncEndGame);
	params: sc.MutableEndGameParams = new sc.MutableEndGameParams();
}

export class EndGameContext {
	events:  sc.zentangleEvents = new sc.zentangleEvents();
	params: sc.ImmutableEndGameParams = new sc.ImmutableEndGameParams();
	state: sc.MutablezentangleState = new sc.MutablezentangleState();
}

export class InitCall {
	func: wasmlib.ScInitFunc = new wasmlib.ScInitFunc(sc.HScName, sc.HFuncInit);
	params: sc.MutableInitParams = new sc.MutableInitParams();
}

export class InitContext {
	events:  sc.zentangleEvents = new sc.zentangleEvents();
	params: sc.ImmutableInitParams = new sc.ImmutableInitParams();
	state: sc.MutablezentangleState = new sc.MutablezentangleState();
}

export class RequestPlayCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncRequestPlay);
	results: sc.ImmutableRequestPlayResults = new sc.ImmutableRequestPlayResults();
}

export class RequestPlayContext {
	events:  sc.zentangleEvents = new sc.zentangleEvents();
	results: sc.MutableRequestPlayResults = new sc.MutableRequestPlayResults();
	state: sc.MutablezentangleState = new sc.MutablezentangleState();
}

export class SendTagsCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncSendTags);
	params: sc.MutableSendTagsParams = new sc.MutableSendTagsParams();
	results: sc.ImmutableSendTagsResults = new sc.ImmutableSendTagsResults();
}

export class SendTagsContext {
	events:  sc.zentangleEvents = new sc.zentangleEvents();
	params: sc.ImmutableSendTagsParams = new sc.ImmutableSendTagsParams();
	results: sc.MutableSendTagsResults = new sc.MutableSendTagsResults();
	state: sc.MutablezentangleState = new sc.MutablezentangleState();
}

export class SetOwnerCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncSetOwner);
	params: sc.MutableSetOwnerParams = new sc.MutableSetOwnerParams();
}

export class SetOwnerContext {
	events:  sc.zentangleEvents = new sc.zentangleEvents();
	params: sc.ImmutableSetOwnerParams = new sc.ImmutableSetOwnerParams();
	state: sc.MutablezentangleState = new sc.MutablezentangleState();
}

export class WithdrawCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncWithdraw);
}

export class WithdrawContext {
	events:  sc.zentangleEvents = new sc.zentangleEvents();
	state: sc.MutablezentangleState = new sc.MutablezentangleState();
}

export class GetOwnerCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewGetOwner);
	results: sc.ImmutableGetOwnerResults = new sc.ImmutableGetOwnerResults();
}

export class GetOwnerContext {
	results: sc.MutableGetOwnerResults = new sc.MutableGetOwnerResults();
	state: sc.ImmutablezentangleState = new sc.ImmutablezentangleState();
}

export class GetPlayerBetsCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewGetPlayerBets);
	results: sc.ImmutableGetPlayerBetsResults = new sc.ImmutableGetPlayerBetsResults();
}

export class GetPlayerBetsContext {
	results: sc.MutableGetPlayerBetsResults = new sc.MutableGetPlayerBetsResults();
	state: sc.ImmutablezentangleState = new sc.ImmutablezentangleState();
}

export class GetPlayerInfoCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewGetPlayerInfo);
	params: sc.MutableGetPlayerInfoParams = new sc.MutableGetPlayerInfoParams();
	results: sc.ImmutableGetPlayerInfoResults = new sc.ImmutableGetPlayerInfoResults();
}

export class GetPlayerInfoContext {
	params: sc.ImmutableGetPlayerInfoParams = new sc.ImmutableGetPlayerInfoParams();
	results: sc.MutableGetPlayerInfoResults = new sc.MutableGetPlayerInfoResults();
	state: sc.ImmutablezentangleState = new sc.ImmutablezentangleState();
}

export class GetPlaysPerImageCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewGetPlaysPerImage);
	params: sc.MutableGetPlaysPerImageParams = new sc.MutableGetPlaysPerImageParams();
	results: sc.ImmutableGetPlaysPerImageResults = new sc.ImmutableGetPlaysPerImageResults();
}

export class GetPlaysPerImageContext {
	params: sc.ImmutableGetPlaysPerImageParams = new sc.ImmutableGetPlaysPerImageParams();
	results: sc.MutableGetPlaysPerImageResults = new sc.MutableGetPlaysPerImageResults();
	state: sc.ImmutablezentangleState = new sc.ImmutablezentangleState();
}

export class GetResultsCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewGetResults);
	params: sc.MutableGetResultsParams = new sc.MutableGetResultsParams();
	results: sc.ImmutableGetResultsResults = new sc.ImmutableGetResultsResults();
}

export class GetResultsContext {
	params: sc.ImmutableGetResultsParams = new sc.ImmutableGetResultsParams();
	results: sc.MutableGetResultsResults = new sc.MutableGetResultsResults();
	state: sc.ImmutablezentangleState = new sc.ImmutablezentangleState();
}

export class ScFuncs {
    static createGame(ctx: wasmlib.ScFuncCallContext): CreateGameCall {
        let f = new CreateGameCall();
        f.func.setPtrs(f.params, null);
        return f;
    }

    static endGame(ctx: wasmlib.ScFuncCallContext): EndGameCall {
        let f = new EndGameCall();
        f.func.setPtrs(f.params, null);
        return f;
    }

    static init(ctx: wasmlib.ScFuncCallContext): InitCall {
        let f = new InitCall();
        f.func.setPtrs(f.params, null);
        return f;
    }

    static requestPlay(ctx: wasmlib.ScFuncCallContext): RequestPlayCall {
        let f = new RequestPlayCall();
        f.func.setPtrs(null, f.results);
        return f;
    }

    static sendTags(ctx: wasmlib.ScFuncCallContext): SendTagsCall {
        let f = new SendTagsCall();
        f.func.setPtrs(f.params, f.results);
        return f;
    }

    static setOwner(ctx: wasmlib.ScFuncCallContext): SetOwnerCall {
        let f = new SetOwnerCall();
        f.func.setPtrs(f.params, null);
        return f;
    }

    static withdraw(ctx: wasmlib.ScFuncCallContext): WithdrawCall {
        return new WithdrawCall();
    }

    static getOwner(ctx: wasmlib.ScViewCallContext): GetOwnerCall {
        let f = new GetOwnerCall();
        f.func.setPtrs(null, f.results);
        return f;
    }

    static getPlayerBets(ctx: wasmlib.ScViewCallContext): GetPlayerBetsCall {
        let f = new GetPlayerBetsCall();
        f.func.setPtrs(null, f.results);
        return f;
    }

    static getPlayerInfo(ctx: wasmlib.ScViewCallContext): GetPlayerInfoCall {
        let f = new GetPlayerInfoCall();
        f.func.setPtrs(f.params, f.results);
        return f;
    }

    static getPlaysPerImage(ctx: wasmlib.ScViewCallContext): GetPlaysPerImageCall {
        let f = new GetPlaysPerImageCall();
        f.func.setPtrs(f.params, f.results);
        return f;
    }

    static getResults(ctx: wasmlib.ScViewCallContext): GetResultsCall {
        let f = new GetResultsCall();
        f.func.setPtrs(f.params, f.results);
        return f;
    }
}
