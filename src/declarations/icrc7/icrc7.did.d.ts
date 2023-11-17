import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface ApprovalArgs {
  'memo' : [] | [Uint8Array | number[]],
  'from_subaccount' : [] | [Uint8Array | number[]],
  'token_ids' : [] | [Array<bigint>],
  'created_at_time' : [] | [bigint],
  'expires_at' : [] | [bigint],
  'spender' : ICRCAccount,
}
export type ApprovalError = {
    'GenericError' : { 'msg' : string, 'error_code' : bigint }
  } |
  { 'TemporaryUnavailable' : null } |
  { 'Unauthorized' : { 'tokens_ids' : Array<bigint> } } |
  { 'TooOld' : null };
export interface CollectionConfig {
  'supply_cap' : [] | [bigint],
  'tx_window' : bigint,
  'permitted_drift' : bigint,
  'name' : string,
  'description' : [] | [string],
  'minting_authority' : Principal,
  'royalties' : [] | [number],
  'royalty_recipient' : [] | [ICRCAccount],
  'image' : [] | [string],
  'symbol' : string,
}
export interface CollectionMetadata {
  'icrc7_supply_cap' : [] | [bigint],
  'icrc7_description' : [] | [string],
  'icrc7_total_supply' : bigint,
  'icrc7_royalty_recipient' : [] | [ICRCAccount],
  'icrc7_royalties' : [] | [number],
  'icrc7_symbol' : string,
  'icrc7_image' : [] | [string],
  'icrc7_name' : string,
}
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
  'status_code' : number,
}
export type ICRC1MetadataValue = { 'Int' : bigint } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string };
export interface ICRCAccount {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface MintArgs {
  'id' : bigint,
  'to' : ICRCAccount,
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
  'to' : ICRCAccount,
  'spender_subaccount' : [] | [Uint8Array | number[]],
  'from' : ICRCAccount,
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
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'icrc7_approve' : ActorMethod<[ApprovalArgs], Result>,
  'icrc7_balance_of' : ActorMethod<[ICRCAccount], bigint>,
  'icrc7_collection_metadata' : ActorMethod<[], CollectionMetadata>,
  'icrc7_description' : ActorMethod<[], [] | [string]>,
  'icrc7_image' : ActorMethod<[], [] | [string]>,
  'icrc7_metadata' : ActorMethod<[bigint], Array<[string, ICRC1MetadataValue]>>,
  'icrc7_mint' : ActorMethod<[MintArgs], bigint>,
  'icrc7_name' : ActorMethod<[], string>,
  'icrc7_owner_of' : ActorMethod<[bigint], ICRCAccount>,
  'icrc7_royalties' : ActorMethod<[], [] | [number]>,
  'icrc7_royalty_recipient' : ActorMethod<[], [] | [ICRCAccount]>,
  'icrc7_supply_cap' : ActorMethod<[], [] | [bigint]>,
  'icrc7_supported_standards' : ActorMethod<[], Array<Standard>>,
  'icrc7_symbol' : ActorMethod<[], string>,
  'icrc7_tokens_of' : ActorMethod<[ICRCAccount], Array<bigint>>,
  'icrc7_total_supply' : ActorMethod<[], bigint>,
  'icrc7_transfer' : ActorMethod<[TransferArgs], Result_1>,
}
