//! bpmn_20_association_a_escalation_ref_escalation_event_definition

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_escalationRef_escalationEventDefinition",
//     name: "A_escalationRef_escalationEventDefinition",
//     visibility: Private,
//     member_end: (
//         "EscalationEventDefinition-escalationRef",
//         "A_escalationRef_escalationEventDefinition-escalationEventDefinition",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_escalationRef_escalationEventDefinition-escalationEventDefinition",
//                 name: "escalationEventDefinition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "EscalationEventDefinition",
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
//                 owning_association: "A_escalationRef_escalationEventDefinition",
//                 association: Some(
//                     "A_escalationRef_escalationEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

