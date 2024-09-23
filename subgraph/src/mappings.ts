import { Protobuf } from "as-proto/assembly";
import { Output as protoOutput } from "./pb/silver_card/v1/Output";
import { Deposit } from "../generated/schema";
import { BigInt, log, crypto, Bytes } from "@graphprotocol/graph-ts";

export function handleTriggers(bytes: Uint8Array): void {
  const input = Protobuf.decode<protoOutput>(bytes, protoOutput.decode);

  // No ID field has been found in the proto input...
  // The input has been hashed to create a unique ID, replace it with the field you want to use as ID
  input.deposits.forEach(deposit => {
    let entity = new Deposit(deposit.txHash);

    entity.txHash = deposit.txHash;
    entity.source = deposit.source;
    entity.destination = deposit.destination;
    entity.inputToken = deposit.inputToken;
    entity.inputAmount = BigInt.fromU64(deposit.inputAmount);
    entity.outputToken = deposit.outputToken;
    entity.outputAmount = BigInt.fromU64(deposit.outputAmount);
    entity.depositType = deposit.depositType
    entity.timestamp = deposit.timestamp

    entity.save();
  });
}
