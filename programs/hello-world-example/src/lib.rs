use anchor_lang::prelude::*;

declare_id!("88EPmpeFwFPtrrWeJjzFvLTVG6NVPE68yQCCx8ApCq8D");

#[program]
pub mod hello_world_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
