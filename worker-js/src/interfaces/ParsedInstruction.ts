export interface ParsedInstruction {
  parsed: Parsed;
  program: string;
  programId: string;
}

export interface Parsed {
  info: Info;
  type: string;
}

export interface Info {
  authority: string;
  destination: string;
  mint: string;
  source: string;
  tokenAmount: TokenAmount;
}

export interface TokenAmount {
  amount: string;
  decimals: number;
  uiAmount: number;
  uiAmountString: string;
}

export function toParsedInstruction(json: string): ParsedInstruction[] {
  return JSON.parse(json);
}
