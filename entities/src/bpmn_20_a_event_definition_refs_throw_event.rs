//! bpmn_20_association_a_event_definition_refs_throw_event

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_eventDefinitionRefs_throwEvent",
//     name: "A_eventDefinitionRefs_throwEvent",
//     visibility: Private,
//     member_end: (
//         "ThrowEvent-eventDefinitionRefs",
//         "A_eventDefinitionRefs_throwEvent-throwEvent",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_eventDefinitionRefs_throwEvent-throwEvent",
//                 name: "throwEvent",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ThrowEvent",
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
//                 owning_association: "A_eventDefinitionRefs_throwEvent",
//                 association: Some(
//                     "A_eventDefinitionRefs_throwEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

