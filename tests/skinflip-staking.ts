import * as anchor from '@project-serum/anchor';
import { SkinflipStaking } from '../target/types/skinflip_staking';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { loadKeypair, SKINFLIP_STAKING_SEED_PREFIX } from '../utils/programs';


describe('skinflip-staking', () => {

  // Configure the client to use the local cluster.
  let provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SkinflipStaking as anchor.Program<SkinflipStaking>;

  // The Account to create.

  const stakingNftVault = loadKeypair('test.json');
  const stakingMachine = anchor.web3.Keypair.generate();

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
    const tokenAccount = anchor.web3.Keypair.generate();

    // Create a new account with seed
    const [stakingAccount, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(SKINFLIP_STAKING_SEED_PREFIX),
        provider.wallet.publicKey.toBuffer(),
        tokenAccount.publicKey.toBuffer()
      ],
      program.programId
    );

    const tx = await program.rpc.stake(bump, {
      accounts: {
        stakingMachine: stakingMachine.publicKey,
        nftHolder: provider.wallet.publicKey,
        nftStakeData: stakingAccount,
        nftTokenAccount: tokenAccount.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: []
    });

    console.log("Your transaction signature", tx);
  });
});
