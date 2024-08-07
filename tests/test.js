const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;
const assert = require("assert");

describe("solana_itus", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaItus;

  let state = null;
  let bottomToken = null;
  let topToken = null;
  let lpBottomTop = null;
  let lpBottomSol = null;
  let lpTopSol = null;
  let userRewards = null;

  before(async () => {
    bottomToken = await createMint(provider);
    topToken = await createMint(provider);

    state = anchor.web3.Keypair.generate();
    lpBottomTop = anchor.web3.Keypair.generate();
    lpBottomSol = anchor.web3.Keypair.generate();
    lpTopSol = anchor.web3.Keypair.generate();
    userRewards = anchor.web3.Keypair.generate();

    await program.rpc.initialize(new anchor.BN(60), new anchor.BN(1000), {
      accounts: {
        state: state.publicKey,
        bottomToken: bottomToken.publicKey,
        topToken: topToken.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [state],
    });

    await program.rpc.initializeTokens({
      accounts: {
        bottomToken: bottomToken.publicKey,
        topToken: topToken.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: anchor.web3.TokenInstructions.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [bottomToken, topToken],
    });

    await program.rpc.initializePools({
      accounts: {
        bottomToken: bottomToken.publicKey,
        topToken: topToken.publicKey,
        lpBottomTop: lpBottomTop.publicKey,
        lpBottomSol: lpBottomSol.publicKey,
        lpTopSol: lpTopSol.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: anchor.web3.TokenInstructions.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [lpBottomTop, lpBottomSol, lpTopSol],
    });
  });

  it("Initializes the state", async () => {
    const stateAccount = await program.account.state.fetch(state.publicKey);
    console.log("State:", stateAccount);
    assert.ok(stateAccount.epochDuration.toNumber() === 60);
    assert.ok(stateAccount.maxReward.toNumber() === 1000);
  });

  it("Votes for epoch duration", async () => {
    await program.rpc.voteForEpochDuration(
      { daily: {} },
      {
        accounts: {
          state: state.publicKey,
          user: provider.wallet.publicKey,
        },
      }
    );

    const stateAccount = await program.account.state.fetch(state.publicKey);
    console.log("State after voting for daily epoch:", stateAccount);
    assert.ok(stateAccount.dailyVotes.toNumber() > 0);
  });

  it("Performs public sale", async () => {
    await program.rpc.publicSale(new anchor.BN(1000), {
      accounts: {
        bottomToken: bottomToken.publicKey,
        topToken: topToken.publicKey,
        bottomTokenAccount: bottomToken.publicKey,
        topTokenAccount: topToken.publicKey,
        user: provider.wallet.publicKey,
        lpBottomTop: lpBottomTop.publicKey,
        lpBottomSol: lpBottomSol.publicKey,
        lpTopSol: lpTopSol.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: anchor.web3.TokenInstructions.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
    });

    const stateAccount = await program.account.state.fetch(state.publicKey);
    console.log("State after public sale:", stateAccount);
    assert.ok(stateAccount.bottomTokenSupply.toNumber() > 0);
    assert.ok(stateAccount.topTokenSupply.toNumber() > 0);
  });

  it("Settles an epoch", async () => {
    await program.rpc.settleEpoch({
      accounts: {
        state: state.publicKey,
        bottomToken: bottomToken.publicKey,
        topToken: topToken.publicKey,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      },
    });

    const stateAccount = await program.account.state.fetch(state.publicKey);
    console.log("State after settle:", stateAccount);
    assert.ok(stateAccount.lastEpochTimestamp.toNumber() > 0);
  });

  it("Distributes rewards", async () => {
    await program.rpc.distributeRewards({
      accounts: {
        state: state.publicKey,
        rewardsAccount: provider.wallet.publicKey,
        tokenProgram: anchor.web3.Token.programId,
      },
    });

    const stateAccount = await program.account.state.fetch(state.publicKey);
    console.log("State after distribute:", stateAccount);
    assert.ok(Object.keys(stateAccount.epochRewards).length > 0);
  });

  it("Claims rewards", async () => {
    const epochId = 1;

    await program.rpc.claimRewards(new anchor.BN(epochId), {
      accounts: {
        state: state.publicKey,
        user: provider.wallet.publicKey,
        userBottomAccount: bottomToken.publicKey,
        userTopAccount: topToken.publicKey,
      },
    });

    const stateAccount = await program.account.state.fetch(state.publicKey);
    console.log("State after claim:", stateAccount);
    assert.ok(
      stateAccount.hasClaimedReward[epochId.toString()][
        provider.wallet.publicKey.toString()
      ]
    );
  });

  async function createMint(provider) {
    const token = await anchor.web3.Token.createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      9,
      anchor.web3.TokenInstructions.TOKEN_PROGRAM_ID
    );
    return token;
  }
});
