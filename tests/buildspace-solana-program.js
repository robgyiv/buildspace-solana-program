// TODO these aren't really tests right now...
const anchor = require('@project-serum/anchor');

// Need the system program
const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log('🚀 Starting test...');

  const provider = anchor.Provider.env();
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.BuildspaceSolanaProgram;

  const GIF_LINK = 'https://media.giphy.com/media/8CPMooKzUbgpP6AyDE/giphy.gif';

  // Create an account keypair for our program to use
  const baseAccount = anchor.web3.Keypair.generate();

  // Call start_stuff_off, passing it the params it needs
  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log('📝 Your transaction signature', tx);

  // Fetch data from the account
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString());

  // Call add_gif
  await program.rpc.addGif(GIF_LINK, {
    accounts: {
      baseAccount: baseAccount.publicKey,
    },
  });

  // Get the account again to check it's changed
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString());

  // Access gif_list on the account!
  console.log('👀 GIF List', account.gifList);

  // Upvote
  console.log('Upvoting...');
  await program.rpc.upvoteItem(0, {
    accounts: {
      baseAccount: baseAccount.publicKey,
    },
  });
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('Upvoted GIF List', account.gifList);
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (e) {
    console.error(e);
    process.exit(1);
  }
};

runMain();
