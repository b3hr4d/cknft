import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface ApprovalArgs {
  'memo' : [] | [Uint8Array | number[]],
  'from_subaccount' : [] | [Uint8Array | number[]],
  'token_ids' : [] | [Array<bigint>],
  'created_at_time' : [] | [bigint],
  'expires_at' : [] | [bigint],
  'spender' : Account,
}
export type ApprovalError = {
    'GenericError' : { 'msg' : string, 'error_code' : bigint }
  } |
  { 'TemporaryUnavailable' : null } |
  { 'Unauthorized' : { 'tokens_ids' : Array<bigint> } } |
  { 'TooOld' : null };
export interface CollectionMetadata {
  'icrc7_supply_cap' : [] | [bigint],
  'icrc7_description' : [] | [string],
  'icrc7_total_supply' : bigint,
  'icrc7_royalty_recipient' : [] | [Account],
  'icrc7_royalties' : [] | [number],
  'icrc7_symbol' : string,
  'icrc7_image' : [] | [string],
  'icrc7_name' : string,
}
export interface InitArg {
  'supply_cap' : [] | [bigint],
  'tx_window' : number,
  'permitted_drift' : number,
  'name' : string,
  'description' : [] | [string],
  'minting_authority' : [] | [Principal],
  'royalties' : [] | [number],
  'image' : [] | [string],
  'royalties_recipient' : [] | [Account],
  'symbol' : string,
}
export type MetadataValue = { 'Int' : bigint } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string };
export interface MintArgs {
  'id' : bigint,
  'to' : Account,
  'name' : string,
  'description' : [] | [string],
  'image' : [] | [Uint8Array | number[]],
}
export type Result = { 'Ok' : bigint } |
  { 'Err' : ApprovalError };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : TransferError };
export interface Standard { 'url' : string, 'name' : string }
export interface TransferArgs {
  'to' : Account,
  'spender_subaccount' : [] | [Uint8Array | number[]],
  'from' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'is_atomic' : [] | [boolean],
  'token_ids' : Array<bigint>,
  'created_at_time' : [] | [bigint],
}
export type TransferError = {
    'GenericError' : { 'msg' : string, 'error_code' : bigint }
  } |
  { 'TemporaryUnavailable' : null } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'Unauthorized' : { 'tokens_ids' : Array<bigint> } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null };
export interface _SERVICE {
  'icrc7_approve' : ActorMethod<[ApprovalArgs], Result>,
  'icrc7_balance_of' : ActorMethod<[Account], bigint>,
  'icrc7_collection_metadata' : ActorMethod<[], CollectionMetadata>,
  'icrc7_description' : ActorMethod<[], [] | [string]>,
  'icrc7_image' : ActorMethod<[], [] | [string]>,
  'icrc7_metadata' : ActorMethod<[bigint], Array<[string, MetadataValue]>>,
  'icrc7_mint' : ActorMethod<[MintArgs], bigint>,
  'icrc7_name' : ActorMethod<[], string>,
  'icrc7_owner_of' : ActorMethod<[bigint], Account>,
  'icrc7_royalties' : ActorMethod<[], [] | [number]>,
  'icrc7_royalty_recipient' : ActorMethod<[], [] | [Account]>,
  'icrc7_supply_cap' : ActorMethod<[], [] | [bigint]>,
  'icrc7_supported_standards' : ActorMethod<[], Array<Standard>>,
  'icrc7_symbol' : ActorMethod<[], string>,
  'icrc7_tokens_of' : ActorMethod<[Account], Array<bigint>>,
  'icrc7_total_supply' : ActorMethod<[], bigint>,
  'icrc7_transfer' : ActorMethod<[TransferArgs], Result_1>,
}
