var web3 = require("@solana/web3.js");
const progWallet = web3.Keypair.fromSecretKey(
  Uint8Array.from([25,150,223,234,168,5,79,211,66,248,120,169,175,2,236,237,24,150,136,45,241,212,80,232,108,69,20,200,210,42,42,96,127,217,168,146,172,96,141,199,37,211,212,41,222,101,173,150,224,152,172,31,212,143,28,64,120,203,165,240,232,182,103,108])
);
console.log("progWallet publicKey:", progWallet.publicKey.toString());