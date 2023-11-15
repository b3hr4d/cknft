import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Result {
  'r' : string,
  's' : string,
  'v' : string,
  'to' : string,
  'gas' : string,
  'value' : string,
  'block_hash' : string,
  'from' : string,
  'transaction_index' : string,
  'hash' : string,
  'block_number' : string,
  'nonce' : string,
  'input' : string,
  'gas_price' : string,
}
export type Result_1 = { 'Ok' : Root } |
  { 'Err' : string };
export interface Root { 'id' : bigint, 'result' : Result, 'jsonrpc' : string }
export interface _SERVICE {
  'balance' : ActorMethod<[], bigint>,
  'deposit_principal' : ActorMethod<[], string>,
  'eth_get_transaction_by_hash' : ActorMethod<[string], Result_1>,
  'expected_input' : ActorMethod<[], string>,
  'verify_transaction' : ActorMethod<[string], [bigint, string]>,
}
