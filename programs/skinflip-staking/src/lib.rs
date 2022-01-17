use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
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
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        msg!("Initialize staking program");

        let staking_machine = &mut ctx.accounts.staking_machine;
        let nft_vault = &mut ctx.accounts.nft_vault;
        staking_machine.nft_vault = *nft_vault.key;
        staking_machine.staked_nfts = 0;

        Ok(())
    }

    pub fn stake(ctx: Context<StakeInstructionStruct>, bump: u8) -> ProgramResult {
        msg!("Stake SkinFlip NFT");

        let staking_machine = &mut ctx.accounts.staking_machine;
        let nft_holder = &mut ctx.accounts.nft_holder;
        let nft_token_account = &mut ctx.accounts.nft_token_account;

        msg!("Staking machine key: {}", staking_machine.key().to_string());

        msg!("Nft holder owner: {}", nft_holder.owner.to_string());
        msg!("Nft holder key: {}", nft_holder.key.to_string());

        msg!("Nft token account key: {}", nft_token_account.key.to_string());
        msg!("Nft token account owner: {}", nft_token_account.owner.to_string());

        msg!("Bump: {}", bump);
        // msg!("Nft token account mint: {}", nft_token_account.mint.to_string());
        // msg!("Nft token account close_authority: {}", nft_token_account.close_authority.unwrap().to_string());
        // msg!("Nft token account amount: {}", nft_token_account.amount);

        staking_machine.staked_nfts = staking_machine.staked_nfts + 1;

        Ok(())
    }

    pub fn unstake(_ctx: Context<UnstakeInstructionStruct>) -> ProgramResult {
        Err(ErrorCode::StakingPeriodActive.into())
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
#[instruction(bump: u8)]
pub struct StakeInstructionStruct<'info> {
    #[account(mut)]
    pub staking_machine: ProgramAccount<'info, StakingMachine>,

    #[account(constraint=(nft_holder.data_is_empty() && nft_holder.lamports() > 0))]
    pub nft_holder: AccountInfo<'info>,

    #[account(mut)]
    //the token account to withdraw from
    pub nft_token_account: AccountInfo<'info>,

    #[account(
        init,
        seeds = [PREFIX.as_bytes(), nft_holder.key().as_ref(), nft_token_account.key().as_ref()],
        payer = nft_holder,
        bump = bump,
        space = 8 + 8 + 32
    )]
    pub nft_stake_data: ProgramAccount<'info, StakingAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}

#[derive(Accounts)]
pub struct UnstakeInstructionStruct<'info> {
    #[account(mut)]
    pub staking_vault: Account<'info, StakingMachine>,

    #[account(
        address = constants::SFX_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap()
    )]
    pub sfx_token_account: Account<'info, Mint>,

    #[account()]
    pub nft_holder: AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct StakingMachine {
    pub staked_nfts: u16,
    pub nft_vault: Pubkey,
}

#[account]
pub struct StakingAccount {
    pub staking_date: u64
}


#[error]
pub enum ErrorCode {
    #[msg("Unstaking is not yet possible.")]
    StakingPeriodActive,
}