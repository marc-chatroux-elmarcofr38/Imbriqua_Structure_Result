//! participant_band_kind
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmndi::*;
use crate::Builder;

/// Conversion of ParticipantBandKind (Enumeration : ParticipantBandKind)
#[derive(Debug, Clone)]
pub enum participant_band_kind {
    /// 'top_initiating' from (id : 'ParticipantBandKind-top_initiating', name : 'top_initiating')
    top_initiating, 
    /// 'middle_initiating' from (id : 'ParticipantBandKind-middle_initiating', name : 'middle_initiating')
    middle_initiating, 
    /// 'bottom_initiating' from (id : 'ParticipantBandKind-bottom_initiating', name : 'bottom_initiating')
    bottom_initiating, 
    /// 'top_non_initiating' from (id : 'ParticipantBandKind-top_non_initiating', name : 'top_non_initiating')
    top_non_initiating, 
    /// 'middle_non_initiating' from (id : 'ParticipantBandKind-middle_non_initiating', name : 'middle_non_initiating')
    middle_non_initiating, 
    /// 'bottom_non_initiating' from (id : 'ParticipantBandKind-bottom_non_initiating', name : 'bottom_non_initiating')
    bottom_non_initiating, 
}
