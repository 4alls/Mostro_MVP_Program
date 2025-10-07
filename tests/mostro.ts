// import { AnchorProvider, BN, setProvider, web3 } from "@coral-xyz/anchor";

// const programId = new web3.PublicKey("2SYi3NFHTnCXHEzxNpa8nEyehkmZPyikbCarmxngSdTn");

import * as anchor from "@coral-xyz/anchor"
import { assert } from "chai"

describe("mostro_program", () => {
  const provider = anchor.AnchorProvider.local()
  anchor.setProvider(provider)

  const program = anchor.workspace.MostroProgram

  it("creates config PDA", async () => {
    const tx = await program.methods
      .createConfig({ admin: provider.wallet.publicKey })
      .rpc()
    assert.ok(tx)
  })
})
