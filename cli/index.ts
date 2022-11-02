import * as anchor from '@project-serum/anchor';
import { CLUSTERS, loadKeypair, SKINFLIP_STAKING_SEED_PREFIX } from "../utils/programs";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { SkinflipStaking } from "../target/types/skinflip_staking";
import { Command } from 'commander';
const program = new Command();
program.version('0.0.1');

// CONFIG
// todo: move to file?
const stakingMachine = loadKeypair('cli/test-staking-machine.json');
const nftVaultAddress = 'EKC62oPeo7YnD5Aso18GQCyZQ6peScW3KGUFE1tA6QQc';

programCommand('init')
  /* .requiredOption(
    '-v, --nft-vault <string>',
    'public key address for the nft vault',
  ) */
  .action(async (directory, cmd) => {
    const {
      keypair,
      env,
      // nftVault
    } = cmd.opts();

    const { stakingProgram, provider } = initAnchor(keypair, env);

    const nftVaultPubkey = new anchor.web3.PublicKey(nftVaultAddress)

    console.log('Initialize staking machine');
    const initializeTx = await stakingProgram.rpc.initialize({
      accounts: {
        initializer: provider.wallet.publicKey,
        nftVault: nftVaultPubkey,
        stakingMachine: stakingMachine.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [stakingMachine]
    });

    console.log("initializeTx signature", initializeTx);
  });

programCommand('stake')
  .requiredOption(
    '-n, --nft-token <string>',
    'nft token address',
  )
  .action(async (directory, cmd) => {
    const {
      keypair,
      env,
      nftToken
    } = cmd.opts();

    const { stakingProgram, provider } = initAnchor(keypair, env);

    const nftTokenPubkey = new anchor.web3.PublicKey(nftToken);
    const nftVaultPubkey = new anchor.web3.PublicKey(nftVaultAddress)

    const [stakingAccount, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(SKINFLIP_STAKING_SEED_PREFIX),
        provider.wallet.publicKey.toBuffer(),
        nftTokenPubkey.toBuffer()
      ],
      stakingProgram.programId
    );

    console.log('Run staking of', nftToken);

    const stakeTx = await stakingProgram.rpc.stake(
      bump,
      {
        accounts: {
          stakingMachine: stakingMachine.publicKey,
          nftStakeData: stakingAccount,
          nftHolder: provider.wallet.publicKey,
          nftToken: nftTokenPubkey,
          nftVault: nftVaultPubkey,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: []
      }
    );
  
    console.log("StakeTx signature", stakeTx);
  });

programCommand('unstake')
  .requiredOption(
    '-n, --nft-token <string>',
    'nft token address',
  )
  .action(async (directory, cmd) => {
    const {
      keypair,
      env,
      nftToken
    } = cmd.opts();

    const { stakingProgram, provider } = initAnchor(keypair, env);

    const nftTokenPubkey = new anchor.web3.PublicKey(nftToken);

    const [stakingAccount, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(SKINFLIP_STAKING_SEED_PREFIX),
        provider.wallet.publicKey.toBuffer(),
        nftTokenPubkey.toBuffer()
      ],
      stakingProgram.programId
    );

    console.log('Run unstaking of', nftToken);
    console.log('Data account', stakingAccount.toString());

    const stakeTx = await stakingProgram.rpc.unstake(
      bump,
      nftTokenPubkey,
      {
        accounts: {
          stakingMachine: stakingMachine.publicKey,
          nftStakeData: stakingAccount,
          nftHolder: provider.wallet.publicKey,
          
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: []
      }
    );
  
    console.log("StakeTx signature", stakeTx);
  });

function programCommand(name: string) {
  return program
    .command(name)
    .option(
      '-e, --env <string>',
      'Solana cluster env name',
      'devnet', //mainnet, testnet, devnet
    )
    .option(
      '-k, --keypair <path>',
      `Solana wallet location`,
      '--keypair not provided',
    )
};

function initAnchor(keypair: string, env: string) {
  process.env.ANCHOR_WALLET = keypair;
  let provider = anchor.Provider.local(CLUSTERS[env]);
  anchor.setProvider(provider);
  return {
    stakingProgram: anchor.workspace.SkinflipStaking as anchor.Program<SkinflipStaking>,
    provider
  };
}

program.parse(process.argv);
