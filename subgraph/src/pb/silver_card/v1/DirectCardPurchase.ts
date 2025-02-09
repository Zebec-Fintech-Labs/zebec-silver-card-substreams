// Code generated by protoc-gen-as. DO NOT EDIT.
// Versions:
//   protoc-gen-as v1.3.3

import { Writer, Reader } from "as-proto/assembly";

export class DirectCardPurchase {
  static encode(message: DirectCardPurchase, writer: Writer): void {
    writer.uint32(8);
    writer.uint64(message.slot);

    writer.uint32(16);
    writer.uint64(message.blockHeight);

    writer.uint32(26);
    writer.string(message.blockhash);

    writer.uint32(32);
    writer.int64(message.timestamp);

    writer.uint32(42);
    writer.string(message.txHash);

    writer.uint32(50);
    writer.string(message.inputToken);

    writer.uint32(58);
    writer.string(message.outputToken);

    writer.uint32(64);
    writer.uint64(message.inputAmount);

    writer.uint32(72);
    writer.uint64(message.outputAmount);

    writer.uint32(80);
    writer.uint64(message.cardId);

    writer.uint32(90);
    writer.string(message.cardType);

    writer.uint32(98);
    writer.string(message.buyer);

    writer.uint32(106);
    writer.string(message.buyerPurchase);

    writer.uint32(114);
    writer.string(message.purchaseRecord);
  }

  static decode(reader: Reader, length: i32): DirectCardPurchase {
    const end: usize = length < 0 ? reader.end : reader.ptr + length;
    const message = new DirectCardPurchase();

    while (reader.ptr < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.slot = reader.uint64();
          break;

        case 2:
          message.blockHeight = reader.uint64();
          break;

        case 3:
          message.blockhash = reader.string();
          break;

        case 4:
          message.timestamp = reader.int64();
          break;

        case 5:
          message.txHash = reader.string();
          break;

        case 6:
          message.inputToken = reader.string();
          break;

        case 7:
          message.outputToken = reader.string();
          break;

        case 8:
          message.inputAmount = reader.uint64();
          break;

        case 9:
          message.outputAmount = reader.uint64();
          break;

        case 10:
          message.cardId = reader.uint64();
          break;

        case 11:
          message.cardType = reader.string();
          break;

        case 12:
          message.buyer = reader.string();
          break;

        case 13:
          message.buyerPurchase = reader.string();
          break;

        case 14:
          message.purchaseRecord = reader.string();
          break;

        default:
          reader.skipType(tag & 7);
          break;
      }
    }

    return message;
  }

  slot: u64;
  blockHeight: u64;
  blockhash: string;
  timestamp: i64;
  txHash: string;
  inputToken: string;
  outputToken: string;
  inputAmount: u64;
  outputAmount: u64;
  cardId: u64;
  cardType: string;
  buyer: string;
  buyerPurchase: string;
  purchaseRecord: string;

  constructor(
    slot: u64 = 0,
    blockHeight: u64 = 0,
    blockhash: string = "",
    timestamp: i64 = 0,
    txHash: string = "",
    inputToken: string = "",
    outputToken: string = "",
    inputAmount: u64 = 0,
    outputAmount: u64 = 0,
    cardId: u64 = 0,
    cardType: string = "",
    buyer: string = "",
    buyerPurchase: string = "",
    purchaseRecord: string = ""
  ) {
    this.slot = slot;
    this.blockHeight = blockHeight;
    this.blockhash = blockhash;
    this.timestamp = timestamp;
    this.txHash = txHash;
    this.inputToken = inputToken;
    this.outputToken = outputToken;
    this.inputAmount = inputAmount;
    this.outputAmount = outputAmount;
    this.cardId = cardId;
    this.cardType = cardType;
    this.buyer = buyer;
    this.buyerPurchase = buyerPurchase;
    this.purchaseRecord = purchaseRecord;
  }
}
