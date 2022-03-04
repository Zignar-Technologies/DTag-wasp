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
use crate::structs::*;

#[derive(Clone, Copy)]
pub struct ArrayOfImmutableBet {
	pub(crate) obj_id: i32,
}

impl ArrayOfImmutableBet {
    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

	pub fn get_bet(&self, index: i32) -> ImmutableBet {
		ImmutableBet { obj_id: self.obj_id, key_id: Key32(index) }
	}
}

#[derive(Clone, Copy)]
pub struct MapStringToImmutableBet {
	pub(crate) obj_id: i32,
}

impl MapStringToImmutableBet {
    pub fn get_bet(&self, key: &str) -> ImmutableBet {
        ImmutableBet { obj_id: self.obj_id, key_id: key.get_key_id() }
    }
}

#[derive(Clone, Copy)]
pub struct MapStringToImmutablePlayer {
	pub(crate) obj_id: i32,
}

impl MapStringToImmutablePlayer {
    pub fn get_player(&self, key: &str) -> ImmutablePlayer {
        ImmutablePlayer { obj_id: self.obj_id, key_id: key.get_key_id() }
    }
}

#[derive(Clone, Copy)]
pub struct ArrayOfImmutablePlayer {
	pub(crate) obj_id: i32,
}

impl ArrayOfImmutablePlayer {
    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

	pub fn get_player(&self, index: i32) -> ImmutablePlayer {
		ImmutablePlayer { obj_id: self.obj_id, key_id: Key32(index) }
	}
}

#[derive(Clone, Copy)]
pub struct ArrayOfImmutableInt32 {
	pub(crate) obj_id: i32,
}

impl ArrayOfImmutableInt32 {
    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

    pub fn get_int32(&self, index: i32) -> ScImmutableInt32 {
        ScImmutableInt32::new(self.obj_id, Key32(index))
    }
}

#[derive(Clone, Copy)]
pub struct ArrayOfImmutableTaggedImage {
	pub(crate) obj_id: i32,
}

impl ArrayOfImmutableTaggedImage {
    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

	pub fn get_tagged_image(&self, index: i32) -> ImmutableTaggedImage {
		ImmutableTaggedImage { obj_id: self.obj_id, key_id: Key32(index) }
	}
}

#[derive(Clone, Copy)]
pub struct ArrayOfImmutableValidTag {
	pub(crate) obj_id: i32,
}

impl ArrayOfImmutableValidTag {
    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

	pub fn get_valid_tag(&self, index: i32) -> ImmutableValidTag {
		ImmutableValidTag { obj_id: self.obj_id, key_id: Key32(index) }
	}
}

#[derive(Clone, Copy)]
pub struct ImmutablezentangleState {
    pub(crate) id: i32,
}

impl ImmutablezentangleState {
    pub fn bets(&self) -> ArrayOfImmutableBet {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_BETS), TYPE_ARRAY | TYPE_BYTES);
		ArrayOfImmutableBet { obj_id: arr_id }
	}

    pub fn creator(&self) -> ScImmutableAgentID {
		ScImmutableAgentID::new(self.id, idx_map(IDX_STATE_CREATOR))
	}

    pub fn description(&self) -> ScImmutableString {
		ScImmutableString::new(self.id, idx_map(IDX_STATE_DESCRIPTION))
	}

    pub fn number_of_images(&self) -> ScImmutableInt32 {
		ScImmutableInt32::new(self.id, idx_map(IDX_STATE_NUMBER_OF_IMAGES))
	}

    pub fn owner(&self) -> ScImmutableAgentID {
		ScImmutableAgentID::new(self.id, idx_map(IDX_STATE_OWNER))
	}

    pub fn pending_play(&self) -> MapStringToImmutableBet {
		let map_id = get_object_id(self.id, idx_map(IDX_STATE_PENDING_PLAY), TYPE_MAP);
		MapStringToImmutableBet { obj_id: map_id }
	}

    pub fn pending_plays(&self) -> ArrayOfImmutableBet {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PENDING_PLAYS), TYPE_ARRAY | TYPE_BYTES);
		ArrayOfImmutableBet { obj_id: arr_id }
	}

    pub fn player(&self) -> MapStringToImmutablePlayer {
		let map_id = get_object_id(self.id, idx_map(IDX_STATE_PLAYER), TYPE_MAP);
		MapStringToImmutablePlayer { obj_id: map_id }
	}

    pub fn players(&self) -> ArrayOfImmutablePlayer {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PLAYERS), TYPE_ARRAY | TYPE_BYTES);
		ArrayOfImmutablePlayer { obj_id: arr_id }
	}

    pub fn plays_per_image(&self) -> ArrayOfImmutableInt32 {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PLAYS_PER_IMAGE), TYPE_ARRAY | TYPE_INT32);
		ArrayOfImmutableInt32 { obj_id: arr_id }
	}

    pub fn processed_images(&self) -> ArrayOfImmutableTaggedImage {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PROCESSED_IMAGES), TYPE_ARRAY | TYPE_BYTES);
		ArrayOfImmutableTaggedImage { obj_id: arr_id }
	}

    pub fn reward(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, idx_map(IDX_STATE_REWARD))
	}

    pub fn tagged_images(&self) -> ArrayOfImmutableTaggedImage {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_TAGGED_IMAGES), TYPE_ARRAY | TYPE_BYTES);
		ArrayOfImmutableTaggedImage { obj_id: arr_id }
	}

    pub fn tags_required_per_image(&self) -> ScImmutableInt32 {
		ScImmutableInt32::new(self.id, idx_map(IDX_STATE_TAGS_REQUIRED_PER_IMAGE))
	}

    pub fn valid_tags(&self) -> ArrayOfImmutableValidTag {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_VALID_TAGS), TYPE_ARRAY | TYPE_BYTES);
		ArrayOfImmutableValidTag { obj_id: arr_id }
	}
}

#[derive(Clone, Copy)]
pub struct ArrayOfMutableBet {
	pub(crate) obj_id: i32,
}

impl ArrayOfMutableBet {
    pub fn clear(&self) {
        clear(self.obj_id);
    }

    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

	pub fn get_bet(&self, index: i32) -> MutableBet {
		MutableBet { obj_id: self.obj_id, key_id: Key32(index) }
	}
}

#[derive(Clone, Copy)]
pub struct MapStringToMutableBet {
	pub(crate) obj_id: i32,
}

impl MapStringToMutableBet {
    pub fn clear(&self) {
        clear(self.obj_id);
    }

    pub fn get_bet(&self, key: &str) -> MutableBet {
        MutableBet { obj_id: self.obj_id, key_id: key.get_key_id() }
    }
}

#[derive(Clone, Copy)]
pub struct MapStringToMutablePlayer {
	pub(crate) obj_id: i32,
}

impl MapStringToMutablePlayer {
    pub fn clear(&self) {
        clear(self.obj_id);
    }

    pub fn get_player(&self, key: &str) -> MutablePlayer {
        MutablePlayer { obj_id: self.obj_id, key_id: key.get_key_id() }
    }
}

#[derive(Clone, Copy)]
pub struct ArrayOfMutablePlayer {
	pub(crate) obj_id: i32,
}

impl ArrayOfMutablePlayer {
    pub fn clear(&self) {
        clear(self.obj_id);
    }

    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

	pub fn get_player(&self, index: i32) -> MutablePlayer {
		MutablePlayer { obj_id: self.obj_id, key_id: Key32(index) }
	}
}

#[derive(Clone, Copy)]
pub struct ArrayOfMutableInt32 {
	pub(crate) obj_id: i32,
}

impl ArrayOfMutableInt32 {
    pub fn clear(&self) {
        clear(self.obj_id);
    }

    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

    pub fn get_int32(&self, index: i32) -> ScMutableInt32 {
        ScMutableInt32::new(self.obj_id, Key32(index))
    }
}

#[derive(Clone, Copy)]
pub struct ArrayOfMutableTaggedImage {
	pub(crate) obj_id: i32,
}

impl ArrayOfMutableTaggedImage {
    pub fn clear(&self) {
        clear(self.obj_id);
    }

    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

	pub fn get_tagged_image(&self, index: i32) -> MutableTaggedImage {
		MutableTaggedImage { obj_id: self.obj_id, key_id: Key32(index) }
	}
}

#[derive(Clone, Copy)]
pub struct ArrayOfMutableValidTag {
	pub(crate) obj_id: i32,
}

impl ArrayOfMutableValidTag {
    pub fn clear(&self) {
        clear(self.obj_id);
    }

    pub fn length(&self) -> i32 {
        get_length(self.obj_id)
    }

	pub fn get_valid_tag(&self, index: i32) -> MutableValidTag {
		MutableValidTag { obj_id: self.obj_id, key_id: Key32(index) }
	}
}

#[derive(Clone, Copy)]
pub struct MutablezentangleState {
    pub(crate) id: i32,
}

impl MutablezentangleState {
    pub fn as_immutable(&self) -> ImmutablezentangleState {
		ImmutablezentangleState { id: self.id }
	}

    pub fn bets(&self) -> ArrayOfMutableBet {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_BETS), TYPE_ARRAY | TYPE_BYTES);
		ArrayOfMutableBet { obj_id: arr_id }
	}

    pub fn creator(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.id, idx_map(IDX_STATE_CREATOR))
	}

    pub fn description(&self) -> ScMutableString {
		ScMutableString::new(self.id, idx_map(IDX_STATE_DESCRIPTION))
	}

    pub fn number_of_images(&self) -> ScMutableInt32 {
		ScMutableInt32::new(self.id, idx_map(IDX_STATE_NUMBER_OF_IMAGES))
	}

    pub fn owner(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.id, idx_map(IDX_STATE_OWNER))
	}

    pub fn pending_play(&self) -> MapStringToMutableBet {
		let map_id = get_object_id(self.id, idx_map(IDX_STATE_PENDING_PLAY), TYPE_MAP);
		MapStringToMutableBet { obj_id: map_id }
	}

    pub fn pending_plays(&self) -> ArrayOfMutableBet {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PENDING_PLAYS), TYPE_ARRAY | TYPE_BYTES);
		ArrayOfMutableBet { obj_id: arr_id }
	}

    pub fn player(&self) -> MapStringToMutablePlayer {
		let map_id = get_object_id(self.id, idx_map(IDX_STATE_PLAYER), TYPE_MAP);
		MapStringToMutablePlayer { obj_id: map_id }
	}

    pub fn players(&self) -> ArrayOfMutablePlayer {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PLAYERS), TYPE_ARRAY | TYPE_BYTES);
		ArrayOfMutablePlayer { obj_id: arr_id }
	}

    pub fn plays_per_image(&self) -> ArrayOfMutableInt32 {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PLAYS_PER_IMAGE), TYPE_ARRAY | TYPE_INT32);
		ArrayOfMutableInt32 { obj_id: arr_id }
	}

    pub fn processed_images(&self) -> ArrayOfMutableTaggedImage {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_PROCESSED_IMAGES), TYPE_ARRAY | TYPE_BYTES);
		ArrayOfMutableTaggedImage { obj_id: arr_id }
	}

    pub fn reward(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, idx_map(IDX_STATE_REWARD))
	}

    pub fn tagged_images(&self) -> ArrayOfMutableTaggedImage {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_TAGGED_IMAGES), TYPE_ARRAY | TYPE_BYTES);
		ArrayOfMutableTaggedImage { obj_id: arr_id }
	}

    pub fn tags_required_per_image(&self) -> ScMutableInt32 {
		ScMutableInt32::new(self.id, idx_map(IDX_STATE_TAGS_REQUIRED_PER_IMAGE))
	}

    pub fn valid_tags(&self) -> ArrayOfMutableValidTag {
		let arr_id = get_object_id(self.id, idx_map(IDX_STATE_VALID_TAGS), TYPE_ARRAY | TYPE_BYTES);
		ArrayOfMutableValidTag { obj_id: arr_id }
	}
}
