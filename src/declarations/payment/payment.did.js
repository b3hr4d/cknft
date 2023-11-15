export const idlFactory = ({ IDL }) => {
  const Result = IDL.Record({
    'r' : IDL.Text,
    's' : IDL.Text,
    'v' : IDL.Text,
    'to' : IDL.Text,
    'gas' : IDL.Text,
    'value' : IDL.Text,
    'block_hash' : IDL.Text,
    'from' : IDL.Text,
    'transaction_index' : IDL.Text,
    'hash' : IDL.Text,
    'block_number' : IDL.Text,
    'nonce' : IDL.Text,
    'input' : IDL.Text,
    'gas_price' : IDL.Text,
  });
  const Root = IDL.Record({
    'id' : IDL.Int64,
    'result' : Result,
    'jsonrpc' : IDL.Text,
  });
  const Result_1 = IDL.Variant({ 'Ok' : Root, 'Err' : IDL.Text });
  return IDL.Service({
    'balance' : IDL.Func([], [IDL.Nat], []),
    'deposit_principal' : IDL.Func([], [IDL.Text], ['query']),
    'eth_get_transaction_by_hash' : IDL.Func([IDL.Text], [Result_1], []),
    'expected_input' : IDL.Func([], [IDL.Text], ['query']),
    'verify_transaction' : IDL.Func([IDL.Text], [IDL.Nat, IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
