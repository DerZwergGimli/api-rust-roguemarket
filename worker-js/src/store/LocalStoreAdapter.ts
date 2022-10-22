import { ISymbol } from "../interfaces/Symbol";
import axios, { AxiosResponse } from "axios";
import { ICurrency } from "../interfaces/Currency";
import { IStarAtlasAPI } from "../interfaces/StarAtlasAPI";

const STARATLASAPIURL = "https://galaxy.staratlas.com/nfts";

class LocalStoreAdapter {
  public initialized = false;
  public symbolsStore: ISymbol[] = [];
  public currencyStore: ICurrency[] = [
    {
      symbol: "USDC",
      mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    },
    {
      symbol: "ATLAS",
      mint: "ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx",
    },
  ];

  public async init() {
    this.symbolsStore = await this.initSymbols();
    this.initialized = true;
    console.log("LocalStoreAdapter initialized");
  }

  private async initSymbols(): Promise<ISymbol[]> {
    let staratlas_list: IStarAtlasAPI[] = [];

    await axios
      .get(STARATLASAPIURL)
      .then((response: AxiosResponse<IStarAtlasAPI[]>) => {
        staratlas_list = response.data;
      });

    let symbols: ISymbol[] = [];
    staratlas_list.forEach((asset) => {
      this.currencyStore.forEach((code) => {
        symbols.push({
          symbol: asset.symbol,
          mint: asset.mint,
          pair: code,
          symbol_pair: asset.symbol + code.symbol,
        });
      });
    });

    return symbols;
  }
}

const localStoreInstance = new LocalStoreAdapter();
export { localStoreInstance };
