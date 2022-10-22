import { ICurrency } from "./Currency";

export interface ISymbol {
  symbol: string;
  mint: string;
  pair?: ICurrency;
  symbol_pair?: string;
}
