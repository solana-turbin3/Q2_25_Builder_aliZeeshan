import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CliffSafe } from "../target/types/cliff_safe";
import { Connection, Keypair, PublicKey, SystemProgram} from "@solana/web3.js";
import { createMint, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, mintTo } from "@solana/spl-token";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.cliffSafe as Program<CliffSafe>;

let creator: Keypair;

let mint: PublicKey;

let vestingContractInfo: PublicKey;

let vault: PublicKey;

let companyAta: PublicKey;
let beneficiaryAta: PublicKey;

const companyName = "CliffSafe";
const isRevocable = true;

let vestingRecordInfo: PublicKey;

let cliffBeneficiary: Keypair;
let cliffBeneficiaryAta: PublicKey;
let cliffBeneficiaryVestingRecordInfo: PublicKey;

before(async () => {
  creator = Keypair.generate();
  cliffBeneficiary = Keypair.generate();

  const transferIx = anchor.web3.SystemProgram.transfer({
    fromPubkey: provider.wallet.publicKey,
    toPubkey: creator.publicKey,
    lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
  });

  const tx = new anchor.web3.Transaction().add(transferIx);
  await provider.sendAndConfirm(tx);

  const transferIx2 = anchor.web3.SystemProgram.transfer({
    fromPubkey: provider.wallet.publicKey,
    toPubkey: cliffBeneficiary.publicKey,
    lamports: 2 * anchor.web3.LAMPORTS_PER_SOL,
  });

  const tx2 = new anchor.web3.Transaction().add(transferIx2);
  await provider.sendAndConfirm(tx2);

  mint = await createMint(
    provider.connection,
    creator,
    creator.publicKey,
    null,
    0,
  );

  [vestingContractInfo] = PublicKey.findProgramAddressSync(
    [Buffer.from(companyName), creator.publicKey.toBuffer(), mint.toBuffer()],
    program.programId
  );

  [vault] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), mint.toBuffer(), creator.publicKey.toBuffer()],
    program.programId
  );

  companyAta = await getAssociatedTokenAddress(
    mint,
    creator.publicKey
  );

  let beneficiaryAtaAccount = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    creator,
    mint,
    provider.wallet.publicKey
  );

  beneficiaryAta = beneficiaryAtaAccount.address;

  let cliffBeneficiaryAtaAccount = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    creator,
    mint,
    cliffBeneficiary.publicKey
  );

  cliffBeneficiaryAta = cliffBeneficiaryAtaAccount.address;

  [cliffBeneficiaryVestingRecordInfo] = PublicKey.findProgramAddressSync(
    [Buffer.from(companyName), cliffBeneficiary.publicKey.toBuffer(), mint.toBuffer()],
    program.programId
  );

  [vestingRecordInfo] = PublicKey.findProgramAddressSync(
    [vestingContractInfo.toBuffer(), provider.wallet.publicKey.toBuffer(), mint.toBuffer()],
    program.programId
  );
})


describe("cliff_safe", () => {
  it("Initialize Vesting", async () => {
    const tx = await program.methods.initializeVesting(companyName, isRevocable)
      .accounts({
        creator: creator.publicKey,
        mint,
        vestingContractInfo,
        vault,
        companyAta,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      }).signers([creator]).rpc();

      console.log(`Transaction Signature: ${tx}`);
  });

  it("Mint tokens", async () => {
    const mintAmount = new anchor.BN(1000000);
    const tx = await program.methods.mintTokens(mintAmount, companyName)
      .accounts({
        creator: creator.publicKey,
        mint,
        vestingContractInfo,
        vault,
        companyAta,
        tokenProgram: TOKEN_PROGRAM_ID,
      }).signers([creator]).rpc();

      console.log(`Transaction Signature: ${tx}`);
  });

  it("Deposit tokens", async () => {
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mint,
      companyAta,
      creator,
      1500,
    );

    const depositAmount = new anchor.BN(999);

    const tx = await program.methods.depositTokens(depositAmount, companyName)
      .accounts({
        creator: creator.publicKey,
        mint,
        vestingContractInfo,
        vault,
        companyAta,
        tokenProgram: TOKEN_PROGRAM_ID,
      }).signers([creator]).rpc();

      console.log(`Transaction Signature: ${tx}`);
    
  });

  it("Initialize Beneficiary record info", async () => {
    const vestingType = 1;
    const vestingStartTime = new anchor.BN(1741003429);
    const vestingEndTime = new anchor.BN(1746273829);
    const cliffPeriod = new anchor.BN(0);
    const vestingAmount = new anchor.BN(1000);

    const tx = await program.methods.initializeBeneficiary(companyName, vestingType, cliffPeriod, vestingStartTime, vestingEndTime, vestingAmount)
      .accounts({
        creator: creator.publicKey,
        beneficiary: provider.wallet.publicKey,
        mint,
        vestingContractInfo,
        vestingRecordInfo,
        beneficiaryAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      }).signers([creator]).rpc();

      console.log(`Transaction Signature: ${tx}`);
  });

  it("Revoke vesting", async () => {
    const tx = await program.methods.revokeVesting(companyName)
      .accounts({
        creator: creator.publicKey,
        mint,
        vestingContractInfo,
        vault,
        companyAta,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      }).signers([creator]).rpc();

      console.log(`Transaction Signature: ${tx}`);
  })

  it("Claim tokens", async () => {
    const tx = await program.methods.claimTokens(companyName)
      .accounts({
        beneficiary: provider.wallet.publicKey,
        creator: creator.publicKey,
        mint,
        vestingContractInfo,
        vestingRecordInfo,
        vault,
        beneficiaryAta,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      }).signers([provider.wallet.payer]).rpc();

      console.log(`Transaction Signature: ${tx}`);
  });

  it("Initialize beneficiary record info for cliff vesting", async () => {
    const vestingType = 0; 
    const vestingStartTime = new anchor.BN(1740821979); 
    const vestingEndTime = new anchor.BN(1746092379);
    const oneMonth = 30 * 24 * 60 * 60; 
    const cliffPeriod = new anchor.BN(oneMonth);
    const vestingAmount = new anchor.BN(1000);

    const tx = await program.methods.initializeBeneficiary(companyName, vestingType, cliffPeriod, vestingStartTime, vestingEndTime, vestingAmount)
      .accounts({
        creator: creator.publicKey,
        beneficiary: cliffBeneficiary.publicKey,
        mint,
        vestingContractInfo,
        cliffBeneficiaryVestingRecordInfo,
        cliffBeneficiaryAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      }).signers([creator]).rpc();

    console.log(`Transaction Signature: ${tx}`);
  });

  it("Claim tokens for cliff vesting", async () => {
    const tx = await program.methods.claimTokens(companyName)
      .accounts({
        beneficiary: cliffBeneficiary.publicKey,
        creator: creator.publicKey,
        mint,
        vestingContractInfo,
        cliffBeneficiaryVestingRecordInfo,
        vault,
        cliffBeneficiaryAta,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      }).signers([cliffBeneficiary]).rpc();

    console.log(`Transaction Signature: ${tx}`);
  });
  
});
