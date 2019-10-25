/*
  Copyright (C) 2018-2019 The Purple Core Developers.
  This file is part of the Purple Core Library.

  The Purple Core Library is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  The Purple Core Library is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with the Purple Core Library. If not, see <http://www.gnu.org/licenses/>.
*/

use crate::chain::ChainErr;
use crate::types::*;
use crate::pow_chain::block::PENDING_VAL_BUF_SIZE;
use crate::pow_chain::epoch_info::EpochInfo;
use crate::pow_chain::validator_entry::ValidatorEntry;
use crypto::{Hash, NodeId};
use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;
use std::net::SocketAddr;

// /// How many epochs to keep in the backlog
// pub const EPOCH_BACKLOG_SIZE: usize = 10;

#[derive(Clone, PartialEq, Debug)]
/// Chain state associated with proof-of-work chains.
/// This is used to calculate the difficulty on the `PowChain`.
pub struct PowChainState {
    /// The current chain height
    pub height: u64,

    /// Current difficulty
    pub difficulty: u64,

    /// Current edge bits
    pub edge_bits: u8,

    /// This denotes the first epoch where a validator will be 
    /// leaving the active validator set. This is `None` if there
    /// is no current active validator set.
    pub first_end_epoch: Option<u64>,

    /// This denotes the last epoch which will have an active
    /// validator set with this configuration. This is `None` if there
    /// is no current active validator set.
    pub last_end_epoch: Option<u64>,

    /// Stack containing buffered validator ids that are 
    /// currently awaiting to join the validator pool.
    pub pending_validators: VecDeque<NodeId>,

    /// Lookup table between node ids and active validator entries.
    pub active_validator_lookup: HashMap<NodeId, ValidatorEntry>,

    /// Lookup table between node ids and pending validator entries.
    pub pending_validator_lookup: HashMap<NodeId, ValidatorEntry>,

    /// Set containing ips of active validators. Used for validation.
    pub active_validator_ips: HashSet<SocketAddr>,

    /// Set containing ips of pending validators. Used for validation.
    pub pending_validator_ips: HashSet<SocketAddr>,

    /// Mapping between epochs and node ids who should join in those epochs.
    pub start_epochs_mapping: HashMap<u64, HashSet<NodeId>>,

    /// Mapping between epochs and node ids who should leave in those epochs.
    pub end_epochs_mapping: HashMap<u64, HashSet<NodeId>>,
}

impl PowChainState {
    pub fn genesis() -> Self {
        PowChainState {
            height: 0,
            difficulty: 0,
            edge_bits: miner::MIN_EDGE_BITS,
            first_end_epoch: None,
            last_end_epoch: None,
            pending_validators: VecDeque::new(),
            active_validator_lookup: HashMap::new(),
            pending_validator_lookup: HashMap::new(),
            active_validator_ips: HashSet::new(),
            pending_validator_ips: HashSet::new(),
            start_epochs_mapping: HashMap::new(),
            end_epochs_mapping: HashMap::new(),
        }
    }

    /// Returns the number of active validators
    pub fn active_validator_count(&self) -> u64 {
        self.active_validator_lookup.len() as u64
    }

    /// Returns the number of validators that are still
    /// waiting to join an active validator pool.
    pub fn pending_validator_count(&self) -> u64 {
        self.pending_validator_lookup.len() as u64
    }

    /// Returns true if there is a validator with the given id 
    /// that is either active in the pool or awaiting to join one.
    pub fn is_pending_or_active(&self, node_id: &NodeId) -> bool {
        self.active_validator_lookup.get(node_id).is_some() || self.pending_validator_lookup.get(node_id).is_some()
    } 

    /// Returns a `HashMap` containing the active validator set and their
    /// total allocated share of events.
    pub fn get_active_validator_set(&self) -> HashMap<NodeId, u64> {
        self.active_validator_lookup
            .iter()
            .map(|(key, entry)| (key.clone(), entry.total_allocated))
            .collect()
    }
    
    /// Returns the start pow block for the validator with the given `NodeId`
    /// if there is any entry for it. Returns `None` if there is no validator 
    /// with the given id.
    pub fn get_start_pow_block(&self, id: &NodeId) -> Option<Hash> {
        let active_result = self
            .active_validator_lookup
            .get(id);

        let pending_result = self
            .pending_validator_lookup
            .get(id);
        
        match (active_result, pending_result) {
            (Some(entry), None) => Some(entry.start_pow_block),
            (None, Some(entry)) => Some(entry.start_pow_block),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}

impl Flushable for PowChainState {
    fn flush(&mut self) -> Result<(), ChainErr> {
        Ok(())
    }
}
