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

    static PROPOSALS_MAP: RefCell<StableBTreeMap<u64, Proposal, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), // Use a different MemoryId if needed
        )
    );
}

