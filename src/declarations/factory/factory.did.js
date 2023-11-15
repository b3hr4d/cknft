export const idlFactory = ({ IDL }) => {
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const CreateArg = IDL.Record({
    'supply_cap' : IDL.Opt(IDL.Nat),
    'name' : IDL.Text,
    'description' : IDL.Opt(IDL.Text),
    'royalties' : IDL.Opt(IDL.Nat16),
    'image' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'royalties_recipient' : IDL.Opt(Account),
    'symbol' : IDL.Text,
  });
  return IDL.Service({
    'create_icrc7_collection' : IDL.Func([CreateArg], [IDL.Principal], []),
  });
};
export const init = ({ IDL }) => { return []; };
