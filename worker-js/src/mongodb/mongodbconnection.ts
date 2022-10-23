import * as mongoDB from "mongodb";
import * as dotenv from "dotenv";

export const collections: {
  processExchange?: mongoDB.Collection;
  counterExchange?: mongoDB.Collection;
  createExchange?: mongoDB.Collection;
  cancelExchange?: mongoDB.Collection;
  unmappedExchange?: mongoDB.Collection;
} = {};

export async function connectToDatabase() {
  dotenv.config();

  const client: mongoDB.MongoClient = new mongoDB.MongoClient(
    process.env.DB_CONN_STRING ?? ""
  );

  await client.connect();

  const db: mongoDB.Db = client.db(process.env.DB_NAME);

  const processExchange: mongoDB.Collection = db.collection(
    process.env.EXCHANGE_COLLECTION ?? "market_interactions"
  );
  const counterExchange: mongoDB.Collection = db.collection(
    process.env.COUNTER_COLLECTION ?? "market_interactions"
  );
  const createExchange: mongoDB.Collection = db.collection(
    process.env.CREATE_COLLECTION ?? "market_interactions"
  );
  const cancelExchange: mongoDB.Collection = db.collection(
    process.env.CANCEL_COLLECTION ?? "market_interactions"
  );
  const unmappedExchange: mongoDB.Collection =
    db.collection("unmappedExchange");

  await processExchange.createIndex({ signature: 1 }, { unique: true });
  await counterExchange.createIndex({ signature: 1 }, { unique: true });
  await createExchange.createIndex({ signature: 1 }, { unique: true });
  await cancelExchange.createIndex({ signature: 1 }, { unique: true });
  await unmappedExchange.createIndex({ signature: 1 }, { unique: true });

  await processExchange.createIndex({ symbol: 1 }, { unique: false });
  await counterExchange.createIndex({ symbol: 1 }, { unique: false });
  await createExchange.createIndex({ symbol: 1 }, { unique: false });
  await cancelExchange.createIndex({ symbol: 1 }, { unique: false });
  await unmappedExchange.createIndex({ symbol: 1 }, { unique: false });

  collections.processExchange = processExchange;
  collections.counterExchange = counterExchange;
  collections.createExchange = createExchange;
  collections.cancelExchange = cancelExchange;
  collections.unmappedExchange = unmappedExchange;

  console.log(`Successfully connected to database: ${db.databaseName}!`);
}
