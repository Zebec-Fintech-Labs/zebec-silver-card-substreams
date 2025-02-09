import { Protobuf } from 'as-proto/assembly';

import {
  BigInt,
  Value,
} from '@graphprotocol/graph-ts';

import {
  CardBotDirectCardPurchase,
  CardPurchase,
  Deposit,
  DirectCardPurchase,
  GenerateYield,
  InitCardBotUserAccount,
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

		entity.slot = BigInt.fromU64(deposit.slot);
		entity.blockHeight = BigInt.fromU64(deposit.blockHeight);
		entity.blockhash = deposit.blockhash;
		entity.timestamp = deposit.timestamp;

		entity.inputAmount = BigInt.fromU64(deposit.inputAmount);
		entity.outputAmount = BigInt.fromU64(deposit.outputAmount);
		entity.inputToken = deposit.inputToken;
		entity.outputToken = deposit.outputToken;
		entity.depositor = deposit.depositor;
		entity.userVault = deposit.userVault;
		entity.depositType = deposit.depositType;
		entity.purchaseRecord = deposit.purchaseRecord;

		entity.save();
	});

	input.withdraws.forEach((withdraw) => {
		let entity = new Withdraw(withdraw.txHash);

		entity.slot = BigInt.fromU64(withdraw.slot);
		entity.blockHeight = BigInt.fromU64(withdraw.blockHeight);
		entity.blockhash = withdraw.blockhash;
		entity.timestamp = withdraw.timestamp;

		entity.amount = BigInt.fromU64(withdraw.amount);
		entity.token = withdraw.token;
		entity.userVault = withdraw.userVault;
		entity.withdrawer = withdraw.withdrawer;

		entity.save();
	});

	input.cardPurchases.forEach((cardPurchase) => {
		let entity = new CardPurchase(cardPurchase.txHash);

		entity.slot = BigInt.fromU64(cardPurchase.slot);
		entity.blockHeight = BigInt.fromU64(cardPurchase.blockHeight);
		entity.blockhash = cardPurchase.blockhash;
		entity.timestamp = cardPurchase.timestamp;

		entity.cardId = BigInt.fromU64(cardPurchase.cardId);
		entity.cardType = cardPurchase.cardType;
		entity.amount = BigInt.fromU64(cardPurchase.amount);
		entity.buyer = cardPurchase.buyer;
		entity.buyerVault = cardPurchase.buyerVault;
		entity.purchaseRecord = cardPurchase.purchaseRecord;

		entity.save();
	});

	input.generateYields.forEach((generateYield) => {
		let entity = new GenerateYield(generateYield.txHash);

		entity.slot = BigInt.fromU64(generateYield.slot);
		entity.blockHeight = BigInt.fromU64(generateYield.blockHeight);
		entity.blockhash = generateYield.blockhash;
		entity.timestamp = generateYield.timestamp;

		entity.amount = BigInt.fromU64(generateYield.amount);
		entity.user = generateYield.user;
		entity.userVault = generateYield.userVault;

		entity.save();
	});

	input.withdrawYields.forEach((withdrawYield) => {
		let entity = new WithdrawYield(withdrawYield.txHash);

		entity.slot = BigInt.fromU64(withdrawYield.slot);
		entity.blockHeight = BigInt.fromU64(withdrawYield.blockHeight);
		entity.blockhash = withdrawYield.blockhash;
		entity.timestamp = withdrawYield.timestamp;

		entity.amount = BigInt.fromU64(withdrawYield.amount);
		entity.withdrawAll = Value.fromBoolean(withdrawYield.withdrawAll).toBoolean();
		entity.user = withdrawYield.user;
		entity.userVault = withdrawYield.userVault;

		entity.save();
	});

	input.directCardPurhcases.forEach((directCardPurchase) => {
		let entity = new DirectCardPurchase(directCardPurchase.txHash);

		entity.slot = BigInt.fromU64(directCardPurchase.slot);
		entity.blockHeight = BigInt.fromU64(directCardPurchase.blockHeight);
		entity.blockhash = directCardPurchase.blockhash;
		entity.timestamp = directCardPurchase.timestamp;

		entity.inputAmount = BigInt.fromU64(directCardPurchase.inputAmount);
		entity.outputAmount = BigInt.fromU64(directCardPurchase.outputAmount);
		entity.inputToken = directCardPurchase.inputToken;
		entity.outputToken = directCardPurchase.outputToken;
		entity.cardId = BigInt.fromU64(directCardPurchase.cardId);
		entity.cardType = directCardPurchase.cardType;
		entity.buyer = directCardPurchase.buyer;
		entity.buyerPurchase = directCardPurchase.buyerPurchase;
		entity.purchaseRecord = directCardPurchase.purchaseRecord;

		entity.save();
	});

	input.cardBotUserAccountInits.forEach((cardBotUserAccountInit) => {
		let entity = new InitCardBotUserAccount(cardBotUserAccountInit.txHash);

		entity.slot = BigInt.fromU64(cardBotUserAccountInit.slot);
		entity.blockHeight = BigInt.fromU64(cardBotUserAccountInit.blockHeight);
		entity.blockhash = cardBotUserAccountInit.blockhash;
		entity.timestamp = cardBotUserAccountInit.timestamp;

		entity.userId = cardBotUserAccountInit.userId;
		entity.userCustody = cardBotUserAccountInit.userCustody;
		entity.usdcToken = cardBotUserAccountInit.usdcToken;
		entity.admin = cardBotUserAccountInit.admin;

		entity.save();
	});

	input.cardBotDirectCardPurchases.forEach((cardBotDirectCardPurchase) => {
		let entity = new CardBotDirectCardPurchase(cardBotDirectCardPurchase.txHash);

		entity.slot = BigInt.fromU64(cardBotDirectCardPurchase.slot);
		entity.blockHeight = BigInt.fromU64(cardBotDirectCardPurchase.blockHeight);
		entity.blockhash = cardBotDirectCardPurchase.blockhash;
		entity.timestamp = cardBotDirectCardPurchase.timestamp;

		entity.userId = cardBotDirectCardPurchase.userId;
		entity.inputAmount = BigInt.fromU64(cardBotDirectCardPurchase.inputAmount);
		entity.outputAmount = BigInt.fromU64(cardBotDirectCardPurchase.outputAmount);
		entity.inputToken = cardBotDirectCardPurchase.inputToken;
		entity.outputToken = cardBotDirectCardPurchase.outputToken;
		entity.userCustody = cardBotDirectCardPurchase.userCustody;
		entity.admin = cardBotDirectCardPurchase.admin;

		entity.save();
	});
}
