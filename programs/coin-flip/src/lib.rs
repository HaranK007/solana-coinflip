use anchor_lang::prelude::*;
use solana_program::system_instruction;

declare_id!("GZ6DpTZAYsAaEKFLLoempLuqiNZjsFTQGKeqe6YYRvCH");

#[program]
pub mod coin_flip {
    use super::*;

    pub fn initstorage(ctx : Context<StorageInit>) -> Result<()>{
        ctx.accounts.pda.result = false;
        Ok(())
    }

    pub fn main(ctx: Context<Main>) -> Result<()>{
        let to = &ctx.accounts.pda;
        let from = &ctx.accounts.signer;

        let instruction = system_instruction::transfer(from.key, &to.key(), 2000000);
        let instruction2 = system_instruction::transfer(&to.key(), &from.key(), 1000000);
        anchor_lang::solana_program::program::invoke_signed(
            &instruction,
            &[
                from.to_account_info(),
                to.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        let clk = Clock::get()?;
        let clock = clk.unix_timestamp % 2;

        if clock == 1{
            anchor_lang::solana_program::program::invoke_signed(
                &instruction2,
                &[
                    to.to_account_info(),
                    from.to_account_info(),
                ],
                &[&[b"coinflip1",&[ctx.bumps["pda"]],]],
            )?;
            ctx.accounts.stroage_pda.result = true;
        }
        else {
            ctx.accounts.stroage_pda.result = false;
        }

        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StorageInit<'info>{
    #[account(
        init,
        seeds = [b"storage"],
        bump,
        space = 8 + 16 + 1,
        payer = signer
    )]
    pub pda : Account<'info,Answer>,
    #[account(mut)]
    pub signer : Signer<'info>,
    pub system_program : Program<'info,System> 
}

#[derive(Accounts)]
pub struct Main<'info>{
  #[account(
    mut,
    seeds = [b"coinflip1"],
    bump
   )]
   /// CHECK: This is not dangerous because we don't read or write from this account
  pub pda  : AccountInfo<'info>,
  #[account(mut)]
  pub signer : Signer<'info>,
  #[account(
    mut,
    seeds = [b"storage"],
    bump
  )]
  pub stroage_pda : Account<'info,Answer>,
  pub system_program: Program<'info, System>,
}

#[account]
pub struct Answer{
    pub result : bool
}