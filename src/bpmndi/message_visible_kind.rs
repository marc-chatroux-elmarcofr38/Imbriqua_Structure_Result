//! message_visible_kind

use crate::bpmndi::*;
use crate::Builder;

/// Conversion of MessageVisibleKind (Enumeration : MessageVisibleKind)
#[derive(Debug, Clone)]
pub enum message_visible_kind {
    /// 'initiating' from (id : 'MessageVisibleKind-initiating', name : 'initiating')
    initiating, 
    /// 'non_initiating' from (id : 'MessageVisibleKind-non_initiating', name : 'non_initiating')
    non_initiating, 
}
