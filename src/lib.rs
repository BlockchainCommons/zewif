// Macros
mod blob_macro;
mod data_macro;
mod impl_attachable_macro;
mod mod_use_macro;
mod string_macro;

// Modules requiring qualified paths
pub mod parser;
pub mod sapling;

// Modules that can use unqualified paths
mod_use!(account);
mod_use!(address_id);
mod_use!(address);
mod_use!(amount);
mod_use!(anchor);
mod_use!(attachments);
mod_use!(blob);
mod_use!(block_height);
mod_use!(branch_id);
mod_use!(compact_size);
mod_use!(data);
mod_use!(derivation_info);
mod_use!(digest_utils);
mod_use!(expiry_height);
mod_use!(groth_proof);
mod_use!(identifiable);
mod_use!(incremental_merkle_tree);
mod_use!(incremental_witness);
mod_use!(int_id);
mod_use!(join_split_description);
mod_use!(mnemonic_language);
mod_use!(network);
mod_use!(orchard_action_description);
mod_use!(orchard_sent_output);
mod_use!(phgr_proof);
mod_use!(position);
mod_use!(script);
mod_use!(seconds_since_epoch);
mod_use!(seed_material);
mod_use!(shielded_address);
mod_use!(spending_key);
mod_use!(sprout_proof);
mod_use!(sprout_witness);
mod_use!(string_utils);
mod_use!(transaction);
mod_use!(transparent_address);
mod_use!(transparent_spend_authority);
mod_use!(tx_in);
mod_use!(tx_out_point);
mod_use!(tx_out);
mod_use!(txid);
mod_use!(u160_type);
mod_use!(u252_type);
mod_use!(u256_type);
mod_use!(zewif_top);
mod_use!(zewif_wallet);

use std::fmt::{self, Debug, Display, Formatter};

pub struct NoQuotesDebugOption<'a, T>(pub &'a Option<T>);

impl<T: Display> Debug for NoQuotesDebugOption<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(val) => write!(f, "Some({})", val),
            None => write!(f, "None"),
        }
    }
}

pub struct DebugOption<'a, T>(&'a Option<T>);

impl<T: Debug> Debug for DebugOption<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(val) => write!(f, "Some({:?})", val),
            None => write!(f, "None"),
        }
    }
}
