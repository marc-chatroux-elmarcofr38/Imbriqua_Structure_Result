//! participant_band_kind
#![allow(unused_imports)]

use crate::bpmndi::*;
use crate::Builder;

/// Conversion of ParticipantBandKind (Enumeration : ParticipantBandKind)
#[derive(Debug, Clone)]
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
