use crate::descriptor::Chain;
use crate::elements::{Address, AssetId, OutPoint, Script, Transaction, TxOutSecrets, Txid};
use crate::pset_create::validate_address;
use crate::secp256k1::PublicKey;
use crate::store::Timestamp;
use crate::{ElementsNetwork, Error};
use lwk_common::burn_script;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::str::FromStr;

/// Details of a wallet transaction output used in [`WalletTx`]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletTxOut {
    pub outpoint: OutPoint,
    pub script_pubkey: Script,
    pub height: Option<u32>,
    pub unblinded: TxOutSecrets,
    pub wildcard_index: u32,
    pub ext_int: Chain,
}

/// Value returned by [`crate::Wollet::transactions()`] containing details about a transaction
/// from the perspective of the wallet, for example the net-balance of the wallet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletTx {
    pub tx: Transaction,
    pub txid: Txid,
    pub height: Option<u32>,
    pub balance: BTreeMap<AssetId, i64>,
    pub fee: u64,
    pub type_: String,
    pub timestamp: Option<Timestamp>,
    pub inputs: Vec<Option<WalletTxOut>>,
    pub outputs: Vec<Option<WalletTxOut>>,
}

/// A recipient of a transaction.
///
/// Note that, since it doesn't use the [`Address`] but the [`Script`] and the [`PublicKey`] it's
/// network independent.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipient {
    pub satoshi: u64,
    pub script_pubkey: Script,
    pub blinding_pubkey: Option<PublicKey>,
    pub asset: AssetId,
}

impl Recipient {
    pub fn from_address(satoshi: u64, address: &Address, asset: AssetId) -> Self {
        Self {
            satoshi,
            script_pubkey: address.script_pubkey(),
            blinding_pubkey: address.blinding_pubkey,
            asset,
        }
    }
}

/// A not-yet validated recipient of a transaction.
///
/// By calling [`UnvalidatedRecipient::validate()`] can be transformed in a validated [`Recipient`]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnvalidatedRecipient {
    /// The amount to send in satoshi
    pub satoshi: u64,

    /// The address to send to
    ///
    /// If "burn", the output will be burned
    pub address: String,

    /// The asset to send
    ///
    /// If empty, the policy asset
    pub asset: String,
}

impl UnvalidatedRecipient {
    pub fn lbtc(address: String, satoshi: u64) -> Self {
        UnvalidatedRecipient {
            address,
            satoshi,
            asset: "".to_string(),
        }
    }
    pub fn burn(asset: String, satoshi: u64) -> Self {
        UnvalidatedRecipient {
            address: "burn".to_string(),
            satoshi,
            asset: asset.to_string(),
        }
    }
}

impl TryFrom<String> for UnvalidatedRecipient {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let pieces: Vec<_> = value.split(':').collect();
        if pieces.len() != 3 {
            // TODO make specific error
            return Err(Error::Generic(format!(
                r#"Invalid number of elements in string "{}", should be "address:satoshi:assetid"#,
                value,
            )));
        }
        Ok(UnvalidatedRecipient {
            satoshi: pieces[1].parse()?,
            address: pieces[0].to_string(),
            asset: pieces[2].to_string(),
        })
    }
}

impl UnvalidatedRecipient {
    fn validate_asset(&self, network: ElementsNetwork) -> Result<AssetId, Error> {
        if self.asset.is_empty() {
            Ok(network.policy_asset())
        } else {
            Ok(AssetId::from_str(&self.asset)?)
        }
    }

    fn validate_satoshi(&self) -> Result<u64, Error> {
        if self.satoshi == 0 {
            return Err(Error::InvalidAmount);
        }
        Ok(self.satoshi)
    }

    pub fn validate(&self, network: ElementsNetwork) -> Result<Recipient, Error> {
        let satoshi = self.validate_satoshi()?;
        let asset = self.validate_asset(network)?;
        if self.address == "burn" {
            let burn_script = burn_script();
            Ok(Recipient {
                satoshi,
                script_pubkey: burn_script,
                blinding_pubkey: None,
                asset,
            })
        } else {
            let address = validate_address(&self.address, network)?;
            Ok(Recipient::from_address(self.satoshi, &address, asset))
        }
    }
}

/// Value returned from [`crate::Wollet::address()`], containing the confidential [`Address`] and the
/// derivation index (the last element in the derivation path)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressResult {
    address: Address,
    index: u32,
}

impl AddressResult {
    pub fn new(address: Address, index: u32) -> Self {
        Self { address, index }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

/// Value returned from [`crate::Wollet::issuance()`] containing details about an issuance
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuanceDetails {
    pub txid: Txid,
    pub vin: u32,
    pub entropy: [u8; 32],
    pub asset: AssetId,
    pub token: AssetId,
    pub asset_amount: Option<u64>,
    pub token_amount: Option<u64>,
    pub is_reissuance: bool,
    // asset_blinder
    // token_blinder
}

pub(crate) struct DisplayTxOutSecrets<'a>(&'a TxOutSecrets);
impl<'a> std::fmt::Display for DisplayTxOutSecrets<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{},{},{},{}",
            self.0.value, self.0.asset, self.0.value_bf, self.0.asset_bf
        )
    }
}

pub(crate) struct DisplayWalletTxInputOutputs<'a>(&'a WalletTx);
impl<'a> std::fmt::Display for DisplayWalletTxInputOutputs<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut first = true;

        for input in self.0.inputs.iter() {
            if let Some(input) = input.as_ref() {
                if !first {
                    write!(f, ",")?;
                }
                write!(f, "{}", DisplayTxOutSecrets(&input.unblinded))?;
                first = false;
            }
        }

        for output in self.0.outputs.iter() {
            if let Some(output) = output.as_ref() {
                if !first {
                    write!(f, ",")?;
                }
                write!(f, "{}", DisplayTxOutSecrets(&output.unblinded))?;
                first = false;
            }
        }
        Ok(())
    }
}

impl WalletTx {
    pub fn unblinded_url(&self, explorer_url: &str) -> String {
        format!(
            "{}tx/{}#blinded={}",
            explorer_url,
            &self.txid,
            DisplayWalletTxInputOutputs(self)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_asset_roundtrip() {
        let hex = "5ac9f65c0efcc4775e0baec4ec03abdde22473cd3cf33c0419ca290e0751b225";
        let asset = AssetId::from_str(hex).unwrap();
        assert_eq!(asset.to_string(), hex);
    }

    #[test]
    fn test_wollet_tx() {
        let json_str = include_str!("../tests/data/wallet_tx.json");
        let wallet_tx: WalletTx = serde_json::from_str(json_str).unwrap();
        assert_eq!(
            wallet_tx.unblinded_url("https://blockstream.info/liquidtestnet/"),
            "https://blockstream.info/liquidtestnet/tx/c6e3187f028942973ad27224ca79baa8382e90ad686e927fc29896e8a2edf3f3#blinded=5000,38fca2d939696061a8f76d4e6b5eecd54e3b4221c846f24a6b279e79952850a5,ab9a42053c7a6ae0d55b774f3d462b1adfaa630e5d0f9b3c0f16640d55b8f6ab,6c5c2b44a0777e463d25eecb70adee84b316c2597b8a28108ffeea38c7acf45d"
        );
    }
}
