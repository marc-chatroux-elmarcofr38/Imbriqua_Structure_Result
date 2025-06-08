//! association_direction

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of AssociationDirection (Enumeration : AssociationDirection)
#[derive(Debug, Clone)]
pub enum association_direction {
    /// 'none' from (id : 'AssociationDirection-None', name : 'None')
    none, 
    /// 'one' from (id : 'AssociationDirection-One', name : 'One')
    one, 
    /// 'both' from (id : 'AssociationDirection-Both', name : 'Both')
    both, 
}
