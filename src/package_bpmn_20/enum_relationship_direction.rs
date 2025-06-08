//! relationship_direction
#![allow(unused_imports)]

use crate::package_bpmn_20::*;
use crate::Builder;

/// Conversion of RelationshipDirection (Enumeration : RelationshipDirection)
#[derive(Debug, Clone)]
pub enum RelationshipDirection {
    /// 'None' from (id : 'RelationshipDirection-None', name : 'None')
    None, 
    /// 'Forward' from (id : 'RelationshipDirection-Forward', name : 'Forward')
    Forward, 
    /// 'Backward' from (id : 'RelationshipDirection-Backward', name : 'Backward')
    Backward, 
    /// 'Both' from (id : 'RelationshipDirection-Both', name : 'Both')
    Both, 
}
