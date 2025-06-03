use anchor_lang::prelude::*;

declare_id!("DxA4FB2WQG6m2BJYcK82eE2bgxv3x2hHZCfxxhqTvCv3");

#[program]
pub mod solana_token_faucet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
