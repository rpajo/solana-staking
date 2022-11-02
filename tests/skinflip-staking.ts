import * as anchor from '@project-serum/anchor';
import { SkinflipStaking } from '../target/types/skinflip_staking';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { ASSOCIATED_TOKEN_PROGRAM, loadKeypair, SKINFLIP_STAKING_SEED_PREFIX } from '../utils/programs';


describe('skinflip-staking', () => {

  // Configure the client to use the local cluster.
  let provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SkinflipStaking as anchor.Program<SkinflipStaking>;

  // The Account to create.

  const stakingNftVault = loadKeypair('tests/keypairs/test-vault.json');
  const stakingMachine = anchor.web3.Keypair.generate();
  const nftToken = new anchor.web3.PublicKey('D7Gd5JQ9ZwL6VihX6FnyHMwJbwcUnxUygeaE5rqbRRcb');

  console.log('Initializer: ', provider.wallet.publicKey.toString());
  console.log('Staking NFT Vault: ', stakingNftVault.publicKey.toString());
  console.log('Staking Machine: ', stakingMachine.publicKey.toString());

  it('should initialize', async () => {
    const tx = await program.rpc.initialize({
      accounts: {
        initializer: provider.wallet.publicKey,
        nftVault: stakingNftVault.publicKey,
        stakingMachine: stakingMachine.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [stakingMachine]
    });
    
    console.log("Your transaction signature", tx);
  });

  it('should accept correct accounts for staking', async () => {
    // Create a new account with seed
    const [stakingAccount, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(SKINFLIP_STAKING_SEED_PREFIX),
        provider.wallet.publicKey.toBuffer(),
        nftToken.toBuffer()
      ],
      program.programId
    );

    const [tokenAccount, _bump] = await anchor.web3.PublicKey.findProgramAddress(
      [nftToken.toBuffer()],
      program.programId
    );

    const tx = await program.rpc.stake(
      bump,
      {
        accounts: {
          stakingMachine: stakingMachine.publicKey,
          
          nftStakeData: stakingAccount,
          nftToken: tokenAccount,
          nftVault: stakingNftVault.publicKey,

          nftHolder: provider.wallet.publicKey,
          
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: []
      }
    );

    console.log("Your transaction signature", tx);
  });

  it('should accept correct accounts for unstaking', async () => {

    // Create a new account with seed
    const [stakingAccount, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(SKINFLIP_STAKING_SEED_PREFIX),
        provider.wallet.publicKey.toBuffer(),
        nftToken.toBuffer()
      ],
      program.programId
    );

    const [tokenAccount, _bump] = await anchor.web3.PublicKey.findProgramAddress(
      [nftToken.toBuffer()],
      program.programId
    );

    const tx = await program.rpc.unstake(
      bump,
      nftToken,
      {
        accounts: {
          stakingMachine: stakingMachine.publicKey,
          nftHolder: provider.wallet.publicKey,
          nftStakeData: stakingAccount,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: []
      }
    );

    console.log("Your transaction signature", tx);
  });
});
