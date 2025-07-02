//! bpmn_20_association_a_signal_ref_signal_event_definition

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_signalRef_signalEventDefinition",
//     name: "A_signalRef_signalEventDefinition",
//     visibility: Private,
//     member_end: (
//         "SignalEventDefinition-signalRef",
//         "A_signalRef_signalEventDefinition-signalEventDefinition",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_signalRef_signalEventDefinition-signalEventDefinition",
//                 name: "signalEventDefinition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "SignalEventDefinition",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "A_signalRef_signalEventDefinition",
//                 association: Some(
//                     "A_signalRef_signalEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

