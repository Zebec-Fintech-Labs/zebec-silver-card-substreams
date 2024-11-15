// Code generated by protoc-gen-as. DO NOT EDIT.
// Versions:
//   protoc-gen-as v1.3.3

import { Writer, Reader } from "as-proto/assembly";

export class WithdrawYield {
  static encode(message: WithdrawYield, writer: Writer): void {
    writer.uint32(10);
    writer.string(message.txHash);

    writer.uint32(18);
    writer.string(message.user);

    writer.uint32(26);
    writer.string(message.userVault);

    writer.uint32(32);
    writer.uint64(message.amount);

    writer.uint32(40);
    writer.bool(message.withdrawAll);

    writer.uint32(48);
    writer.int64(message.timestamp);

    writer.uint32(56);
    writer.uint64(message.slot);

    writer.uint32(64);
    writer.uint64(message.blockHeight);

    writer.uint32(74);
    writer.string(message.blockhash);
  }

  static decode(reader: Reader, length: i32): WithdrawYield {
    const end: usize = length < 0 ? reader.end : reader.ptr + length;
    const message = new WithdrawYield();

    while (reader.ptr < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.txHash = reader.string();
          break;

        case 2:
          message.user = reader.string();
          break;

        case 3:
          message.userVault = reader.string();
          break;

        case 4:
          message.amount = reader.uint64();
          break;

        case 5:
          message.withdrawAll = reader.bool();
          break;

        case 6:
          message.timestamp = reader.int64();
          break;

        case 7:
          message.slot = reader.uint64();
          break;

        case 8:
          message.blockHeight = reader.uint64();
          break;

        case 9:
          message.blockhash = reader.string();
          break;

        default:
          reader.skipType(tag & 7);
          break;
      }
    }

    return message;
  }

  txHash: string;
  user: string;
  userVault: string;
  amount: u64;
  withdrawAll: bool;
  timestamp: i64;
  slot: u64;
  blockHeight: u64;
  blockhash: string;

  constructor(
    txHash: string = "",
    user: string = "",
    userVault: string = "",
    amount: u64 = 0,
    withdrawAll: bool = false,
    timestamp: i64 = 0,
    slot: u64 = 0,
    blockHeight: u64 = 0,
    blockhash: string = ""
  ) {
    this.txHash = txHash;
    this.user = user;
    this.userVault = userVault;
    this.amount = amount;
    this.withdrawAll = withdrawAll;
    this.timestamp = timestamp;
    this.slot = slot;
    this.blockHeight = blockHeight;
    this.blockhash = blockhash;
  }
}
