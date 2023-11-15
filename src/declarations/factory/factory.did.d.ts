import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface CreateArg {
  'supply_cap' : [] | [bigint],
  'name' : string,
  'description' : [] | [string],
  'royalties' : [] | [number],
  'image' : [] | [Uint8Array | number[]],
  'royalties_recipient' : [] | [Account],
  'symbol' : string,
}
export interface _SERVICE {
  'create_icrc7_collection' : ActorMethod<[CreateArg], Principal>,
}
