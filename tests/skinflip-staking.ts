import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SkinflipStaking } from '../target/types/skinflip_staking';
import fs from 'fs';


describe('skinflip-staking', () => {

  // Configure the client to use the local cluster.
  let provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SkinflipStaking as Program<SkinflipStaking>;

  // The Account to create.

  const stakingVault = anchor.web3.Keypair.generate();

  console.log(provider.wallet.publicKey.toString());

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({
      accounts: {
        initializer: provider.wallet.publicKey,
        stakingVault: stakingVault.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [stakingVault]
    });
    
    console.log("Your transaction signature", tx);
  });
});
