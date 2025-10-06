// // scripts/create_config.ts
// import * as anchor from "@coral-xyz/anchor"
// import { PublicKey, SystemProgram } from "@solana/web3.js"
// // import { MostroProgram } from "../target/types/mostro_program" // auto-generated IDL types

// async function main() {
//   // 1️⃣ Set up Anchor provider (wallet + connection)
//   const provider = anchor.AnchorProvider.local() // local wallet, can be changed
//   anchor.setProvider(provider)

//   // 2️⃣ Get program object
//   const program = anchor.workspace
//     .MostroProgram as anchor.Program<MostroProgram>

//   // 3️⃣ Derive a PDA for global config
//   const [configPda, configBump] = await PublicKey.findProgramAddress(
//     [Buffer.from("mostro_config")],
//     program.programId
//   )

//   // 4️⃣ Call the create_config instruction
//   console.log("Creating config PDA:", configPda.toBase58())

//   const tx = await program.methods
//     .createConfig({
//       admin: provider.wallet.publicKey, // admin wallet
//       percentageArtist: 10, // example: 10%
//       percentageMostro: 3, // example: 3%
//       percentageBondingCurve: 87 // example: 87%
//     })
//     .accounts({
//       config: configPda,
//       payer: provider.wallet.publicKey,
//       systemProgram: SystemProgram.programId
//     })
//     .rpc()

//   console.log("Transaction successful! Signature:", tx)
// }

// main()
//   .then(() => console.log("✅ Config created successfully"))
//   .catch((err) => console.error("❌ Error creating config:", err))
