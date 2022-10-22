export interface IDBObject {
  signature: string;
  timestamp?: number | null;
  data?: JSON[];
  size?: number;
  price?: number;
  symbol: string;
}
