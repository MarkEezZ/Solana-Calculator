use anchor_lang::prelude::*;

declare_id!("ABhS9S2NKsGKXqmLEUpWD6xW4dUm2Ue4v5C3cgdsgYs8");

#[program]
pub mod solana_calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = message;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Calculator {
    greeting: String,
    result: i64,
    remainder: i64,
}
