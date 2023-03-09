#![no_std]

use gmeta::metawasm;
use gstd::{prelude::*, ActorId};
use escrow_io::*;

#[metawasm]
pub trait Metawasm {
   type State = Escrow;

   fn seller(state: Self::State) -> ActorId {
       state.seller
   }

   fn buyer(state: Self::State) -> ActorId {
       state.buyer
   }

   fn escrow_state(state: Self::State) -> EscrowState {
       state.state
   }
}