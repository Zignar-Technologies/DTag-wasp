// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmclient from "wasmclient"
import * as events from "./events"

const ArgDescription = "description";
const ArgImageId = "imageId";
const ArgInputJson = "inputJson";
const ArgMission = "mission";
const ArgNumberOfImages = "numberOfImages";
const ArgOwner = "owner";
const ArgPlayerAddress = "playerAddress";
const ArgResetPlayerInfo = "resetPlayerInfo";
const ArgTagsRequiredPerImage = "tagsRequiredPerImage";

const ResImageId = "imageId";
const ResInfo = "info";
const ResOwner = "owner";
const ResPlayerBets = "playerBets";
const ResPlaysPerImage = "playsPerImage";
const ResResults = "results";

///////////////////////////// createGame /////////////////////////////

export class CreateGameFunc extends wasmclient.ClientFunc {
	private args: wasmclient.Arguments = new wasmclient.Arguments();
	
	public description(v: string): void {
		this.args.set(ArgDescription, this.args.fromString(v));
	}
	
	public numberOfImages(v: wasmclient.Uint32): void {
		this.args.set(ArgNumberOfImages, this.args.fromUint32(v));
	}
	
	public tagsRequiredPerImage(v: wasmclient.Uint32): void {
		this.args.set(ArgTagsRequiredPerImage, this.args.fromUint32(v));
	}
	
	public async post(): Promise<wasmclient.RequestID> {
		this.args.mandatory(ArgDescription);
		this.args.mandatory(ArgNumberOfImages);
		return await super.post(0x585dcce2, this.args);
	}
}

///////////////////////////// endGame /////////////////////////////

export class EndGameFunc extends wasmclient.ClientFunc {
	private args: wasmclient.Arguments = new wasmclient.Arguments();
	
	public mission(v: string): void {
		this.args.set(ArgMission, this.args.fromString(v));
	}
	
	public resetPlayerInfo(v: boolean): void {
		this.args.set(ArgResetPlayerInfo, this.args.fromBool(v));
	}
	
	public async post(): Promise<wasmclient.RequestID> {
		this.args.mandatory(ArgMission);
		return await super.post(0xb2303ef2, this.args);
	}
}

///////////////////////////// init /////////////////////////////

export class InitFunc extends wasmclient.ClientFunc {
	private args: wasmclient.Arguments = new wasmclient.Arguments();
	
	public owner(v: wasmclient.AgentID): void {
		this.args.set(ArgOwner, this.args.fromAgentID(v));
	}
	
	public async post(): Promise<wasmclient.RequestID> {
		return await super.post(0x1f44d644, this.args);
	}
}

///////////////////////////// requestPlay /////////////////////////////

export class RequestPlayFunc extends wasmclient.ClientFunc {
	
	public async post(): Promise<wasmclient.RequestID> {
		return await super.post(0x74f0bf82, null);
	}
}

export class RequestPlayResults extends wasmclient.Results {

	imageId(): wasmclient.Uint32 {
		return this.toUint32(this.get(ResImageId));
	}
}

///////////////////////////// sendTags /////////////////////////////

export class SendTagsFunc extends wasmclient.ClientFunc {
	private args: wasmclient.Arguments = new wasmclient.Arguments();
	
	public inputJson(v: string): void {
		this.args.set(ArgInputJson, this.args.fromString(v));
	}
	
	public async post(): Promise<wasmclient.RequestID> {
		this.args.mandatory(ArgInputJson);
		return await super.post(0xc31816cb, this.args);
	}
}

export class SendTagsResults extends wasmclient.Results {

	imageId(): wasmclient.Uint32 {
		return this.toUint32(this.get(ResImageId));
	}
}

///////////////////////////// setOwner /////////////////////////////

export class SetOwnerFunc extends wasmclient.ClientFunc {
	private args: wasmclient.Arguments = new wasmclient.Arguments();
	
	public owner(v: wasmclient.AgentID): void {
		this.args.set(ArgOwner, this.args.fromAgentID(v));
	}
	
	public async post(): Promise<wasmclient.RequestID> {
		this.args.mandatory(ArgOwner);
		return await super.post(0x2a15fe7b, this.args);
	}
}

///////////////////////////// withdraw /////////////////////////////

export class WithdrawFunc extends wasmclient.ClientFunc {
	
	public async post(): Promise<wasmclient.RequestID> {
		return await super.post(0x9dcc0f41, null);
	}
}

///////////////////////////// getOwner /////////////////////////////

export class GetOwnerView extends wasmclient.ClientView {

	public async call(): Promise<GetOwnerResults> {
		const res = new GetOwnerResults();
		await this.callView("getOwner", null, res);
		return res;
	}
}

export class GetOwnerResults extends wasmclient.Results {

	owner(): wasmclient.AgentID {
		return this.toAgentID(this.get(ResOwner));
	}
}

///////////////////////////// getPlayerBets /////////////////////////////

export class GetPlayerBetsView extends wasmclient.ClientView {

	public async call(): Promise<GetPlayerBetsResults> {
		const res = new GetPlayerBetsResults();
		await this.callView("getPlayerBets", null, res);
		return res;
	}
}

export class GetPlayerBetsResults extends wasmclient.Results {

	playerBets(): string {
		return this.toString(this.get(ResPlayerBets));
	}
}

///////////////////////////// getPlayerInfo /////////////////////////////

export class GetPlayerInfoView extends wasmclient.ClientView {
	private args: wasmclient.Arguments = new wasmclient.Arguments();
	
	public playerAddress(v: string): void {
		this.args.set(ArgPlayerAddress, this.args.fromString(v));
	}

	public async call(): Promise<GetPlayerInfoResults> {
		this.args.mandatory(ArgPlayerAddress);
		const res = new GetPlayerInfoResults();
		await this.callView("getPlayerInfo", this.args, res);
		return res;
	}
}

export class GetPlayerInfoResults extends wasmclient.Results {

	info(): string {
		return this.toString(this.get(ResInfo));
	}
}

///////////////////////////// getPlaysPerImage /////////////////////////////

export class GetPlaysPerImageView extends wasmclient.ClientView {
	private args: wasmclient.Arguments = new wasmclient.Arguments();
	
	public imageId(v: wasmclient.Uint32): void {
		this.args.set(ArgImageId, this.args.fromUint32(v));
	}

	public async call(): Promise<GetPlaysPerImageResults> {
		this.args.mandatory(ArgImageId);
		const res = new GetPlaysPerImageResults();
		await this.callView("getPlaysPerImage", this.args, res);
		return res;
	}
}

export class GetPlaysPerImageResults extends wasmclient.Results {

	playsPerImage(): wasmclient.Uint32 {
		return this.toUint32(this.get(ResPlaysPerImage));
	}
}

///////////////////////////// getResults /////////////////////////////

export class GetResultsView extends wasmclient.ClientView {
	private args: wasmclient.Arguments = new wasmclient.Arguments();
	
	public imageId(v: wasmclient.Uint32): void {
		this.args.set(ArgImageId, this.args.fromUint32(v));
	}

	public async call(): Promise<GetResultsResults> {
		this.args.mandatory(ArgImageId);
		const res = new GetResultsResults();
		await this.callView("getResults", this.args, res);
		return res;
	}
}

export class GetResultsResults extends wasmclient.Results {

	results(): string {
		return this.toString(this.get(ResResults));
	}
}

///////////////////////////// ZentangleService /////////////////////////////

export class ZentangleService extends wasmclient.Service {

	public constructor(cl: wasmclient.ServiceClient) {
		super(cl, 0xf707a9c6);
	}

	public newEventHandler(): events.ZentangleEvents {
		return new events.ZentangleEvents();
	}

	public createGame(): CreateGameFunc {
		return new CreateGameFunc(this);
	}

	public endGame(): EndGameFunc {
		return new EndGameFunc(this);
	}

	public init(): InitFunc {
		return new InitFunc(this);
	}

	public requestPlay(): RequestPlayFunc {
		return new RequestPlayFunc(this);
	}

	public sendTags(): SendTagsFunc {
		return new SendTagsFunc(this);
	}

	public setOwner(): SetOwnerFunc {
		return new SetOwnerFunc(this);
	}

	public withdraw(): WithdrawFunc {
		return new WithdrawFunc(this);
	}

	public getOwner(): GetOwnerView {
		return new GetOwnerView(this);
	}

	public getPlayerBets(): GetPlayerBetsView {
		return new GetPlayerBetsView(this);
	}

	public getPlayerInfo(): GetPlayerInfoView {
		return new GetPlayerInfoView(this);
	}

	public getPlaysPerImage(): GetPlaysPerImageView {
		return new GetPlaysPerImageView(this);
	}

	public getResults(): GetResultsView {
		return new GetResultsView(this);
	}
}
