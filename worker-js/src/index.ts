import { Connection, PublicKey } from "@solana/web3.js";
import { SolanaParser } from "@sonarwatch/solana-transaction-parser";
import { Idl } from "@project-serum/anchor";
import { getGmIDL } from "@staratlas/factory";
import { collections, connectToDatabase } from "./mongodb/mongodbconnection";
import { IDBObject } from "./types/DBObjects";
import { sleeper } from "./sleeper";
import { localStoreInstance } from "./store/LocalStoreAdapter";
import * as dotenv from "dotenv";
dotenv.config();

interface Stats {
  total: number;
  exchanges: number;
  counter: number;
  creates: number;
  cancels: number;
  direct: number;
  unmapped: number;
  written_to_db: number;
}

const solanaConnection = new Connection(
  process.env.RPC ?? "https://api.mainnet-beta.solana.com"
);
const program_pubKey = new PublicKey(
  "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg"
);

const execute = async () => {
  console.log("-- Staring --");
  await localStoreInstance.init();
  await connectToDatabase();

  switch (process.env.MODE) {
    case "sync": {
      let last_signature = undefined;
      while (true) {
        last_signature = await fetch_and_map_task(last_signature, undefined);
        await sleeper(parseInt(process.env.SLEEP ?? "10000"));
      }
    }
    default: {
      let last_signature: string | undefined = undefined;
      await solanaConnection.onProgramAccountChange(
        program_pubKey,
        async (account, context) => {
          last_signature = await fetch_and_map_task(undefined, last_signature);
        },
        "finalized"
      );
    }
  }
};

execute().catch((err) => {
  console.log(err);
});

async function fetch_and_map_task(
  before?: string,
  until?: string
): Promise<string> {
  let last_signature = "";
  let last_timestamp = 0;
  let stats: Stats = {
    total: 0,
    exchanges: 0,
    counter: 0,
    creates: 0,
    cancels: 0,
    direct: 0,
    unmapped: 0,
    written_to_db: 0,
  };

  const txParser = new SolanaParser([
    {
      idl: getGmIDL(program_pubKey) as unknown as Idl,
      programId: program_pubKey.toString(),
    },
  ]);

  let transactionList = await solanaConnection.getSignaturesForAddress(
    program_pubKey,
    {
      limit: 10,
      before: before,
      until: until,
    }
  );

  stats.total = transactionList.length;

  let signatureList = transactionList.map(
    (transaction) => transaction.signature
  );

  for (const transaction of transactionList) {
    const parsed = await txParser.parseTransaction(
      solanaConnection,
      transaction.signature,
      false
    );

    const db_data: IDBObject = {
      signature: transaction.signature,
      timestamp: transaction.blockTime,
      data: JSON.parse(JSON.stringify(parsed)),
      symbol: "none",
    };

    switch (parsed?.[0].name ?? "") {
      case "processExchange": {
        const currency_mint = parsed?.[0].accounts
          .find((account) => account.name == "currencyMint")
          ?.pubkey.toString();
        const asset_mint = parsed?.[0].accounts
          .find((account) => account.name == "assetMint")
          ?.pubkey.toString();

        let d = parsed?.[0].args as any;
        db_data.size = parseInt(d.purchaseQuantity.toString());
        db_data.price = parseInt(d.expectedPrice.toString());
        db_data.symbol =
          localStoreInstance.symbolsStore.find(
            (symbol) =>
              symbol.mint === asset_mint && symbol.pair?.mint === currency_mint
          )?.symbol_pair ?? "not-found";

        await collections.processExchange?.insertOne(db_data).catch((err) => {
          if (err.code != 11000) throw err;
        });
        stats.exchanges++;
        break;
      }
      case "initializeOpenOrdersCounter": {
        await collections.counterExchange?.insertOne(db_data).catch((err) => {
          if (err.code != 11000) throw err;
        });
        stats.counter++;
        break;
      }
      case "createAccount": {
        await collections.createExchange?.insertOne(db_data).catch((err) => {
          if (err.code != 11000) throw err;
        });
        stats.creates++;
        break;
      }
      case "processCancel": {
        await collections.cancelExchange?.insertOne(db_data).catch((err) => {
          if (err.code != 11000) throw err;
        });
        stats.cancels++;
        break;
      }
      case "createAssociatedTokenAccount": {
        await collections.cancelExchange?.insertOne(db_data).catch((err) => {
          if (err.code != 11000) throw err;
        });
        stats.direct++;
        break;
      }
      default: {
        await collections.unmappedExchange
          ?.insertOne(db_data)
          .then(() => stats.written_to_db++)
          .catch((err) => {
            if (err.code != 11000) throw err;
          });
        stats.unmapped++;
        break;
      }
    }
    stats.written_to_db++;
    transaction.blockTime;
    last_signature = transaction.signature;
    last_timestamp = transaction.blockTime ?? 0;
  }

  console.log(
    `${process.env.MODE}: total=${stats.total}, exchanges=${stats.exchanges}, counter=${stats.counter}, created=${stats.creates}, canceled=${stats.cancels}, direct=${stats.direct} unmapped=${stats.unmapped} written_to_db=${stats.written_to_db}`
  );
  console.log(`${last_signature} - ${new Date(last_timestamp * 1000)}`);

  return last_signature;
}