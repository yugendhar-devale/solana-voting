import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import * as borsh from "borsh";
let connection;
let defaultWallet;
export let programWallet;
let programId;
export async function getConnection() {
  connection = new Connection("http://127.0.0.1:8899", "confirmed");
  //For checking whether the connection is successfully made
  console.log(connection.getSlot());
}
export async function getDefaultWallet() {
  defaultWallet = Keypair.fromSecretKey(
    Uint8Array.from([
      173, 212, 194, 66, 241, 46, 249, 59, 115, 78, 97, 188, 226, 86, 130, 221,
      21, 129, 183, 137, 226, 117, 148, 90, 198, 243, 82, 29, 61, 155, 115, 92,
      248, 98, 190, 139, 6, 18, 87, 4, 190, 82, 2, 126, 32, 250, 51, 170, 61,
      252, 41, 102, 48, 9, 76, 79, 39, 60, 228, 15, 90, 153, 193, 135,
    ])
  );
}
programWallet = Keypair.fromSecretKey(
  Uint8Array.from([
    25, 150, 223, 234, 168, 5, 79, 211, 66, 248, 120, 169, 175, 2, 236, 237,
    24, 150, 136, 45, 241, 212, 80, 232, 108, 69, 20, 200, 210, 42, 42, 96,
    127, 217, 168, 146, 172, 96, 141, 199, 37, 211, 212, 41, 222, 101, 173,
    150, 224, 152, 172, 31, 212, 143, 28, 64, 120, 203, 165, 240, 232, 182,
    103, 108,
  ])
);
export async function getProgramId() {
  programId = programWallet.publicKey;
}
class VoterData {
  voteFor = 0; //index of the proposal
  voted = false; //whether the voter has voted
  delegate = ""; //delegate to vote for publicKey in string
  constructor(fields) {
    console.log("VoterData constructor: ",fields);
    if (fields) {
      this.voteFor = fields.voteFor;
      this.voted = fields.voted;
      this.delegate = fields.delegate;
    }
  }
}
const VoterDataSchema = new Map([
  [
    VoterData,
    {
      kind: "struct",
      fields: [
        ["voteFor", "u8"],
        ["voted", "u8"],
        ["delegate", "String"],
      ],
    },
  ],
]);

class StringStruct {
  data = "";
  constructor(fields) {
    console.log(fields);
    if (fields) {
      this.data = fields.data;
    }
  }
}
const StringStructSchema = new Map([
  [
    StringStruct,
    {
      kind: "struct",
      fields: [
        ["data", "String"],
      ],
    },
  ],
]);

export async function invokeProgram() {
  console.log("Saying hello to", programId);
  const v = new VoterData({
    publickey: defaultWallet.publicKey.toString(),
    vote: 3,
  });
  console.log("VoterData = ", v);
  const ser = borsh.serialize(VoterSchema, v);
  console.log("ser = ", ser);
  const instruction = new TransactionInstruction({
    keys: [
      { pubkey: defaultWallet.publicKey, isSigner: false, isWritable: true },
    ],
    programId,
    data: Buffer.from(borsh.serialize(VoterSchema, v)),
  });
  let sig = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [defaultWallet]
  );
  console.log("confirmation sig:", sig);
}
export async function getKeyForBallot() {
    return Keypair.generate();
}
export async function createBallot(ballotName) {
  console.log("Creating ballot using ", programId, " name ", ballotName);
  const strucData = new StringStruct({data: ballotName});
  const ser = borsh.serialize(StringStructSchema, strucData);
  
  const data = [1, ...ser]; //1 = create ballot
  console.log("Data  = ", data);
  const ballotKey = await getKeyForBallot();

  console.log("Ballot Ptivate Key = ", ballotKey.secretKey.toString());
  console.log('Caller =', defaultWallet.publicKey.toString());
  console.log('ballotKey =', ballotKey.publicKey.toString());
 // const sysProg = new PublicKey.from("11111111111111111111111111111111");
  console.log('SystemProgram.progId =', SystemProgram.programId.toString());
  const instruction = new TransactionInstruction({
    keys: [
      { pubkey: defaultWallet.publicKey, isSigner: false, isWritable: true },
      { pubkey: ballotKey.publicKey, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isWritable: false },
    ],
    programId,
    data: Buffer.from(data),
  });
  let sig = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [defaultWallet, ballotKey]
  );
  console.log("confirmation sig:", sig);
}
export const ballotAcc = Keypair.fromSecretKey(Uint8Array.from([199,4,242,159,73,81,207,180,74,188,180,127,176,129,241,120,165,234,166,168,183,95,174,15,101,138,192,198,75,160,197,218,16,121,110,94,161,107,169,88,193,88,200,218,0,236,80,33,198,124,146,241,237,245,107,235,255,92,93,97,131,27,87,171]));

export async function getWeb3AccInfo(wallet) {
    return await connection.getAccountInfo(wallet.publicKey);
}
export async function getWeb3ProgramAccounts(wallet) {
    return await connection.getProgramAccounts(wallet.publicKey);
}
export async function getWeb3BallotAccData(wallet) {
    let accInfo = await getWeb3AccInfo(wallet);
    let data = accInfo.data.data;
    //let ballotData = borsh.deserialize(StringStructSchema, data);
}

export async function readData(dummyData) {
  console.log("readData using ", programId, " dummyData ", dummyData);
  const strucData = new StringStruct({data: dummyData});
  const ser = borsh.serialize(StringStructSchema, strucData);
  
  const data = [3, ...ser]; //1 = create ballot
  console.log("Data  = ", data);

  console.log('Caller =', defaultWallet.publicKey.toString());
  console.log('ballotKey =', ballotAcc.publicKey.toString());
 // const sysProg = new PublicKey.from("11111111111111111111111111111111");
  console.log('SystemProgram.progId =', SystemProgram.programId.toString());
  const instruction = new TransactionInstruction({
    keys: [
      { pubkey: defaultWallet.publicKey, isSigner: false, isWritable: true },
      { pubkey: ballotAcc.publicKey, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isWritable: false },
    ],
    programId,
    data: Buffer.from(data),
  });
  let sig = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [defaultWallet, ballotAcc]
  );
  console.log("confirmation sig:", sig);
}

export const voter1 = Keypair.fromSecretKey(Uint8Array.from([166,139,104,95,225,20,180,108,222,213,180,34,165,82,130,151,39,173,10,144,168,118,33,127,215,91,125,130,16,198,186,201,198,206,25,0,65,142,131,188,174,210,119,197,26,12,240,67,74,180,97,93,85,126,150,209,122,110,85,97,230,67,8,179]));
export const voter2 = Keypair.fromSecretKey(Uint8Array.from([165,231,253,182,179,198,184,22,250,249,245,223,152,231,90,145,197,45,155,184,214,239,81,195,23,4,161,254,77,169,252,146,158,64,177,89,65,9,216,234,114,230,239,195,87,140,92,140,138,8,43,3,127,67,35,123,40,244,183,127,143,39,161,168]));

export async function vote(voterWallet, voteFor) {
  console.log("Voting for: ", voteFor, " from: ", voterWallet.publicKey.toString());
  const voterData = new VoterData({
    voteFor: voteFor,
    voted: false,
    delegate: "Shabaz testing",
  });
  const ser = borsh.serialize(VoterDataSchema, voterData);
  
  const data = [2, ...ser]; //2 = vote
  console.log("Data  = ", data);

  console.log('Voter =', voterWallet.publicKey.toString());
  console.log('ballotKey =', ballotAcc.publicKey.toString());
  console.log('SystemProgram.progId =', SystemProgram.programId.toString());
  const instruction = new TransactionInstruction({
    keys: [
      { pubkey: voterWallet.publicKey, isSigner: false, isWritable: true },
      { pubkey: ballotAcc.publicKey, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isWritable: false },
    ],
    programId,
    data: Buffer.from(data),
  });
  let sig = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [voterWallet, ballotAcc]
  );
  console.log("voting confirmation sig:", sig);
}
