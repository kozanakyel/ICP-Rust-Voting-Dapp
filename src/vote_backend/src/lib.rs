use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};


#[derive(CandidType, Deserialize)]
struct Proposal {
    description: String,
    approve: u32,
    reject: u32,
    pass: u32,
    is_active: bool,
    voted: Vec<candid::Principal>,
    owner: candid::Principal,
}

#[derive(CandidType, Deserialize)]
struct CreateProposal {
    description: String,
    is_active: bool,
}

#[derive(CandidType, Deserialize)]
enum VoteTypes {
    Approve,
    Reject,
    Pass,
}

#[derive(CandidType, Deserialize)]
enum VoteError {
    AlreadyVoted,
    ProposalNotActive,
    Unauthorized,
    NoProposal,
    UpdateError,
    VoteFailed,
}

impl Storable for Proposal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

type Memory = VirtualMemory<DefaultMemoryImpl>;
const MAX_VALUE_SIZE: u32 = 100;

// Implement BoundedStorable for Proposal
impl BoundedStorable for Proposal {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE; // Adjust the size as needed
    const IS_FIXED_SIZE: bool = false;
}

// Initialize the proposals map with a new MemoryId
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PROPOSAL_MAP: RefCell<StableBTreeMap<u64, Proposal, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), // Use a different MemoryId if needed
        )
    );
}


#[ic_cdk_macros::query]
fn get_proposal(key: u64) -> Option<Proposal> {
    PROPOSAL_MAP.with(|p| p.borrow().get(&key))
}

#[ic_cdk_macros::query]
fn get_proposal_count() -> u64 {
    PROPOSAL_MAP.with(|p| p.borrow().len())
}

#[ic_cdk_macros::update]
fn create_proposal(key: u64, proposal: CreateProposal) -> Option<Proposal> { 
    let value = Proposal {
        description: proposal.description,
        approve: 0u32,
        reject: 0u32,
        pass: 0u32,
        is_active: proposal.is_active,
        voted: vec![],
        owner: ic_cdk::caller(),
    };
    PROPOSAL_MAP.with(|p| p.borrow_mut().insert(key, value))
}


#[ic_cdk_macros::update]
fn edit_proposal(key: u64, proposal: CreateProposal) -> Result<(), VoteError> {
    PROPOSAL_MAP.with(|p| {
        let old_proposal = match p.borrow().get(&key) {
            Some(value) => value,
            None => return Err(VoteError::NoProposal),
        };
        if ic_cdk::caller() != old_proposal.owner {            
            return Err(VoteError::Unauthorized);
        }
        let value = Proposal {
            description: proposal.description,
            approve: old_proposal.approve,
            reject: old_proposal.reject,
            pass: old_proposal.pass,
            is_active: proposal.is_active,
            voted: old_proposal.voted,
            owner: ic_cdk::caller(),
        };
        let res = p.borrow_mut().insert(key, value);
        match res {
            Some(_) => Ok(()),
            None => Err(VoteError::UpdateError),
        }
    })
}

#[ic_cdk_macros::update]
fn end_proposal(key: u64) -> Result<(), VoteError> {
    PROPOSAL_MAP.with(|p| {
        let mut proposal = p.borrow_mut().get(&key).unwrap();
        if ic_cdk::caller() != proposal.owner {
            return Err(VoteError::Unauthorized);
        }
        proposal.is_active = false;
        let res = p.borrow_mut().insert(key, proposal);
        match res {
            Some(_) => Ok(()),
            None => Err(VoteError::UpdateError),
        }
    })
}

#[ic_cdk_macros::update]
fn vote(key: u64, choice: VoteTypes) -> Result<(), VoteError> {
    PROPOSAL_MAP.with(|p| {
        let mut proposal = p.borrow_mut().get(&key).unwrap();
        let caller = ic_cdk::caller();
        if proposal.voted.contains(&caller) {
            return Err(VoteError::AlreadyVoted);
        } else if !proposal.is_active {
            return Err(VoteError::ProposalNotActive);
        }
        match choice {
            VoteTypes::Approve => proposal.approve += 1,
            VoteTypes::Reject => proposal.reject += 1,
            VoteTypes::Pass => proposal.pass += 1,
        }
        proposal.voted.push(caller);
        let res = p.borrow_mut().insert(key, proposal);
        match res {
            Some(_) => Ok(()),
            None => Err(VoteError::VoteFailed),
        }
    })
}

#[ic_cdk_macros::query]
fn get_proposal_status(key: u64) -> String {
    let proposal = PROPOSAL_MAP.with(|p| p.borrow().get(&key));

    match proposal {
        Some(proposal) => {
            // Check if the proposal has at least 5 votes
            if proposal.approve + proposal.reject + proposal.pass < 5 {
                return String::from("UNDECIDED");
            }

            let total_votes = proposal.approve + proposal.reject + proposal.pass;

            // Calculate the majority condition (at least 50% of votes)
            let majority_condition = total_votes / 2;

            // Determine the status based on votes
            if proposal.approve >= majority_condition {
                return String::from("APPROVED");
            } else if proposal.reject >= majority_condition {
                return String::from("REJECTED");
            } else if proposal.pass >= majority_condition {
                return String::from("PASSED");
            } else {
                return String::from("UNDECIDED");
            }
        }
        None => String::from("NO_PROPOSAL"),
    }
}


