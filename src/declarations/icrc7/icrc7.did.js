export const idlFactory = ({ IDL }) => {
  const ICRCAccount = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const CollectionConfig = IDL.Record({
    'supply_cap' : IDL.Opt(IDL.Nat),
    'tx_window' : IDL.Nat64,
    'permitted_drift' : IDL.Nat64,
    'name' : IDL.Text,
    'description' : IDL.Opt(IDL.Text),
    'minting_authority' : IDL.Principal,
    'royalties' : IDL.Opt(IDL.Nat16),
    'royalty_recipient' : IDL.Opt(ICRCAccount),
    'image' : IDL.Opt(IDL.Text),
    'symbol' : IDL.Text,
  });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'status_code' : IDL.Nat16,
  });
  const ApprovalArgs = IDL.Record({
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'from_subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'token_ids' : IDL.Opt(IDL.Vec(IDL.Nat)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'expires_at' : IDL.Opt(IDL.Nat64),
    'spender' : ICRCAccount,
  });
  const ApprovalError = IDL.Variant({
    'GenericError' : IDL.Record({ 'msg' : IDL.Text, 'error_code' : IDL.Nat }),
    'TemporaryUnavailable' : IDL.Null,
    'Unauthorized' : IDL.Record({ 'tokens_ids' : IDL.Vec(IDL.Nat) }),
    'TooOld' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : ApprovalError });
  const CollectionMetadata = IDL.Record({
    'icrc7_supply_cap' : IDL.Opt(IDL.Nat),
    'icrc7_description' : IDL.Opt(IDL.Text),
    'icrc7_total_supply' : IDL.Nat,
    'icrc7_royalty_recipient' : IDL.Opt(ICRCAccount),
    'icrc7_royalties' : IDL.Opt(IDL.Nat16),
    'icrc7_symbol' : IDL.Text,
    'icrc7_image' : IDL.Opt(IDL.Text),
    'icrc7_name' : IDL.Text,
  });
  const ICRC1MetadataValue = IDL.Variant({
    'Int' : IDL.Int,
    'Nat' : IDL.Nat,
    'Blob' : IDL.Vec(IDL.Nat8),
    'Text' : IDL.Text,
  });
  const MintArgs = IDL.Record({
    'id' : IDL.Nat,
    'to' : ICRCAccount,
    'name' : IDL.Text,
    'description' : IDL.Opt(IDL.Text),
    'image' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const Standard = IDL.Record({ 'url' : IDL.Text, 'name' : IDL.Text });
  const TransferArgs = IDL.Record({
    'to' : ICRCAccount,
    'spender_subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'from' : ICRCAccount,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'is_atomic' : IDL.Opt(IDL.Bool),
    'token_ids' : IDL.Vec(IDL.Nat),
    'created_at_time' : IDL.Opt(IDL.Nat64),
  });
  const TransferError = IDL.Variant({
    'GenericError' : IDL.Record({ 'msg' : IDL.Text, 'error_code' : IDL.Nat }),
    'TemporaryUnavailable' : IDL.Null,
    'Duplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat }),
    'Unauthorized' : IDL.Record({ 'tokens_ids' : IDL.Vec(IDL.Nat) }),
    'CreatedInFuture' : IDL.Record({ 'ledger_time' : IDL.Nat64 }),
    'TooOld' : IDL.Null,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : TransferError });
  return IDL.Service({
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'icrc7_approve' : IDL.Func([ApprovalArgs], [Result], []),
    'icrc7_balance_of' : IDL.Func([ICRCAccount], [IDL.Nat], ['query']),
    'icrc7_collection_metadata' : IDL.Func([], [CollectionMetadata], ['query']),
    'icrc7_description' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'icrc7_image' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'icrc7_metadata' : IDL.Func(
        [IDL.Nat],
        [IDL.Vec(IDL.Tuple(IDL.Text, ICRC1MetadataValue))],
        ['query'],
      ),
    'icrc7_mint' : IDL.Func([MintArgs], [IDL.Nat], []),
    'icrc7_name' : IDL.Func([], [IDL.Text], ['query']),
    'icrc7_owner_of' : IDL.Func([IDL.Nat], [ICRCAccount], ['query']),
    'icrc7_royalties' : IDL.Func([], [IDL.Opt(IDL.Nat16)], ['query']),
    'icrc7_royalty_recipient' : IDL.Func([], [IDL.Opt(ICRCAccount)], ['query']),
    'icrc7_supply_cap' : IDL.Func([], [IDL.Opt(IDL.Nat)], ['query']),
    'icrc7_supported_standards' : IDL.Func([], [IDL.Vec(Standard)], ['query']),
    'icrc7_symbol' : IDL.Func([], [IDL.Text], ['query']),
    'icrc7_tokens_of' : IDL.Func([ICRCAccount], [IDL.Vec(IDL.Nat)], ['query']),
    'icrc7_total_supply' : IDL.Func([], [IDL.Nat], ['query']),
    'icrc7_transfer' : IDL.Func([TransferArgs], [Result_1], []),
  });
};
export const init = ({ IDL }) => {
  const ICRCAccount = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const CollectionConfig = IDL.Record({
    'supply_cap' : IDL.Opt(IDL.Nat),
    'tx_window' : IDL.Nat64,
    'permitted_drift' : IDL.Nat64,
    'name' : IDL.Text,
    'description' : IDL.Opt(IDL.Text),
    'minting_authority' : IDL.Principal,
    'royalties' : IDL.Opt(IDL.Nat16),
    'royalty_recipient' : IDL.Opt(ICRCAccount),
    'image' : IDL.Opt(IDL.Text),
    'symbol' : IDL.Text,
  });
  return [CollectionConfig];
};
