import * as anchor from '@coral-xyz/anchor';
import { expect } from 'chai';
import { createMint, TOKEN_PROGRAM_ID } from '@solana/spl-token';

describe('mostro - create_artist', () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const programAny = (anchor.workspace.MostroProgram as any) as any;

  it('creates artist account and vault', async () => {
    const wallet = provider.wallet as anchor.Wallet;

    // Ensure global_config exists: if not, create it so admin check passes
    const [globalConfigPda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from('global_config')],
      programAny.programId
    );
    const globalRaw = await provider.connection.getAccountInfo(globalConfigPda);
    if (!globalRaw) {
      // create config with default values
      await programAny.methods
        .createConfig(30, 70, wallet.publicKey)
        .accounts({
          admin: wallet.publicKey,
          config: globalConfigPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
    }

    // create a new mint for pump token
    const mint = await createMint(
      provider.connection,
      (wallet as any).payer as any,
      wallet.publicKey,
      null,
      0
    );

    // artist wallet keypair (we'll use a generated Keypair)
    const artistKeypair = anchor.web3.Keypair.generate();

    // derive PDAs
    const [artistAccountPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('artist'), artistKeypair.publicKey.toBuffer()],
      programAny.programId
    );

    const [artistVaultAuthPda, artistVaultAuthBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('artist_vault'), artistKeypair.publicKey.toBuffer()],
      programAny.programId
    );

    const artistVaultPda = artistVaultAuthPda; // same PDA for vault and authority seeds

    const metadataUri = 'https://example.com/metadata.json';
    const percentageArtist = 15;
    const percentageMostro = 85;

    // fund the artist keypair so that it can be used in CPI if needed
    const sig = await provider.connection.requestAirdrop(
      artistKeypair.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);

    // call createArtist
    let created = false;
    try {
      const tx = await programAny.methods
        .createArtist(metadataUri, percentageArtist, percentageMostro)
        .accounts({
          admin: wallet.publicKey,
          adminAccount: wallet.publicKey,
          artistWallet: artistKeypair.publicKey,
          artistAccount: artistAccountPda,
          pumpTokenMint: mint,
          globalConfig: globalConfigPda,
          artistVaultAuthority: artistVaultAuthPda,
          artistVault: artistVaultPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      expect(typeof tx).to.equal('string');
      created = true;
    } catch (err: any) {
      const logs: string[] = err.logs || (err.error && err.error.logs) || [];
      // If PDA/account already exists, fetch existing and continue
      if (logs.join(' ').includes('already in use')) {
        console.warn('createArtist: PDA already exists, fetching existing account');
      } else {
        if (logs.length) console.error('Simulation logs:', logs);
        if (err.error && err.error.logs) console.error('Inner logs:', err.error.logs);
        throw err;
      }
    }

  // fetch the artist account and assert fields
  const raw = await provider.connection.getAccountInfo(artistAccountPda);
  console.log('artistAccount raw exists:', !!raw, 'len:', raw ? raw.data.length : 0, 'owner:', raw ? raw.owner.toBase58() : 'none');
  const artistAccount: any = await programAny.account.artist.fetch(artistAccountPda);
  console.log('artistAccount fetched:', artistAccount);
  expect(artistAccount.artistWallet.toBase58()).to.equal(artistKeypair.publicKey.toBase58());
  expect(artistAccount.pumpTokenMint.toBase58()).to.equal(mint.toBase58());
  expect(artistAccount.metadataUri).to.equal(metadataUri);
  expect(Number(artistAccount.percentageArtist)).to.equal(percentageArtist);
  expect(Number(artistAccount.percentageMostro)).to.equal(percentageMostro);
  });
});
