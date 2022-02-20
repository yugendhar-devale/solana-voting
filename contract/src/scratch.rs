extern crate borsh;
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use std::collections::HashMap;


/* let mut voterData = VoterData::try_from_slice(instruction_data)?;
msg!("VoterData: {:?}", voterData);
msg!(
    "process_instruction: {}: {} accounts, data={:?}",
    program_id,
    accounts.len(),
    instruction_data
);
// Get the account that holds the vote count
for i in 0..accounts.len() {
    let account_info = next_account_info(accounts_iter)?;
    let account_data = &account_info.data;
    let account_bal = &account_info.lamports;
    let account_owner = account_info.owner;
    let account_executable = account_info.executable;
    msg!(
        "account {}: data={:#?}, lamports={:?}, owner={}, executable={}",
        i,
        account_data,
        account_bal,
        account_owner,
        account_executable
    );
} */

struct T {
    i: i32,
    s: String,
}
impl PartialEq for T {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.s == other.s
    }
}
impl Eq for T {}

#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Voter {
    pub vote_for: u8, // index of thr propsal the account is vooting for
    pub voted: bool,
    pub delagate: String, //If the account has delegated its vote
}
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Proposal {
    pub name: String,
    pub voteCount: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Ballot {
    pub name: String,
    pub chairPerson: String,
    pub proposals: Vec<Proposal>,
    pub voters: HashMap<String, Voter>, //user public address of the voter to get if he has already voted
}

//This starts the elections
// Ballot -> n Proposals(hardcoded) -> n Voters
//Chair person = who started the ballot
fn create_ballot(name: &String, chairPersonKey: &String) -> Ballot {
    //Save proposal to Program chain
    let all_proposals = (0..3)
        .map(|i| Proposal {
            name: format!("Propsal {}", i).to_string(),
            voteCount: 0,
        })
        .collect();
    let mut ballot = Ballot {
        name: name.to_string(),
        chairPerson: chairPersonKey.to_string(),
        proposals: all_proposals,
        voters: HashMap::new(),
    };
    ballot.voters.insert(chairPersonKey.to_string(), Voter {
        vote_for: 0,
        voted: true,
        delagate: "Shabaz".to_string(),
    });   ballot.voters.insert("sdkfhdskhk".to_string(), Voter {
        vote_for: 0,
        voted: true,
        delagate: "Shabaz".to_string(),
    });   ballot.voters.insert("fsdfhkjdshkfjh".to_string(), Voter {
        vote_for: 0,
        voted: true,
        delagate: "Shabaz".to_string(),
    });
    println!("Ballot created {:#?}", ballot);
    ballot
}
fn main() {
    let ballot = create_ballot(&"TestBallot".to_string(), &"ChairPerson".to_string());
    let sz = Ballot::try_to_vec(&ballot).unwrap();
    println!("Serialized {:?}", sz);
    println!("Serialized length {:?}", sz.len());
    let dz = Ballot::try_from_slice(&sz).unwrap();
    println!("Serialized {:?}", dz);

    /*     let s = "Hello, world!".to_string();
    println!("Strin {:?}", s);

    let sz = BorshSerialize::try_to_vec(&s).unwrap();
    println!("sz  {:?}", sz);
    let fdd: u8 = 32;
    let sm = BorshSerialize::try_to_vec(&fdd).unwrap();
    let input = [];//[sm, sz].concat();
    let (tag, rest) = input.split_first().unwrap_or_else(||{
        println!("err");
        (&0, &[0])
    });
    println!("tag {:?}", tag);
    println!("rest {:?}", rest);
    if(*tag != 0) {
    let dz:String = BorshDeserialize::try_from_slice(&rest).unwrap();
    println!("dz String {:#?}", dz); */
}
