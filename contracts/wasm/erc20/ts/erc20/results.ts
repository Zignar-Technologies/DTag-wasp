// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmtypes from "wasmlib/wasmtypes";
import * as sc from "./index";

export class ImmutableAllowanceResults extends wasmtypes.ScProxy {
	amount(): wasmtypes.ScImmutableUint64 {
		return new wasmtypes.ScImmutableUint64(this.proxy.root(sc.ResultAmount));
	}
}

export class MutableAllowanceResults extends wasmtypes.ScProxy {
	amount(): wasmtypes.ScMutableUint64 {
		return new wasmtypes.ScMutableUint64(this.proxy.root(sc.ResultAmount));
	}
}

export class ImmutableBalanceOfResults extends wasmtypes.ScProxy {
	amount(): wasmtypes.ScImmutableUint64 {
		return new wasmtypes.ScImmutableUint64(this.proxy.root(sc.ResultAmount));
	}
}

export class MutableBalanceOfResults extends wasmtypes.ScProxy {
	amount(): wasmtypes.ScMutableUint64 {
		return new wasmtypes.ScMutableUint64(this.proxy.root(sc.ResultAmount));
	}
}

export class ImmutableTotalSupplyResults extends wasmtypes.ScProxy {
	supply(): wasmtypes.ScImmutableUint64 {
		return new wasmtypes.ScImmutableUint64(this.proxy.root(sc.ResultSupply));
	}
}

export class MutableTotalSupplyResults extends wasmtypes.ScProxy {
	supply(): wasmtypes.ScMutableUint64 {
		return new wasmtypes.ScMutableUint64(this.proxy.root(sc.ResultSupply));
	}
}
