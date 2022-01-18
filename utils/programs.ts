import * as path from 'path';
import { readFileSync } from 'fs';
import * as anchor from '@project-serum/anchor';
import { ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';

export const SKINFLIP_STAKING_SEED_PREFIX = 'skinflip-staking';

export const SFX_TOKEN_ACCOUNT = 'SFXb4HZkGAZNkD5apUugy2829NQVxqDhms8RKK2JJGR';
export const SFX_TOKEN_ACCOUNT_KEYPAIR = new anchor.web3.PublicKey(SFX_TOKEN_ACCOUNT);

export const SKINFLIP_NFT_AUTHORITY = '???';
export const SKINFLIP_NFT_AUTHORITY_DEVNET = '5SMxJndLVw7utiMD6AEedHMS5urxWnjWUyWxZaW78daf'

export const SKINFLIP_STAKING_PROGRAM_ID = 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS';

export const ASSOCIATED_TOKEN_PROGRAM = new anchor.web3.PublicKey(ASSOCIATED_TOKEN_PROGRAM_ID.toString());

export const loadKeypair = (keyPath: string): anchor.web3.Keypair => {
  const _path = path.resolve(process.cwd(), keyPath);
  console.log('Load keypair: ', _path);
  const loaded = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(
      readFileSync(_path).toString()
    )),
  );
  return loaded;
}

export const getStakingMachineKeypair = async(): Promise<[anchor.web3.PublicKey, number]> => {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(SKINFLIP_STAKING_SEED_PREFIX), SFX_TOKEN_ACCOUNT_KEYPAIR.toBuffer()],
    new anchor.web3.PublicKey(SKINFLIP_STAKING_PROGRAM_ID)
  );
};

export const CLUSTERS = {
  mainnet: 'https://api.mainnet-beta.solana.com',
  devnet:' https://api.devnet.solana.com',
  testnet: 'https://api.testnet.solana.com',
} 