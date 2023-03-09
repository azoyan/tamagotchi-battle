#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<(), ()>;
    type Handle = InOut<(), ()>;
    type Reply = InOut<(), ()>;
    type Others = InOut<(), ()>;
    type Signal = ();
    type State = ();
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum BattleState {
    Registration,
    Moves,
    Waiting,
    GameIsOver,
}

impl Default for BattleState {
    fn default() -> Self {
        BattleState::Registration
    }
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum BattleAction {
    UpdateInfo,
    MakeMove,
    Registration { tmg_id: ActorId },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum BattleEvent {
    Registered { tmg_id: ActorId },
    GameIsOver,
    MoveMade,
    InfoUpdated,
    GoToWaitingState,
}
