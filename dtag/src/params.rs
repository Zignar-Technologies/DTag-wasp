// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;
use wasmlib::host::*;

use crate::*;
use crate::keys::*;

#[derive(Clone, Copy)]
pub struct ImmutableCreateGameParams {
    pub(crate) id: i32,
}

impl ImmutableCreateGameParams {
    pub fn description(&self) -> ScImmutableString {
		ScImmutableString::new(self.id, idx_map(IDX_PARAM_DESCRIPTION))
	}

    pub fn number_of_images(&self) -> ScImmutableInt32 {
		ScImmutableInt32::new(self.id, idx_map(IDX_PARAM_NUMBER_OF_IMAGES))
	}

    pub fn tags_required_per_image(&self) -> ScImmutableInt32 {
		ScImmutableInt32::new(self.id, idx_map(IDX_PARAM_TAGS_REQUIRED_PER_IMAGE))
	}
}

#[derive(Clone, Copy)]
pub struct MutableCreateGameParams {
    pub(crate) id: i32,
}

impl MutableCreateGameParams {
    pub fn description(&self) -> ScMutableString {
		ScMutableString::new(self.id, idx_map(IDX_PARAM_DESCRIPTION))
	}

    pub fn number_of_images(&self) -> ScMutableInt32 {
		ScMutableInt32::new(self.id, idx_map(IDX_PARAM_NUMBER_OF_IMAGES))
	}

    pub fn tags_required_per_image(&self) -> ScMutableInt32 {
		ScMutableInt32::new(self.id, idx_map(IDX_PARAM_TAGS_REQUIRED_PER_IMAGE))
	}
}

#[derive(Clone, Copy)]
pub struct ImmutableInitParams {
    pub(crate) id: i32,
}

impl ImmutableInitParams {
    pub fn owner(&self) -> ScImmutableAgentID {
		ScImmutableAgentID::new(self.id, idx_map(IDX_PARAM_OWNER))
	}
}

#[derive(Clone, Copy)]
pub struct MutableInitParams {
    pub(crate) id: i32,
}

impl MutableInitParams {
    pub fn owner(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.id, idx_map(IDX_PARAM_OWNER))
	}
}

#[derive(Clone, Copy)]
pub struct ImmutableSendTagsParams {
    pub(crate) id: i32,
}

impl ImmutableSendTagsParams {
    pub fn boost(&self) -> ScImmutableInt32 {
		ScImmutableInt32::new(self.id, idx_map(IDX_PARAM_BOOST))
	}

    pub fn h(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, idx_map(IDX_PARAM_H))
	}

    pub fn w(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, idx_map(IDX_PARAM_W))
	}

    pub fn x(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, idx_map(IDX_PARAM_X))
	}

    pub fn y(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, idx_map(IDX_PARAM_Y))
	}
}

#[derive(Clone, Copy)]
pub struct MutableSendTagsParams {
    pub(crate) id: i32,
}

impl MutableSendTagsParams {
    pub fn boost(&self) -> ScMutableInt32 {
		ScMutableInt32::new(self.id, idx_map(IDX_PARAM_BOOST))
	}

    pub fn h(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, idx_map(IDX_PARAM_H))
	}

    pub fn w(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, idx_map(IDX_PARAM_W))
	}

    pub fn x(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, idx_map(IDX_PARAM_X))
	}

    pub fn y(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, idx_map(IDX_PARAM_Y))
	}
}

#[derive(Clone, Copy)]
pub struct ImmutableSetOwnerParams {
    pub(crate) id: i32,
}

impl ImmutableSetOwnerParams {
    pub fn owner(&self) -> ScImmutableAgentID {
		ScImmutableAgentID::new(self.id, idx_map(IDX_PARAM_OWNER))
	}
}

#[derive(Clone, Copy)]
pub struct MutableSetOwnerParams {
    pub(crate) id: i32,
}

impl MutableSetOwnerParams {
    pub fn owner(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.id, idx_map(IDX_PARAM_OWNER))
	}
}

#[derive(Clone, Copy)]
pub struct ImmutableGetPlaysPerImageParams {
    pub(crate) id: i32,
}

impl ImmutableGetPlaysPerImageParams {
    pub fn image_id(&self) -> ScImmutableInt32 {
		ScImmutableInt32::new(self.id, idx_map(IDX_PARAM_IMAGE_ID))
	}
}

#[derive(Clone, Copy)]
pub struct MutableGetPlaysPerImageParams {
    pub(crate) id: i32,
}

impl MutableGetPlaysPerImageParams {
    pub fn image_id(&self) -> ScMutableInt32 {
		ScMutableInt32::new(self.id, idx_map(IDX_PARAM_IMAGE_ID))
	}
}

#[derive(Clone, Copy)]
pub struct ImmutableGetResultsParams {
    pub(crate) id: i32,
}

impl ImmutableGetResultsParams {
    pub fn image_id(&self) -> ScImmutableInt32 {
		ScImmutableInt32::new(self.id, idx_map(IDX_PARAM_IMAGE_ID))
	}
}

#[derive(Clone, Copy)]
pub struct MutableGetResultsParams {
    pub(crate) id: i32,
}

impl MutableGetResultsParams {
    pub fn image_id(&self) -> ScMutableInt32 {
		ScMutableInt32::new(self.id, idx_map(IDX_PARAM_IMAGE_ID))
	}
}
