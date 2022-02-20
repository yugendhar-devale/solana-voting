use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    program_error::ProgramError,
    system_instruction, system_program,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize, BorshSchema};
use std::mem;
use std::collections::HashMap;

/* 
Here is the plan
There will 3 account who will satand for election ,
They will submit the proposal with their name

the structure will have 
    public address of the account
    name of the account
    the amount of votes it got initially 0
    did it win the election

I will create the proposals manually the voting will be done by the contract

The user will select the proposal and vote for it the transaction will be signed and payed by the user
Once he votes a bool value voted will added to user account so that he can't vote again.
*/
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Voter {
    pub vote_for: u8, // index of thr propsal the account is vooting for
    pub voted: bool,
    pub delagate: String, //If the account has delegated its vote

}
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Proposal {
    pub id : u8,
    pub name : String,
    pub voteCount : u32,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct StringStruct {
    pub data: String
}
    
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Ballot {
    pub name: String,
    pub chairPerson : Pubkey,
    pub proposals: Vec<Proposal>,
    pub voters: HashMap<String, Voter>, //user public address of the voter to get if he has already voted
}


entrypoint!(process_instruction);
//This starts the elections
//Ballot is also an account owned by voting program
// Ballot -> n Proposals(hardcoded) -> n Voters
//Chair person = who started the ballot
/* fn create_ballot(program_id: &Pubkey, ballot_acc: &AccountInfo, name : &String, chairPersonAcc: &AccountInfo, system_program_account: &AccountInfo) -> Option<bool> {
    //Save proposal to Program chain
    msg!("creating ballot name {:?}", name);
    msg!("ballot acc {:?}", ballot_acc);
    msg!("name {:?}", name);
    msg!("chairPersonKey {:?}", chairPersonAcc.key);
    let all_proposals = (0..3).map(|i| Proposal {
        id: i as u8,
        name: format!("Propsal {}", i).to_string(),
        voteCount: 0,
    }).collect();
    let mut ballot = Ballot {
        name: name.to_string(),
        chairPerson: *chairPersonAcc.key,
        proposals: all_proposals,
        voters: HashMap::new(),
    };
    msg!("Ballot in memory {:#?}", ballot);
    let mut ballot_data_serialized = ballot.try_to_vec().unwrap();
    invoke(&system_instruction::create_account(
            &*chairPersonAcc.key,
            &*ballot_acc.key,
            500*1_000_000_000,
            (ballot_data_serialized.len() + 800) as u64,
            program_id,
        ),
        &[ //you need to pass all accounts invloved for txn to take place
            chairPersonAcc.clone(),
            ballot_acc.clone(),
            system_program_account.clone(),
        ],
    ).ok()?;

    msg!("Ballot account creatd {:#?}", ballot_acc);
    // Make this program the owner of the new account
    invoke(
        &system_instruction::assign(ballot_acc.key, program_id),
        &[ballot_acc.clone(), system_program_account.clone()],
    ).ok()?;

    // Write the serialized data to the time slot account
    ballot_data_serialized.swap_with_slice(*ballot_acc.try_borrow_mut_data().ok()?);

    msg!("Ballot account data {:#?}", ballot_acc);
    Some(true)
} */
fn vote(voter_public_key: &Pubkey, vote: &mut Voter, ballot_acc: &AccountInfo) {
    msg!("vote data {:?}", vote);
    let mut ballot: Ballot  = BorshDeserialize::try_from_slice(&ballot_acc.try_borrow_data().unwrap()).unwrap();
    msg!("Initial Ballot data {:?}", ballot);
    match ballot.voters.get(&(voter_public_key.to_string())) {
        Some(voter_data) => msg!("voter already there {:?}: {:#?}", voter_public_key, voter_data),
        None => {
            msg!("No data present for {:?} .", voter_public_key);
            let (mut proposal) = ballot.proposals.get_mut(vote.vote_for as usize).unwrap();
            proposal.voteCount = proposal.voteCount +1;
            let mut vote_clone  = vote.clone();
            vote_clone.voted = true;
            ballot.voters.insert(voter_public_key.to_string(), vote_clone);
            msg!("Ballot data after vote {:?}", ballot);
            let mut ballot_data_serialized = ballot.try_to_vec().unwrap();
            ballot_data_serialized.swap_with_slice(*ballot_acc.try_borrow_mut_data().unwrap());
            msg!("Ballot data writter to chain ");

        },
    }

}
fn read_account(ballot_acc: &AccountInfo) {
    let ballot: Ballot  = BorshDeserialize::try_from_slice(&ballot_acc.try_borrow_data().unwrap()).unwrap();
    msg!("Ballot data {:?}", ballot);
}
fn process_instruction(
    program_id: &Pubkey,      // Public key of program account
    accounts: &[AccountInfo], // data accounts , caller, ballot account and system program
    instruction_data: &[u8],  // 0 = error 1 = create proposal, 2 = vote
) -> ProgramResult {
    msg!("Rust program entrypoint");
    
    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();
    let calling_account = next_account_info(accounts_iter)?;
    let mut ballot_acc = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;
    let (tag, rest) = instruction_data.split_first().unwrap_or_else(||{
        msg!("err unwrapping");
        (&0, &[0])
    });

    msg!("tag : {} rest {:?}", tag, rest);
    match tag {
        1 => {
            msg!("invoked for create ballot");
            let rest_data: StringStruct = BorshDeserialize::try_from_slice(rest)?;
            let name: String = rest_data.data.clone();
            msg!("creating ballot with name {:?}", name);
            ///create_ballot(&program_id, &ballot_acc, &name, &calling_account, &system_program_account);
            let chairPersonAcc = calling_account;
            msg!("creating ballot name {:?}", name);
            msg!("ballot acc {:?}", ballot_acc);
            msg!("name {:?}", name);
            msg!("chairPersonKey {:?}", chairPersonAcc.key);
            let all_proposals = (0..3).map(|i| Proposal {
                id: i as u8,
                name: format!("Propsal {}", i).to_string(),
                voteCount: 0,
            }).collect();
            let mut ballot = Ballot {
                name: name.to_string(),
                chairPerson: *chairPersonAcc.key,
                proposals: all_proposals,
                voters: HashMap::new(),
            };
            msg!("Ballot in memory {:#?}", ballot);
            let mut ballot_data_serialized = ballot.try_to_vec().unwrap();
            let create_result = invoke(&system_instruction::create_account(
                    &*chairPersonAcc.key,
                    &*ballot_acc.key,
                    500*1_000_000_000,
                    (ballot_data_serialized.len()) as u64,
                    program_id,
                ),
                &[ //you need to pass all accounts invloved for txn to take place
                    chairPersonAcc.clone(),
                    ballot_acc.clone(),
                    system_program_account.clone(),
                ],
            ).unwrap();
            msg!("Ballot create_result {:#?}", create_result);
            msg!("Ballot account creatd {:#?}", ballot_acc);
            // Make this program the owner of the new account
            let result = invoke(
                &system_instruction::assign(ballot_acc.key, program_id),
                &[ballot_acc.clone(), system_program_account.clone()],
            ).unwrap();
            msg!("assign result {:?}", result);
        
            // Write the serialized data to the time slot account
            ballot_data_serialized.swap_with_slice(*ballot_acc.try_borrow_mut_data().unwrap());
        
        },
        2 => {
            //do this voting here
            msg!("invoked for vote");
            let mut vote_data: Voter = BorshDeserialize::try_from_slice(rest)?;

            msg!("vote data {:?}", vote_data);
            vote(&calling_account.key, &mut vote_data, &mut ballot_acc);
        },
        3 => {
            msg!("Reading Data");
            read_account(&ballot_acc)
        },
        _ => msg!("Unknown instruction"),
    }
    Ok(())  
}

/* // tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_sanity() {
        // mock program id

        let program_id = Pubkey::default();

        // mock accounts array...

        let key = Pubkey::default(); // anything
        let mut lamports = 0;

        let mut data = vec![0; 2 * mem::size_of::<u32>()];
        LittleEndian::write_u32(&mut data[0..4], 0); // set storage to zero
        LittleEndian::write_u32(&mut data[4..8], 0);

        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,             // account pubkey
            false,            // is_signer
            true,             // is_writable
            &mut lamports,    // balance in lamports
            &mut data,        // storage
            &owner,           // owner pubkey
            false,            // is_executable
            Epoch::default(), // rent_epoch
        );

        let mut instruction_data: Vec<u8> = vec![0];

        let accounts = vec![account];

        assert_eq!(LittleEndian::read_u32(&accounts[0].data.borrow()[0..4]), 0);
        assert_eq!(LittleEndian::read_u32(&accounts[0].data.borrow()[4..8]), 0);

        // vote for candidate 1

        instruction_data[0] = 1;
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(LittleEndian::read_u32(&accounts[0].data.borrow()[0..4]), 1);
        assert_eq!(LittleEndian::read_u32(&accounts[0].data.borrow()[4..8]), 0);

        // vote for candidate 2

        instruction_data[0] = 2;
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(LittleEndian::read_u32(&accounts[0].data.borrow()[0..4]), 1);
        assert_eq!(LittleEndian::read_u32(&accounts[0].data.borrow()[4..8]), 1);
    }
}
 */