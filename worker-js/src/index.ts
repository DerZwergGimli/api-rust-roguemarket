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
  process.env.RPC ?? "https://api.mainnet-beta.solana.com",
  { commitment: "finalized" }
);
const program_pubKey = new PublicKey(
  "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg"
);

const execute = async () => {
  console.log("-- Staring --");
  await localStoreInstance.init();
  await connectToDatabase();

  let last_signature: string | undefined = process.env.LASTSIG ?? undefined;
  switch (process.env.MODE) {
    case "sync": {
      while (true) {
        last_signature = await fetch_and_map_task(last_signature, undefined);
        await sleeper(parseInt(process.env.SLEEP ?? "10000"));
      }
    }
    default: {
      while (true) {
        last_signature = await fetch_and_map_task(undefined, undefined);
        await sleeper(parseInt(process.env.SLEEP ?? "10000"));
      }
      /*solanaConnection.onProgramAccountChange(
        program_pubKey,
        async (account, context) => {
          console.log(account)
          last_signature = await fetch_and_map_task(undefined, last_signature);
        },
        "finalized"
      );*/
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

  try {
    const txParser = new SolanaParser([
      {
        idl: getGmIDL(program_pubKey) as unknown as Idl,
        programId: program_pubKey.toString(),
      },
    ]);

    let transactionList = await solanaConnection.getSignaturesForAddress(
      program_pubKey,
      {
        limit: parseInt(process.env.LIMIT ?? "10"),
        before: before,
        until: until,
      }
    );

    stats.total = transactionList.length;

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

      if (parsed?.find((element) => element.name == "processExchange")) {
        //region MAP
        let d: any = parsed?.find(
          (element) => element.name == "processExchange"
        );

        const currency_mint = d?.accounts
          .find((account: any) => account.name == "currencyMint")
          ?.pubkey.toString();
        const asset_mint = d?.accounts
          .find((account: any) => account.name == "assetMint")
          ?.pubkey.toString();

        db_data.size = parseInt(d?.args.purchaseQuantity.toString());
        db_data.price = parseInt(d?.args.expectedPrice.toString());
        db_data.symbol =
          localStoreInstance.symbolsStore.find(
            (symbol) =>
              symbol.mint === asset_mint && symbol.pair?.mint === currency_mint
          )?.symbol_pair ?? "not-found";
        //endregion
        await collections.processExchange
          ?.insertOne(db_data)
          .then(() => stats.written_to_db++)
          .catch((err) => {
            if (err.code != 11000) throw err;
          });
        stats.exchanges++;
      } else if (parsed?.find((element) => element.name == "processCancel")) {
        //region MAP
        let d: any = parsed?.find((element) => element.name == "processCancel");
        //endregion
        await collections.cancelExchange
          ?.insertOne(db_data)
          .then(() => stats.written_to_db++)
          .catch((err) => {
            if (err.code != 11000) throw err;
          });
        stats.cancels++;
      } else if (parsed?.find((element) => element.name == "createAccount")) {
        //region MAP
        let d: any = parsed?.find(
          (element) => element.name == "processInitializeSell"
        );

        if (!d) {
          d = parsed?.find((element) => element.name == "processInitializeBuy");
        }

        const currency_mint = d.accounts
          .find((account: any) => account.name == "receiveMint")
          ?.pubkey.toString();
        const asset_mint = d.accounts
          .find((account: any) => account.name == "depositMint")
          ?.pubkey.toString();

        db_data.size = parseInt(d.args.originationQty.toString());
        db_data.price = parseInt(d.args.price.toString());
        db_data.symbol =
          localStoreInstance.symbolsStore.find(
            (symbol) =>
              symbol.mint === asset_mint && symbol.pair?.mint === currency_mint
          )?.symbol_pair ?? "not-found";
        //endregion

        await collections.createExchange
          ?.insertOne(db_data)
          .then(() => stats.written_to_db++)
          .catch((err) => {
            if (err.code != 11000) throw err;
          });
        stats.creates++;
      } else {
        await collections.unmappedExchange
          ?.insertOne(db_data)
          .then(() => stats.written_to_db++)
          .catch((err) => {
            if (err.code != 11000) throw err;
          });
        stats.unmapped++;
      }

      /*switch (parsed?.[0].name ?? "") {
        case "processExchange": {
          //region MAP
          let d = parsed?.[0] as any;

          const currency_mint = d.accounts
            .find((account: any) => account.name == "currencyMint")
            ?.pubkey.toString();
          const asset_mint = d.accounts
            .find((account: any) => account.name == "assetMint")
            ?.pubkey.toString();

          db_data.size = parseInt(d.args.purchaseQuantity.toString());
          db_data.price = parseInt(d.args.expectedPrice.toString());
          db_data.symbol =
            localStoreInstance.symbolsStore.find(
              (symbol) =>
                symbol.mint === asset_mint &&
                symbol.pair?.mint === currency_mint
            )?.symbol_pair ?? "not-found";
          //endregion
          await collections.processExchange?.insertOne(db_data).catch((err) => {
            stats.written_to_db--;
            if (err.code != 11000) throw err;
          });
          stats.exchanges++;
          break;
        }
        case "initializeOpenOrdersCounter": {
          //region MAP
          let d = parsed?.[2] as any;

          const currency_mint = d.accounts
            .find((account: any) => account.name == "receiveMint")
            ?.pubkey.toString();
          const asset_mint = d.accounts
            .find((account: any) => account.name == "receiveMint")
            ?.pubkey.toString();

          db_data.size = parseInt(d.args.originationQty.toString());
          db_data.price = parseInt(d.args.price.toString());
          db_data.symbol =
            localStoreInstance.symbolsStore.find(
              (symbol) =>
                symbol.mint === asset_mint &&
                symbol.pair?.mint === currency_mint
            )?.symbol_pair ?? "not-found";
          //endregion

          await collections.counterExchange?.insertOne(db_data).catch((err) => {
            stats.written_to_db--;
            if (err.code != 11000) throw err;
          });
          stats.counter++;
          break;
        }
        case "createAccount": {
          //region MAP
          let d = parsed?.[1] as any;

          const currency_mint = d.accounts
            .find((account: any) => account.name == "receiveMint")
            ?.pubkey.toString();
          const asset_mint = d.accounts
            .find((account: any) => account.name == "depositMint")
            ?.pubkey.toString();

          db_data.size = parseInt(d.args.originationQty.toString());
          db_data.price = parseInt(d.args.price.toString());
          db_data.symbol =
            localStoreInstance.symbolsStore.find(
              (symbol) =>
                symbol.mint === asset_mint &&
                symbol.pair?.mint === currency_mint
            )?.symbol_pair ?? "not-found";
          //endregion

          await collections.createExchange?.insertOne(db_data).catch((err) => {
            stats.written_to_db--;
            if (err.code != 11000) throw err;
          });
          stats.creates++;
          break;
        }
        case "processCancel": {
          //region MAP
          let d = parsed?.[0] as any;

          const currency_mint = d.accounts
            .find((account: any) => account.name == "receiveMint")
            ?.pubkey.toString();
          const asset_mint = d.accounts
            .find((account: any) => account.name == "depositMint")
            ?.pubkey.toString();

          db_data.symbol =
            localStoreInstance.symbolsStore.find(
              (symbol) =>
                symbol.mint === asset_mint &&
                symbol.pair?.mint === currency_mint
            )?.symbol_pair ?? "not-found";
          //endregion

          await collections.cancelExchange?.insertOne(db_data).catch((err) => {
            stats.written_to_db--;
            if (err.code != 11000) throw err;
          });
          stats.cancels++;
          break;
        }
        case "createAssociatedTokenAccount": {
          await collections.unmappedExchange
            ?.insertOne(db_data)
            .catch((err) => {
              stats.written_to_db--;
              if (err.code != 11000) throw err;
            });
          stats.direct++;
          break;
        }
        default: {
        }
      }*/
      last_signature = transaction.signature;
      last_timestamp = transaction.blockTime ?? 0;
    }

    console.log(
      `${process.env.MODE}: total=${stats.total}, exchanges=${stats.exchanges}, counter=${stats.counter}, created=${stats.creates}, canceled=${stats.cancels}, direct=${stats.direct} unmapped=${stats.unmapped} written_to_db=${stats.written_to_db}`
    );
    console.log(`${last_signature} - ${new Date(last_timestamp * 1000)}`);

    return last_signature;
  } catch (err) {
    console.error(err);
    return before ? before : until ?? "";
  }
}
