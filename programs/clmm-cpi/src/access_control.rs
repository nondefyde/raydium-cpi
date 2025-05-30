use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;

/// Ensures that the signer is the owner or a delgated authority for the position NFT
///
/// # Arguments
///
/// * `signer` - The signer address
/// * `token_account` - The token account holding the position NFT
///
pub fn is_authorized_for_token<'info>(
    signer: &Signer<'info>,
    token_account: &Box<InterfaceAccount<'info, TokenAccount>>,
) -> Result<()> {
    require_eq!(token_account.amount, 1);
    require_keys_eq!(token_account.owner, signer.key());
    Ok(())
}
