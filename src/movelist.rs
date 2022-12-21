use crate::types::Move;
use arrayvec::ArrayVec;
#[doc = " A container for moves that can be stored inline on the stack."]
#[doc = ""]
#[doc = " The capacity is limited, but there is enough space to hold the legal"]
#[doc = " moves of any chess position, including any of the supported chess variants,"]
#[doc = " if enabled."]
#[doc = ""]
#[doc = " # Example"]
#[doc = ""]
#[doc = " ```"]
#[doc = " use shakmaty::{Chess, Position, Role};"]
#[doc = ""]
#[doc = " let pos = Chess::default();"]
#[doc = " let mut moves = pos.legal_moves();"]
#[doc = " moves.retain(|m| m.role() == Role::Pawn);"]
#[doc = " assert_eq!(moves.len(), 16);"]
#[doc = " ```"]
pub type MoveList = ArrayVec<
    Move,
    {
        #[cfg(feature = "variant")]
        {
            512
        }
        #[cfg(not(feature = "variant"))]
        {
            256
        }
    },
>;
