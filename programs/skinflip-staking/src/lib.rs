use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token},
    // associated_token::AssociatedToken,
    associated_token::AssociatedToken
    // mint,
};

declare_id!("99pkxD7en56LmNoTQcjNdy4vreeCJBjeaoDvWJe2mcjc");


#[cfg(not(feature = "local-testing"))]
pub mod constants {
    pub const SFX_TOKEN_MINT_PUBKEY: &str = "test6YDiRhymkkK2Big9sn6xzfXNT454eREb3saJ4TB";
    // pub const SFX_TOKEN_MINT_PUBKEY: &str = "SFXb4HZkGAZNkD5apUugy2829NQVxqDhms8RKK2JJGR";
    pub const SKINFLIP_NFT_AUTHORITY: &str = "5SMxJndLVw7utiMD6AEedHMS5urxWnjWUyWxZaW78daf";
}

#[cfg(feature = "local-testing")]
pub mod constants {
    pub const SFX_TOKEN_MINT_PUBKEY: &str = "test6YDiRhymkkK2Big9sn6xzfXNT454eREb3saJ4TB";
    pub const SKINFLIP_NFT_AUTHORITY: &str = "5SMxJndLVw7utiMD6AEedHMS5urxWnjWUyWxZaW78daf";
}

const PREFIX: &str = "skinflip-staking";

#[program]
pub mod skinflip_staking {
    use anchor_spl::token;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        msg!("Initialize staking program");

        let staking_machine = &mut ctx.accounts.staking_machine;
        let nft_vault = &mut ctx.accounts.nft_vault;
        staking_machine.nft_vault = *nft_vault.key;
        staking_machine.staked_nfts = 0;

        Ok(())
    }

    pub fn stake(ctx: Context<StakeInstructionStruct>, bump: u8,  nft_token: Pubkey) -> ProgramResult {
        msg!("Stake SkinFlip NFT");

        msg!("NFT: {}", nft_token.key().to_string());

        let staking_machine = &mut ctx.accounts.staking_machine;
        let nft_holder = &mut ctx.accounts.nft_holder;
        let nft_stake_data = &mut ctx.accounts.nft_stake_data;

        // let nft_token = &mut ctx.accounts.nft_token;
        // let nft_token_account_recipient = &mut ctx.accounts.nft_token_account_recipient;
        let clock = &ctx.accounts.clock;


        msg!("Staking machine key: {}", staking_machine.key().to_string());

        msg!("Nft holder owner: {}", nft_holder.owner.to_string());
        msg!("Nft holder key: {}", nft_holder.key.to_string());

        msg!("nft_stake_data key: {}", nft_stake_data.key().to_string());

        // msg!("Nft token account mint: {}", nft_token_account_recipient.mint.to_string());
        // msg!("Nft token account owner: {}", nft_token_account_recipient.owner.to_string());

        msg!("Bump: {}", bump);
        // msg!("Nft token account mint: {}", nft_token_account.mint.to_string());
        // msg!("Nft token account close_authority: {}", nft_token_account.close_authority.unwrap().to_string());
        // msg!("Nft token account amount: {}", nft_token_account.amount);

        staking_machine.staked_nfts = staking_machine.staked_nfts + 1;
        nft_stake_data.staking_date = clock.unix_timestamp;
        
        msg!("Staking time: {}", nft_stake_data.staking_date);
        

        Ok(())
    }

    pub fn unstake(ctx: Context<UnstakeInstructionStruct>, bump: u8,  nft_token: Pubkey) -> ProgramResult {
        let staking_machine = &mut ctx.accounts.staking_machine;
        let nft_holder = &mut ctx.accounts.nft_holder;
        let nft_stake_data = &mut ctx.accounts.nft_stake_data;
        let clock = &ctx.accounts.clock;

        msg!("Unstake SkinFlip NFT");
        msg!("NFT: {}", nft_token.key().to_string());
        msg!("NFT holder: {}", nft_holder.key().to_string());

        
        let time_diff = clock.unix_timestamp - nft_stake_data.staking_date;
        msg!("Staked nfts {}", staking_machine.staked_nfts);
        msg!("Staked at {}, time diff: {}", nft_stake_data.staking_date, time_diff);

        // Err(ErrorCode::StakingPeriodActive.into())
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    ///pays rent on the initializing accounts
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        init,
        payer = initializer,
        space = 8 + 1 + 2 + 32 + 8
    )]
    pub staking_machine: ProgramAccount<'info, StakingMachine>,

    #[account()]
    nft_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(bump: u8, nft_token: Pubkey)]
pub struct StakeInstructionStruct<'info> {
    #[account(mut)]
    pub staking_machine: ProgramAccount<'info, StakingMachine>,

    #[account(constraint=(nft_holder.data_is_empty() && nft_holder.lamports() > 0))]
    pub nft_holder: AccountInfo<'info>,
/*
    #[account(mut)]
    pub nft_token: AccountInfo<'info>,

         
    #[account(
        init,
        payer = nft_holder,
        associated_token::mint = nft_token,
        associated_token::authority = staking_machine,
    )]
    //the token account to withdraw TO
    pub nft_token_account_recipient: Account<'info, TokenAccount>,
    */

    #[account(
        init_if_needed,
        seeds = [PREFIX.as_bytes(), nft_holder.key().as_ref(), nft_token.as_ref()],
        payer = nft_holder,
        bump = bump,
        space = 8 + 8
    )]
    pub nft_stake_data: ProgramAccount<'info, StakingAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(bump: u8, nft_token: Pubkey)]
pub struct UnstakeInstructionStruct<'info> {
    #[account(mut)]
    pub staking_machine: Account<'info, StakingMachine>,

    #[account()]
    pub nft_holder: AccountInfo<'info>,

    #[account(
        seeds = [PREFIX.as_bytes(), nft_holder.key().as_ref(), nft_token.as_ref()],
        bump = bump
    )]
    pub nft_stake_data: ProgramAccount<'info, StakingAccount>,

    pub clock: Sysvar<'info, Clock>,
}

#[account]
#[derive(Default)]
pub struct StakingMachine {
    pub staked_nfts: u16,
    pub nft_vault: Pubkey,
}

#[account]
pub struct StakingAccount {
    pub staking_date: i64
}


#[error]
pub enum ErrorCode {
    #[msg("Unstaking is not yet possible.")]
    StakingPeriodActive,
}