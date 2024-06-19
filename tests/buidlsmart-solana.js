const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("🚀 Starting test...")

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BuidlsmartSolana;
  const baseAccount = anchor.web3.Keypair.generate();
  let tx = await program.rpc.initialize({
    accounts: {
      baseApplication: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  console.log("📝 Your transaction signature", tx);

  let account = await program.account.application.fetch(baseAccount.publicKey);
  console.log("no. of applications", account.applicationList.length)

  await program.rpc.newApplication("demo app 1", {
    accounts: {
      baseApplication: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.application.fetch(baseAccount.publicKey);
  console.log("no. of applications", account.applicationList.length)
  console.log("application id: ", account.applicationList[0].id)
  console.log("application name: ", account.applicationList[0].name)

  await program.rpc.addObjectToApplication(0, "todo", ["task"], ["string"], {
    accounts: {
      baseApplication: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.application.fetch(baseAccount.publicKey);
  console.log("no. of objects", account.applicationList[0].objects.length)
  console.log("object id: ", account.applicationList[0].objects[0].id)
  console.log("object name: ", account.applicationList[0].objects[0].name)
  console.log("object field: ", account.applicationList[0].objects[0].field)

  await program.rpc.addFunctionToApplication(0, "do_something", "create", 0, ["input"], ["string"], {
    accounts: {
      baseApplication: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.application.fetch(baseAccount.publicKey);
  console.log("no. of functions", account.applicationList[0].functions.length)
  console.log("function id: ", account.applicationList[0].functions[0].id)
  console.log("function name: ", account.applicationList[0].functions[0].name)
  console.log("function parameter: ", account.applicationList[0].functions[0].parameters[0])
  console.log("function obj: ", account.applicationList[0].functions[0].target)
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();