//! AssociationDirection
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of AssociationDirection (Enumeration : AssociationDirection)
#[derive(Debug, Clone)]
pub enum AssociationDirection {
    /// 'None' from (id : 'AssociationDirection-None', name : 'None')
    None, 
    /// 'One' from (id : 'AssociationDirection-One', name : 'One')
    One, 
    /// 'Both' from (id : 'AssociationDirection-Both', name : 'Both')
    Both, 
}
