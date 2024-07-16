let web3 = require("@solana/web3.js");

const solanaConnection = new web3.Connection(
  web3.clusterApiUrl("mainnet-beta")
);

const getTransactions = async (address, limit) => {
  const pubKey = new web3.PublicKey(address);
  let transactionList = await solanaConnection.getSignaturesForAddress(pubKey, {
    limit: limit,
  });
  let signatureList = transactionList.map(
    (transaction) => transaction.signature
  );

  console.log(signatureList);

  for await (const sig of signatureList) {
    console.log(
      await solanaConnection.getParsedTransaction(sig, {
        maxSupportedTransactionVersion: 0,
      })
    );
  }
};

let myAddress = "AzuiWapU4pttFt7qQLHaiQMcuhzVb2mwEok5LRWgZJZx";

getTransactions(myAddress, 3).then(() => {
  console.log("done");
});
