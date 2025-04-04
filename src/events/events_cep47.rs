use casper_sdk::{casper::Entity, prelude::*, types::Address};

use crate::types::TokenIdentifier;

#[casper]
#[derive(Debug, Clone, PartialEq)]
pub enum CEP47Event {
    Mint {
        recipient: Address,
        token_id: TokenIdentifier,
    },
    Burn {
        owner: Entity,
        token_id: TokenIdentifier,
        burner: Entity,
    },
    ApprovalGranted {
        owner: Address,
        spender: Address,
        token_id: TokenIdentifier,
    },
    ApprovalRevoked {
        owner: Address,
        token_id: TokenIdentifier,
    },
    ApprovalForAll {
        owner: Address,
        operator: Address,
    },
    RevokedForAll {
        owner: Address,
        operator: Address,
    },
    Transfer {
        sender: Address,
        recipient: Address,
        token_id: TokenIdentifier,
    },
    MetadataUpdate {
        token_id: TokenIdentifier,
    },
    VariablesSet,
    Migrate,
}
