// THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.

import {
  TypedMap,
  Entity,
  Value,
  ValueKind,
  store,
  Bytes,
  BigInt,
  BigDecimal,
} from "@graphprotocol/graph-ts";

export class Deposit extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Deposit entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Deposit must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`,
      );
      store.set("Deposit", id.toString(), this);
    }
  }

  static loadInBlock(id: string): Deposit | null {
    return changetype<Deposit | null>(store.get_in_block("Deposit", id));
  }

  static load(id: string): Deposit | null {
    return changetype<Deposit | null>(store.get("Deposit", id));
  }

  get id(): string {
    let value = this.get("id");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get txHash(): string {
    let value = this.get("txHash");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set txHash(value: string) {
    this.set("txHash", Value.fromString(value));
  }

  get source(): string {
    let value = this.get("source");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set source(value: string) {
    this.set("source", Value.fromString(value));
  }

  get destination(): string {
    let value = this.get("destination");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set destination(value: string) {
    this.set("destination", Value.fromString(value));
  }

  get inputToken(): string {
    let value = this.get("inputToken");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set inputToken(value: string) {
    this.set("inputToken", Value.fromString(value));
  }

  get outputToken(): string {
    let value = this.get("outputToken");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set outputToken(value: string) {
    this.set("outputToken", Value.fromString(value));
  }

  get inputAmount(): BigInt {
    let value = this.get("inputAmount");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toBigInt();
    }
  }

  set inputAmount(value: BigInt) {
    this.set("inputAmount", Value.fromBigInt(value));
  }

  get outputAmount(): BigInt {
    let value = this.get("outputAmount");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toBigInt();
    }
  }

  set outputAmount(value: BigInt) {
    this.set("outputAmount", Value.fromBigInt(value));
  }

  get depositType(): i32 {
    let value = this.get("depositType");
    if (!value || value.kind == ValueKind.NULL) {
      return 0;
    } else {
      return value.toI32();
    }
  }

  set depositType(value: i32) {
    this.set("depositType", Value.fromI32(value));
  }

  get timestamp(): i64 {
    let value = this.get("timestamp");
    if (!value || value.kind == ValueKind.NULL) {
      return 0;
    } else {
      return value.toTimestamp();
    }
  }

  set timestamp(value: i64) {
    this.set("timestamp", Value.fromTimestamp(value));
  }
}
