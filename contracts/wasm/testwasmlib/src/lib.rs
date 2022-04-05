// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use testwasmlib::*;
use wasmlib::*;

use crate::consts::*;
use crate::events::*;
use crate::params::*;
use crate::results::*;
use crate::state::*;
use crate::structs::*;
use crate::typedefs::*;

mod consts;
mod contract;
mod events;
mod params;
mod results;
mod state;
mod structs;
mod typedefs;

mod testwasmlib;

const EXPORT_MAP: ScExportMap = ScExportMap {
    names: &[
    	FUNC_ARRAY_OF_ARRAYS_APPEND,
    	FUNC_ARRAY_OF_ARRAYS_CLEAR,
    	FUNC_ARRAY_OF_ARRAYS_SET,
    	FUNC_ARRAY_OF_MAPS_CLEAR,
    	FUNC_ARRAY_OF_MAPS_SET,
    	FUNC_MAP_OF_ARRAYS_APPEND,
    	FUNC_MAP_OF_ARRAYS_CLEAR,
    	FUNC_MAP_OF_ARRAYS_SET,
    	FUNC_MAP_OF_MAPS_CLEAR,
    	FUNC_MAP_OF_MAPS_SET,
    	FUNC_PARAM_TYPES,
    	FUNC_RANDOM,
    	FUNC_TRIGGER_EVENT,
    	VIEW_ARRAY_OF_ARRAYS_LENGTH,
    	VIEW_ARRAY_OF_ARRAYS_VALUE,
    	VIEW_ARRAY_OF_MAPS_VALUE,
    	VIEW_BLOCK_RECORD,
    	VIEW_BLOCK_RECORDS,
    	VIEW_GET_RANDOM,
    	VIEW_IOTA_BALANCE,
    	VIEW_MAP_OF_ARRAYS_LENGTH,
    	VIEW_MAP_OF_ARRAYS_VALUE,
    	VIEW_MAP_OF_MAPS_VALUE,
	],
    funcs: &[
    	func_array_of_arrays_append_thunk,
    	func_array_of_arrays_clear_thunk,
    	func_array_of_arrays_set_thunk,
    	func_array_of_maps_clear_thunk,
    	func_array_of_maps_set_thunk,
    	func_map_of_arrays_append_thunk,
    	func_map_of_arrays_clear_thunk,
    	func_map_of_arrays_set_thunk,
    	func_map_of_maps_clear_thunk,
    	func_map_of_maps_set_thunk,
    	func_param_types_thunk,
    	func_random_thunk,
    	func_trigger_event_thunk,
	],
    views: &[
    	view_array_of_arrays_length_thunk,
    	view_array_of_arrays_value_thunk,
    	view_array_of_maps_value_thunk,
    	view_block_record_thunk,
    	view_block_records_thunk,
    	view_get_random_thunk,
    	view_iota_balance_thunk,
    	view_map_of_arrays_length_thunk,
    	view_map_of_arrays_value_thunk,
    	view_map_of_maps_value_thunk,
	],
};

#[no_mangle]
fn on_call(index: i32) {
	ScExports::call(index, &EXPORT_MAP);
}

#[no_mangle]
fn on_load() {
    ScExports::export(&EXPORT_MAP);
}

pub struct ArrayOfArraysAppendContext {
	events:  TestWasmLibEvents,
	params: ImmutableArrayOfArraysAppendParams,
	state: MutableTestWasmLibState,
}

fn func_array_of_arrays_append_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcArrayOfArraysAppend");
	let f = ArrayOfArraysAppendContext {
		events:  TestWasmLibEvents {},
		params: ImmutableArrayOfArraysAppendParams { proxy: params_proxy() },
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.index().exists(), "missing mandatory index");
	func_array_of_arrays_append(ctx, &f);
	ctx.log("testwasmlib.funcArrayOfArraysAppend ok");
}

pub struct ArrayOfArraysClearContext {
	events:  TestWasmLibEvents,
	state: MutableTestWasmLibState,
}

fn func_array_of_arrays_clear_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcArrayOfArraysClear");
	let f = ArrayOfArraysClearContext {
		events:  TestWasmLibEvents {},
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	func_array_of_arrays_clear(ctx, &f);
	ctx.log("testwasmlib.funcArrayOfArraysClear ok");
}

pub struct ArrayOfArraysSetContext {
	events:  TestWasmLibEvents,
	params: ImmutableArrayOfArraysSetParams,
	state: MutableTestWasmLibState,
}

fn func_array_of_arrays_set_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcArrayOfArraysSet");
	let f = ArrayOfArraysSetContext {
		events:  TestWasmLibEvents {},
		params: ImmutableArrayOfArraysSetParams { proxy: params_proxy() },
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.index0().exists(), "missing mandatory index0");
	ctx.require(f.params.index1().exists(), "missing mandatory index1");
	ctx.require(f.params.value().exists(), "missing mandatory value");
	func_array_of_arrays_set(ctx, &f);
	ctx.log("testwasmlib.funcArrayOfArraysSet ok");
}

pub struct ArrayOfMapsClearContext {
	events:  TestWasmLibEvents,
	state: MutableTestWasmLibState,
}

fn func_array_of_maps_clear_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcArrayOfMapsClear");
	let f = ArrayOfMapsClearContext {
		events:  TestWasmLibEvents {},
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	func_array_of_maps_clear(ctx, &f);
	ctx.log("testwasmlib.funcArrayOfMapsClear ok");
}

pub struct ArrayOfMapsSetContext {
	events:  TestWasmLibEvents,
	params: ImmutableArrayOfMapsSetParams,
	state: MutableTestWasmLibState,
}

fn func_array_of_maps_set_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcArrayOfMapsSet");
	let f = ArrayOfMapsSetContext {
		events:  TestWasmLibEvents {},
		params: ImmutableArrayOfMapsSetParams { proxy: params_proxy() },
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.index().exists(), "missing mandatory index");
	ctx.require(f.params.key().exists(), "missing mandatory key");
	ctx.require(f.params.value().exists(), "missing mandatory value");
	func_array_of_maps_set(ctx, &f);
	ctx.log("testwasmlib.funcArrayOfMapsSet ok");
}

pub struct MapOfArraysAppendContext {
	events:  TestWasmLibEvents,
	params: ImmutableMapOfArraysAppendParams,
	state: MutableTestWasmLibState,
}

fn func_map_of_arrays_append_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcMapOfArraysAppend");
	let f = MapOfArraysAppendContext {
		events:  TestWasmLibEvents {},
		params: ImmutableMapOfArraysAppendParams { proxy: params_proxy() },
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.name().exists(), "missing mandatory name");
	ctx.require(f.params.value().exists(), "missing mandatory value");
	func_map_of_arrays_append(ctx, &f);
	ctx.log("testwasmlib.funcMapOfArraysAppend ok");
}

pub struct MapOfArraysClearContext {
	events:  TestWasmLibEvents,
	params: ImmutableMapOfArraysClearParams,
	state: MutableTestWasmLibState,
}

fn func_map_of_arrays_clear_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcMapOfArraysClear");
	let f = MapOfArraysClearContext {
		events:  TestWasmLibEvents {},
		params: ImmutableMapOfArraysClearParams { proxy: params_proxy() },
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.name().exists(), "missing mandatory name");
	func_map_of_arrays_clear(ctx, &f);
	ctx.log("testwasmlib.funcMapOfArraysClear ok");
}

pub struct MapOfArraysSetContext {
	events:  TestWasmLibEvents,
	params: ImmutableMapOfArraysSetParams,
	state: MutableTestWasmLibState,
}

fn func_map_of_arrays_set_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcMapOfArraysSet");
	let f = MapOfArraysSetContext {
		events:  TestWasmLibEvents {},
		params: ImmutableMapOfArraysSetParams { proxy: params_proxy() },
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.index().exists(), "missing mandatory index");
	ctx.require(f.params.name().exists(), "missing mandatory name");
	ctx.require(f.params.value().exists(), "missing mandatory value");
	func_map_of_arrays_set(ctx, &f);
	ctx.log("testwasmlib.funcMapOfArraysSet ok");
}

pub struct MapOfMapsClearContext {
	events:  TestWasmLibEvents,
	params: ImmutableMapOfMapsClearParams,
	state: MutableTestWasmLibState,
}

fn func_map_of_maps_clear_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcMapOfMapsClear");
	let f = MapOfMapsClearContext {
		events:  TestWasmLibEvents {},
		params: ImmutableMapOfMapsClearParams { proxy: params_proxy() },
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.name().exists(), "missing mandatory name");
	func_map_of_maps_clear(ctx, &f);
	ctx.log("testwasmlib.funcMapOfMapsClear ok");
}

pub struct MapOfMapsSetContext {
	events:  TestWasmLibEvents,
	params: ImmutableMapOfMapsSetParams,
	state: MutableTestWasmLibState,
}

fn func_map_of_maps_set_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcMapOfMapsSet");
	let f = MapOfMapsSetContext {
		events:  TestWasmLibEvents {},
		params: ImmutableMapOfMapsSetParams { proxy: params_proxy() },
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.key().exists(), "missing mandatory key");
	ctx.require(f.params.name().exists(), "missing mandatory name");
	ctx.require(f.params.value().exists(), "missing mandatory value");
	func_map_of_maps_set(ctx, &f);
	ctx.log("testwasmlib.funcMapOfMapsSet ok");
}

pub struct ParamTypesContext {
	events:  TestWasmLibEvents,
	params: ImmutableParamTypesParams,
	state: MutableTestWasmLibState,
}

fn func_param_types_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcParamTypes");
	let f = ParamTypesContext {
		events:  TestWasmLibEvents {},
		params: ImmutableParamTypesParams { proxy: params_proxy() },
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	func_param_types(ctx, &f);
	ctx.log("testwasmlib.funcParamTypes ok");
}

pub struct RandomContext {
	events:  TestWasmLibEvents,
	state: MutableTestWasmLibState,
}

fn func_random_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcRandom");
	let f = RandomContext {
		events:  TestWasmLibEvents {},
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	func_random(ctx, &f);
	ctx.log("testwasmlib.funcRandom ok");
}

pub struct TriggerEventContext {
	events:  TestWasmLibEvents,
	params: ImmutableTriggerEventParams,
	state: MutableTestWasmLibState,
}

fn func_trigger_event_thunk(ctx: &ScFuncContext) {
	ctx.log("testwasmlib.funcTriggerEvent");
	let f = TriggerEventContext {
		events:  TestWasmLibEvents {},
		params: ImmutableTriggerEventParams { proxy: params_proxy() },
		state: MutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.address().exists(), "missing mandatory address");
	ctx.require(f.params.name().exists(), "missing mandatory name");
	func_trigger_event(ctx, &f);
	ctx.log("testwasmlib.funcTriggerEvent ok");
}

pub struct ArrayOfArraysLengthContext {
	results: MutableArrayOfArraysLengthResults,
	state: ImmutableTestWasmLibState,
}

fn view_array_of_arrays_length_thunk(ctx: &ScViewContext) {
	ctx.log("testwasmlib.viewArrayOfArraysLength");
	let f = ArrayOfArraysLengthContext {
		results: MutableArrayOfArraysLengthResults { proxy: results_proxy() },
		state: ImmutableTestWasmLibState { proxy: state_proxy() },
	};
	view_array_of_arrays_length(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testwasmlib.viewArrayOfArraysLength ok");
}

pub struct ArrayOfArraysValueContext {
	params: ImmutableArrayOfArraysValueParams,
	results: MutableArrayOfArraysValueResults,
	state: ImmutableTestWasmLibState,
}

fn view_array_of_arrays_value_thunk(ctx: &ScViewContext) {
	ctx.log("testwasmlib.viewArrayOfArraysValue");
	let f = ArrayOfArraysValueContext {
		params: ImmutableArrayOfArraysValueParams { proxy: params_proxy() },
		results: MutableArrayOfArraysValueResults { proxy: results_proxy() },
		state: ImmutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.index0().exists(), "missing mandatory index0");
	ctx.require(f.params.index1().exists(), "missing mandatory index1");
	view_array_of_arrays_value(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testwasmlib.viewArrayOfArraysValue ok");
}

pub struct ArrayOfMapsValueContext {
	params: ImmutableArrayOfMapsValueParams,
	results: MutableArrayOfMapsValueResults,
	state: ImmutableTestWasmLibState,
}

fn view_array_of_maps_value_thunk(ctx: &ScViewContext) {
	ctx.log("testwasmlib.viewArrayOfMapsValue");
	let f = ArrayOfMapsValueContext {
		params: ImmutableArrayOfMapsValueParams { proxy: params_proxy() },
		results: MutableArrayOfMapsValueResults { proxy: results_proxy() },
		state: ImmutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.index().exists(), "missing mandatory index");
	ctx.require(f.params.key().exists(), "missing mandatory key");
	view_array_of_maps_value(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testwasmlib.viewArrayOfMapsValue ok");
}

pub struct BlockRecordContext {
	params: ImmutableBlockRecordParams,
	results: MutableBlockRecordResults,
	state: ImmutableTestWasmLibState,
}

fn view_block_record_thunk(ctx: &ScViewContext) {
	ctx.log("testwasmlib.viewBlockRecord");
	let f = BlockRecordContext {
		params: ImmutableBlockRecordParams { proxy: params_proxy() },
		results: MutableBlockRecordResults { proxy: results_proxy() },
		state: ImmutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.block_index().exists(), "missing mandatory blockIndex");
	ctx.require(f.params.record_index().exists(), "missing mandatory recordIndex");
	view_block_record(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testwasmlib.viewBlockRecord ok");
}

pub struct BlockRecordsContext {
	params: ImmutableBlockRecordsParams,
	results: MutableBlockRecordsResults,
	state: ImmutableTestWasmLibState,
}

fn view_block_records_thunk(ctx: &ScViewContext) {
	ctx.log("testwasmlib.viewBlockRecords");
	let f = BlockRecordsContext {
		params: ImmutableBlockRecordsParams { proxy: params_proxy() },
		results: MutableBlockRecordsResults { proxy: results_proxy() },
		state: ImmutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.block_index().exists(), "missing mandatory blockIndex");
	view_block_records(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testwasmlib.viewBlockRecords ok");
}

pub struct GetRandomContext {
	results: MutableGetRandomResults,
	state: ImmutableTestWasmLibState,
}

fn view_get_random_thunk(ctx: &ScViewContext) {
	ctx.log("testwasmlib.viewGetRandom");
	let f = GetRandomContext {
		results: MutableGetRandomResults { proxy: results_proxy() },
		state: ImmutableTestWasmLibState { proxy: state_proxy() },
	};
	view_get_random(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testwasmlib.viewGetRandom ok");
}

pub struct IotaBalanceContext {
	results: MutableIotaBalanceResults,
	state: ImmutableTestWasmLibState,
}

fn view_iota_balance_thunk(ctx: &ScViewContext) {
	ctx.log("testwasmlib.viewIotaBalance");
	let f = IotaBalanceContext {
		results: MutableIotaBalanceResults { proxy: results_proxy() },
		state: ImmutableTestWasmLibState { proxy: state_proxy() },
	};
	view_iota_balance(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testwasmlib.viewIotaBalance ok");
}

pub struct MapOfArraysLengthContext {
	params: ImmutableMapOfArraysLengthParams,
	results: MutableMapOfArraysLengthResults,
	state: ImmutableTestWasmLibState,
}

fn view_map_of_arrays_length_thunk(ctx: &ScViewContext) {
	ctx.log("testwasmlib.viewMapOfArraysLength");
	let f = MapOfArraysLengthContext {
		params: ImmutableMapOfArraysLengthParams { proxy: params_proxy() },
		results: MutableMapOfArraysLengthResults { proxy: results_proxy() },
		state: ImmutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.name().exists(), "missing mandatory name");
	view_map_of_arrays_length(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testwasmlib.viewMapOfArraysLength ok");
}

pub struct MapOfArraysValueContext {
	params: ImmutableMapOfArraysValueParams,
	results: MutableMapOfArraysValueResults,
	state: ImmutableTestWasmLibState,
}

fn view_map_of_arrays_value_thunk(ctx: &ScViewContext) {
	ctx.log("testwasmlib.viewMapOfArraysValue");
	let f = MapOfArraysValueContext {
		params: ImmutableMapOfArraysValueParams { proxy: params_proxy() },
		results: MutableMapOfArraysValueResults { proxy: results_proxy() },
		state: ImmutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.index().exists(), "missing mandatory index");
	ctx.require(f.params.name().exists(), "missing mandatory name");
	view_map_of_arrays_value(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testwasmlib.viewMapOfArraysValue ok");
}

pub struct MapOfMapsValueContext {
	params: ImmutableMapOfMapsValueParams,
	results: MutableMapOfMapsValueResults,
	state: ImmutableTestWasmLibState,
}

fn view_map_of_maps_value_thunk(ctx: &ScViewContext) {
	ctx.log("testwasmlib.viewMapOfMapsValue");
	let f = MapOfMapsValueContext {
		params: ImmutableMapOfMapsValueParams { proxy: params_proxy() },
		results: MutableMapOfMapsValueResults { proxy: results_proxy() },
		state: ImmutableTestWasmLibState { proxy: state_proxy() },
	};
	ctx.require(f.params.key().exists(), "missing mandatory key");
	ctx.require(f.params.name().exists(), "missing mandatory name");
	view_map_of_maps_value(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testwasmlib.viewMapOfMapsValue ok");
}
