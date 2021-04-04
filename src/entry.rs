use crate::values::ValueId;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug,PartialEq)]
pub enum Entry {
    Value {
        value_ref: ValueId,
        seq_number: u64
    },
    Deletion {
        seq_number: u64,
        // not actually used but ensures the enum is always of the same size
        _value_ref: ValueId,
    }
}

impl Entry {
    pub fn get_sequence_number(&self) -> u64 {
        match self {
            Self::Value{seq_number, ..} | Self::Deletion{seq_number, ..} => *seq_number
        }
    }

    #[ allow(dead_code) ]
    pub fn get_value_ref(&self) -> Option<&ValueId> {
        match self {
            Self::Value{ value_ref, .. } => Some(value_ref),
            Self::Deletion{..} => None,
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry::Value{ seq_number: 0, value_ref: (0,0) }
    }
}
