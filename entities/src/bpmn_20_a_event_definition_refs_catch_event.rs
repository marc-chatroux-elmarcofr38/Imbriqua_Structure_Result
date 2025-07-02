//! bpmn_20_association_a_event_definition_refs_catch_event

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_eventDefinitionRefs_catchEvent",
//     name: "A_eventDefinitionRefs_catchEvent",
//     visibility: Private,
//     member_end: (
//         "CatchEvent-eventDefinitionRefs",
//         "A_eventDefinitionRefs_catchEvent-catchEvent",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_eventDefinitionRefs_catchEvent-catchEvent",
//                 name: "catchEvent",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CatchEvent",
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
//                 owning_association: "A_eventDefinitionRefs_catchEvent",
//                 association: Some(
//                     "A_eventDefinitionRefs_catchEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

