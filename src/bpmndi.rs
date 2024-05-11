//! bpmndi

/// Conversion of _packageImport.0 (PackageImport)
use crate::di;

/// Conversion of _packageImport.1 (PackageImport)
use crate::dc;

/// Conversion of _packageImport.2 (PackageImport)
use crate::bpmn_20;

/// Conversion of ParticipantBandKind (Enumeration : ParticipantBandKind)
pub enum ParticipantBandKind {
    /// 'TopInitiating' from (id : 'ParticipantBandKind-top_initiating', name : 'top_initiating')
    TopInitiating,
    /// 'MiddleInitiating' from (id : 'ParticipantBandKind-middle_initiating', name : 'middle_initiating')
    MiddleInitiating,
    /// 'BottomInitiating' from (id : 'ParticipantBandKind-bottom_initiating', name : 'bottom_initiating')
    BottomInitiating,
    /// 'TopNonInitiating' from (id : 'ParticipantBandKind-top_non_initiating', name : 'top_non_initiating')
    TopNonInitiating,
    /// 'MiddleNonInitiating' from (id : 'ParticipantBandKind-middle_non_initiating', name : 'middle_non_initiating')
    MiddleNonInitiating,
    /// 'BottomNonInitiating' from (id : 'ParticipantBandKind-bottom_non_initiating', name : 'bottom_non_initiating')
    BottomNonInitiating,
}

/// Conversion of MessageVisibleKind (Enumeration : MessageVisibleKind)
pub enum MessageVisibleKind {
    /// 'Initiating' from (id : 'MessageVisibleKind-initiating', name : 'initiating')
    Initiating,
    /// 'NonInitiating' from (id : 'MessageVisibleKind-non_initiating', name : 'non_initiating')
    NonInitiating,
}
