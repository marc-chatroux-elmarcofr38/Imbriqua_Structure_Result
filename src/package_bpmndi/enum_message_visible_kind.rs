//! message_visible_kind
#![allow(unused_imports)]

use crate::package_bpmndi::*;
use crate::Builder;

/// Conversion of MessageVisibleKind (Enumeration : MessageVisibleKind)
#[derive(Debug, Clone)]
pub enum MessageVisibleKind {
    /// 'Initiating' from (id : 'MessageVisibleKind-initiating', name : 'initiating')
    Initiating, 
    /// 'NonInitiating' from (id : 'MessageVisibleKind-non_initiating', name : 'non_initiating')
    NonInitiating, 
}
