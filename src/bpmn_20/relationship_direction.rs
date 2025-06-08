//! relationship_direction

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of RelationshipDirection (Enumeration : RelationshipDirection)
#[derive(Debug, Clone)]
pub enum relationship_direction {
    /// 'none' from (id : 'RelationshipDirection-None', name : 'None')
    none, 
    /// 'forward' from (id : 'RelationshipDirection-Forward', name : 'Forward')
    forward, 
    /// 'backward' from (id : 'RelationshipDirection-Backward', name : 'Backward')
    backward, 
    /// 'both' from (id : 'RelationshipDirection-Both', name : 'Both')
    both, 
}
