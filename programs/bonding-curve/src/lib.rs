pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HmmfMAKaX6ZiAUdadTpD8JXHz2MVTtMmGLmdXiSi5Jg8");

#[program]
pub mod bonding_curve {
    use crate::instruction::InitializeBondingCurve;

    use super::*;

    pub fn initialize_bonding_curve(ctx: Context<InitializeBondingCurve>, fee_percentage: u64, sol_amount: u16, bump: u8, min_tokens_out: u64, vault_bump: u8) -> Result<()> {
        initialize_bonding_curve::handler(ctx,fee_percentage,sol_amount,min_tokens_out,bump,vault_bump)?;
        Ok(())
    }
}
