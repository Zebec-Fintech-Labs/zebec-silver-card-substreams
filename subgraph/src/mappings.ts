import { Protobuf } from 'as-proto/assembly';

import {
  BigInt,
  Value,
} from '@graphprotocol/graph-ts';

import {
  CardPurchase,
  Deposit,
  GenerateYield,
  Withdraw,
  WithdrawYield,
} from '../generated/schema';
import { Output as protoOutput } from './pb/silver_card/v1/Output';

export function handleTriggers(bytes: Uint8Array): void {
	const input = Protobuf.decode<protoOutput>(bytes, protoOutput.decode);

	// No ID field has been found in the proto input...
	// The input has been hashed to create a unique ID, replace it with the field you want to use as ID
	input.deposits.forEach((deposit) => {
		let entity = new Deposit(deposit.txHash);

		entity.txHash = deposit.txHash;
		entity.depositor = deposit.depositor;
		entity.userVault = deposit.userVault;
		entity.inputToken = deposit.inputToken;
		entity.inputAmount = BigInt.fromU64(deposit.inputAmount);
		entity.outputToken = deposit.outputToken;
		entity.outputAmount = BigInt.fromU64(deposit.outputAmount);
		entity.depositType = deposit.depositType;
		entity.timestamp = deposit.timestamp;
		entity.slot = BigInt.fromU64(deposit.slot);
		entity.blockHeight = BigInt.fromU64(deposit.blockHeight);
		entity.blockhash = deposit.blockhash;

		entity.save();
	});

	input.withdraws.forEach((withdraw) => {
		let entity = new Withdraw(withdraw.txHash);

		entity.txHash = withdraw.txHash;
		entity.userVault = withdraw.userVault;
		entity.withdrawer = withdraw.withdrawer;
		entity.token = withdraw.token;
		entity.amount = BigInt.fromU64(withdraw.amount);
		entity.timestamp = withdraw.timestamp;
		entity.slot = BigInt.fromU64(withdraw.slot);
		entity.blockHeight = BigInt.fromU64(withdraw.blockHeight);
		entity.blockhash = withdraw.blockhash;

		entity.save();
	});

	input.cardPurchases.forEach((cardPurchase) => {
		let entity = new CardPurchase(cardPurchase.txHash);

		entity.txHash = cardPurchase.txHash;
		entity.cardId = BigInt.fromU64(cardPurchase.cardId);
		entity.buyer = cardPurchase.buyer;
		entity.buyerVault = cardPurchase.buyerVault;
		entity.amount = BigInt.fromU64(cardPurchase.amount);
		entity.cardType = cardPurchase.cardType;
		entity.timestamp = cardPurchase.timestamp;
		entity.slot = BigInt.fromU64(cardPurchase.slot);
		entity.blockHeight = BigInt.fromU64(cardPurchase.blockHeight);
		entity.blockhash = cardPurchase.blockhash;

		entity.save();
	});

	input.generateYields.forEach((generateYield) => {
		let entity = new GenerateYield(generateYield.txHash);

		entity.txHash = generateYield.txHash;
		entity.user = generateYield.user;
		entity.userVault = generateYield.userVault;
		entity.amount = BigInt.fromU64(generateYield.amount);
		entity.timestamp = generateYield.timestamp;
		entity.slot = BigInt.fromU64(generateYield.slot);
		entity.blockHeight = BigInt.fromU64(generateYield.blockHeight);
		entity.blockhash = generateYield.blockhash;

		entity.save();
	});

	input.withdrawYields.forEach((withdrawYield) => {
		let entity = new WithdrawYield(withdrawYield.txHash);

		entity.txHash = withdrawYield.txHash;
		entity.user = withdrawYield.user;
		entity.userVault = withdrawYield.userVault;
		entity.amount = BigInt.fromU64(withdrawYield.amount);
		entity.withdrawAll = Value.fromBoolean(withdrawYield.withdrawAll).toBoolean();
		entity.timestamp = withdrawYield.timestamp;
		entity.slot = BigInt.fromU64(withdrawYield.slot);
		entity.blockHeight = BigInt.fromU64(withdrawYield.blockHeight);
		entity.blockhash = withdrawYield.blockhash;

		entity.save();
	});
}
