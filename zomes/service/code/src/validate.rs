use ed25519_dalek; // {Signature, PublicKey}
use failure::Error;
use hcid::HcidEncoding;
use lazy_static::lazy_static;
use serde::{Serialize, Serializer, Deserialize, Deserializer, de, };
use std::{fmt, str::FromStr, convert::{From, TryFrom, TryInto}, };

use hdk::{
    holochain_persistence_api::{
        hash::HashString,
        cas::content::Address,
    },
    holochain_json_api::{
        json::JsonString, error::JsonError,
    },
    holochain_wasm_utils::api_serialization::{
        get_entry::{GetEntryResult, GetEntryOptions, GetEntryResultType},
    },
    holochain_core_types::{
        entry::{AppEntryValue, Entry},
        error::HolochainError,
        signature::Provenance,
        time::Iso8601,
    },
};

/// Validates and round-trips an Agent from a ed25519 PublicKey, and validates AgentSignatures
#[derive(Debug, Clone, PartialEq)]
pub struct Agent(ed25519_dalek::PublicKey);

lazy_static! {
    pub static ref AGENT_CODEC: hcid::HcidEncoding =
        HcidEncoding::with_kind("hcs0").expect("Couldn't init Agent hcid 'hcs0' codec.");
}

impl Agent {
    #[inline]
    pub fn to_bytes(&self) -> [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] {
        self.0.to_bytes()
    }

    #[inline]
    pub fn from_bytes(
        bytes: &[u8]
    ) -> Result<Agent, Error> {
        Ok(Agent(ed25519_dalek::PublicKey::from_bytes( bytes )?))
    }

    pub fn verify(
        &self,
        message: &[u8],
        signature: &AgentSignature,
    ) -> bool {
        hdk::debug(format!("Verify signature {}", &signature)).ok();
        self.0.verify(message, &signature.0).is_ok()
    }
}

// An Agent is indeed an ed25519 Public key, derivable from either a Public/Secret Key, or
// the bytes of the PublicKey.
impl From<ed25519_dalek::PublicKey> for Agent {
    fn from(public_key: ed25519_dalek::PublicKey) -> Agent {
        Agent(public_key)
    }
}

impl From<&ed25519_dalek::SecretKey> for Agent {
    fn from(secret_key: &ed25519_dalek::SecretKey) -> Agent {
        Agent(secret_key.into())
    }
}

impl TryFrom<&[u8]> for Agent {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Agent::from_bytes(bytes)
    }
}

impl TryFrom<String> for Agent {
    type Error = Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Agent::from_str(&s)
    }
}

impl TryFrom<&str> for Agent {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Agent::from_str(s)
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", AGENT_CODEC.encode(&self.to_bytes())
               .map_err(|_e| std::fmt::Error)?)
    }
}

impl FromStr for Agent {
    type Err = Error;
    fn from_str(agent_id: &str) -> Result<Self, Self::Err> {
        Ok(Agent(
            ed25519_dalek::PublicKey::from_bytes(
                &AGENT_CODEC.decode(agent_id)?
            )?
        ))
    }
}

impl Serialize for Agent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'d> Deserialize<'d> for Agent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        let agent_id = String::deserialize(deserializer)?; // HcScj...
        Ok(
            Agent::from_str(&agent_id)
                .map_err(de::Error::custom)?
        )
    }
}


/// Takes a holochain Signature(String) in base-64 form, and converts to/from an ed25519 Signature
#[derive(Debug, Clone, PartialEq)]
pub struct AgentSignature(ed25519_dalek::Signature);

impl AgentSignature {
    #[inline]
    pub fn to_bytes(&self) -> [u8; ed25519_dalek::SIGNATURE_LENGTH] {
        self.0.to_bytes()
    }

    #[inline]
    pub fn from_bytes(
        bytes: &[u8]
    ) -> Result<AgentSignature, Error> {
        Ok(AgentSignature(ed25519_dalek::Signature::from_bytes( bytes )?))
    }
}

impl TryFrom<&[u8]> for AgentSignature {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        AgentSignature::from_bytes(bytes)
    }
}

impl fmt::Display for AgentSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        base64::encode_config_buf(self.to_bytes().as_ref(), base64::STANDARD, &mut buf);
        write!(f, "{}", buf)
    }
}

impl FromStr for AgentSignature {
    type Err = Error;
    fn from_str(agent_sig: &str) -> Result<Self, Self::Err> {
        let sig_bytes = base64::decode(&agent_sig)?;
        let sig = ed25519_dalek::Signature::from_bytes(&sig_bytes)?;
        Ok(AgentSignature(sig))
    }
}

impl Serialize for AgentSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'d> Deserialize<'d> for AgentSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        let agent_sig = String::deserialize(deserializer)?;
        AgentSignature::from_str(&agent_sig)
            .map_err(de::Error::custom)
    }
}

/// The Client Digest is de/serialized as a base-58 formatted multicodec-prefixed SHA2-256
/// hash, ie. "Qm...".  This is the "native" form of a HashString, but it is not checked.  Check it
/// here, and make it available for signature verification.
/// 
/// TODO: Much of this should be upstreamed into holochain_persistence; the validation of hashes by
/// both size and encoding is very common, and Holo / Holochain contains various hashes with various
/// size and semantic differences, which we should be validating.
#[derive(Debug, Clone, PartialEq)]
pub struct Digest([u8; 32]);

/// Convert a SHA2-256 digest in a fixed-size array directly
impl From<[u8; 32]> for Digest {
    fn from(a: [u8; 32]) -> Digest {
        Digest(a)
    }
}

impl Digest {
    pub fn to_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Expects an unencoded SHA2-256 digest; verifies the length of the provided reference
    pub fn from_bytes(
        bytes: &[u8]
    ) -> Result<Digest, Error> {
        if bytes.len() == 32 {
            let mut bits: [u8; 32] = [0u8; 32];
            bits.copy_from_slice(&bytes[..32]);
            Ok(Digest(bits))
        } else {
            Err(format_err!(
                "Digest requires a SHA2-256 digest",
            ))
        }
    }

    pub fn to_vec_encoded(&self) -> Vec<u8> {
        let mut v = vec![0x12_u8, 32];
        v.extend( self.to_bytes() );
        v
    }
}

impl FromStr for Digest {
    type Err = Error;
    fn from_str(request_hash: &str) -> Result<Self, Self::Err> {
        // A base-58 32-byte SHA2-256 "Qm..." 
        let maybe_hash: Result<Vec<u8>, _> = HashString::from(request_hash).try_into();
        if let Ok(hash_bytes) = maybe_hash { // validates base-58
            if hash_bytes.len() >= 2 // SHA2-256 multicodec prefix
                && hash_bytes[0] == 0x12_u8
                && hash_bytes[1] == 32 { // strip prefix, validate size (again)
                if let Ok(hash) = Digest::from_bytes(&hash_bytes[2..]) {
                    return Ok(hash)
                }
            }
        }
        Err(format_err!( // All failures provide this more informative error message
            "Digest requires a multicodec base-58 SHA2-256 digest, found {}",
            request_hash
        ))
    }
}

impl TryFrom<String> for Digest {
    type Error = Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Digest::from_str(&s)
    }
}

impl TryFrom<&str> for Digest {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Digest::from_str(s)
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", HashString::from(self.to_vec_encoded()))
    }
}

impl Serialize for Digest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(
            &String::from(HashString::from(self.to_vec_encoded()))
        )
    }
}

/// Verifies base-58 encoding, multicodec prefix and size
impl<'d> Deserialize<'d> for Digest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        let hash_str = String::deserialize(deserializer)?;
        Digest::from_str(&hash_str)
            .map_err(de::Error::custom)
    }
}


/// Partial ChainHeader details for an App Entry entry of type R
#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct CommitMeta {
    pub address: Address,
    pub provenance: Provenance,
    pub timestamp: Iso8601,
}

/// Get and decode an App Entry type, and return it with some commit metadata
pub fn get_meta_and_entry_as_type<R: TryFrom<AppEntryValue>>(
    address: Address
) -> Result<(CommitMeta,R), HolochainError> {
    let get_result = hdk::get_entry_result(
        &address,
        GetEntryOptions {
            entry: true,
            headers: true,
            ..Default::default()
        }
    )?;
    if let GetEntryResult { result: GetEntryResultType::Single(item)} = get_result {
        if let Some(Entry::App(_, entry_value)) = item.entry { // None, if nothing found
            if let Ok(entry) = R::try_from(entry_value.to_owned()) {
                if let [header] = item.headers.as_slice() {
                    if let [provenance] = header.provenances().as_slice() {
                        let commit_meta = CommitMeta {
                            address,
                            provenance: provenance.to_owned(),
                            timestamp: header.timestamp().to_owned(),
                        };
                        return Ok((commit_meta,entry))
                    }
                }
            }
            return Err(format!(
                "No entry of designated type at address {}: {:?}",
                address, entry_value
            ).into())
        }
    }
    Err(format!(
        "No entry at address {}", address
    ).into())
}
