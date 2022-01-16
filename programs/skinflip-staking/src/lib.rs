use anchor_lang::prelude::*;
use anchor_spl::{
    mint,
    token::{Mint, Token, TokenAccount}
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


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


#[program]
pub mod skinflip_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        msg!("Initialize staking program"); 

        let staking_vault = &mut ctx.accounts.staking_vault;
        staking_vault.staked_nfts = 0;

        Ok(())
    }

    pub fn stake(ctx: Context<StakeInstructionStruct>) -> ProgramResult {
        msg!("Stake SkinFlip NFT"); 

        let staking_vault = &mut ctx.accounts.staking_vault;
        let nft_holder = &mut ctx.accounts.nft_holder;
        let nft_token_account = &mut ctx.accounts.nft_token_account;

        msg!("Nft holder owner: {}", nft_holder.owner.to_string());
        msg!("Nft holder key: {}", nft_holder.key().to_string());

        msg!("Nft token account owner: {}", nft_token_account.owner.to_string());
        msg!("Nft token account mint: {}", nft_token_account.mint.to_string());
        msg!("Nft token account close_authority: {}", nft_token_account.close_authority.unwrap().to_string());
        msg!("Nft token account amount: {}", nft_token_account.amount);


        // staking_vault.staked_nfts = staking_vault.staked_nfts + 1;

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
    
    #[account(init, payer = initializer, space = 8 + 2)] // 8-byte-discriminator + 2-byte u16 (staked_nfts) space
    pub staking_vault: Account<'info, StakingParentStateAccount>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct StakeInstructionStruct<'info> {
    #[account(mut)]
    pub staking_vault: Account<'info, StakingParentStateAccount>,

    #[account(constraint=(nft_holder.data_is_empty() && nft_holder.lamports() > 0))]
    pub nft_holder: AccountInfo<'info>,


    #[account(mut)]
    //the token account to withdraw from
    pub nft_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = nft_holder,
        space = 8 + 8,

    )]
    pub nft_stake_data: Account<'info, StakingAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}

#[derive(Accounts)]
pub struct UnstakeInstructionStruct<'info> {
    #[account(mut)]
    pub staking_vault: Account<'info, StakingParentStateAccount>,

    #[account(
        address = constants::SFX_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap()
    )]
    pub sfx_token_account: Account<'info, Mint>,

    #[account()]
    pub nft_holder: AccountInfo<'info>,
}

#[account]
pub struct StakingParentStateAccount {
    pub staked_nfts: u16
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