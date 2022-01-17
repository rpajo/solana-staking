import { readFileSync } from 'fs';
import * as anchor from '@project-serum/anchor';

export const SKINFLIP_STAKING_SEED_PREFIX = 'skinflip-staking';

export const SFX_TOKEN_ACCOUNT = 'SFXb4HZkGAZNkD5apUugy2829NQVxqDhms8RKK2JJGR';
export const SFX_TOKEN_ACCOUNT_KEYPAIR = new anchor.web3.PublicKey(SFX_TOKEN_ACCOUNT);

export const SKINFLIP_NFT_AUTHORITY = '???';
export const SKINFLIP_NFT_AUTHORITY_DEVNET = '5SMxJndLVw7utiMD6AEedHMS5urxWnjWUyWxZaW78daf'

export const SKINFLIP_STAKING_PROGRAM_ID = 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS';

export const loadKeypair = (keyName: string): anchor.web3.Keypair => {
  const loaded = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(readFileSync(`tests/keypairs/${keyName}`).toString())),
  );
  return loaded;
}

export const getStakingMachineKeypair = async(): Promise<[anchor.web3.PublicKey, number]> => {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(SKINFLIP_STAKING_SEED_PREFIX), SFX_TOKEN_ACCOUNT_KEYPAIR.toBuffer()],
    new anchor.web3.PublicKey(SKINFLIP_STAKING_PROGRAM_ID)
  );
}