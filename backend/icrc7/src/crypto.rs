use b3_utils::{
    ledger::raw_keccak256,
    memory::types::{Bound, Storable},
    vec_to_hex_string,
};
use k256::ecdsa::VerifyingKey;
use std::borrow::Cow;

#[derive(Clone, candid::CandidType, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct EcdsaSignature {
    r: [u8; 32],
    s: [u8; 32],
    v: u8,
}

impl Storable for EcdsaSignature {
    const BOUND: Bound = Bound::Bounded {
        max_size: 65,
        is_fixed_size: true,
    };

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let mut bytes = bytes.into_owned();
        let v = bytes.split_off(64);
        let s = bytes.split_off(32);
        Self {
            r: bytes.try_into().unwrap(),
            s: s.try_into().unwrap(),
            v: v[0],
        }
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        let mut bytes = Vec::with_capacity(65);
        bytes.extend_from_slice(&self.r);
        bytes.extend_from_slice(&self.s);
        bytes.push(self.v);
        bytes.into()
    }
}

impl std::string::ToString for EcdsaSignature {
    fn to_string(&self) -> String {
        let mut bytes = Vec::with_capacity(65);
        bytes.extend_from_slice(&self.r);
        bytes.extend_from_slice(&self.s);
        bytes.push(self.v);
        vec_to_hex_string(&bytes)
    }
}

impl EcdsaSignature {
    // pub fn from_sec1(bytes: &[u8]) -> Self {
    //     let mut bytes = bytes.to_vec();
    //     let s = bytes.split_off(32);
    //     Self {
    //         r: bytes.try_into().unwrap(),
    //         s: s.try_into().unwrap(),
    //         v: 0,
    //     }
    // }

    pub fn from_rsv(r: &[u8], s: &[u8], v: u8) -> Self {
        Self {
            r: r.try_into().unwrap(),
            s: s.try_into().unwrap(),
            v,
        }
    }

    pub fn from_signature_v(signature: &[u8], v: u8) -> Self {
        let mut signature = signature.to_vec();
        let s = signature.split_off(32);
        Self {
            r: signature.try_into().unwrap(),
            s: s.try_into().unwrap(),
            v,
        }
    }
}

pub fn ethereum_address_from_public_key(public_key: &[u8]) -> Result<[u8; 20], String> {
    let uncompressed_pubkey = VerifyingKey::from_sec1_bytes(public_key)
        .map_err(|_| "Pubkey parse error")?
        //.unwrap()
        .to_encoded_point(false);
    let ethereum_pubkey = &uncompressed_pubkey.as_bytes()[1..]; // trim off the first 0x04 byte

    let hashed = raw_keccak256(ethereum_pubkey).to_vec();

    Ok((&hashed[12..32]).try_into().unwrap())
}

#[test]
fn test_calculate_eth_address() {
    let public_key = b3_utils::hex_string_to_vec("04A4A4C5160DFA830E9D5FAD6DBA5248E7A9C783C30974A3382247DCE5A815DBAA4CB31812FD016561DE57A5A53EF527499031705BE824016842688B498F61FDE7").unwrap();

    let tecdsa_signer_address: [u8; 20] = ethereum_address_from_public_key(&public_key).unwrap();
    assert_eq!(
        tecdsa_signer_address,
        b3_utils::hex_string_to_vec("3b75ea5c82e96d9489ed740d455da4900f152f95")
            .unwrap()
            .as_slice()
    );
}

// In the following, we register a custom getrandom implementation because
// otherwise getrandom (which is a dependency of k256) fails to compile.
// This is necessary because getrandom by default fails to compile for the
// wasm32-unknown-unknown target (which is required for deploying a canister).
// Our custom implementation always fails, which is sufficient here because
// we only use the k256 crate for verifying secp256k1 signatures, and such
// signature verification does not require any randomness.
getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}
