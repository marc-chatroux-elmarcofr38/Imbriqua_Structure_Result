//! bpmn_20_association_a_message_flow_refs_communication

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_messageFlowRefs_communication",
//     name: "A_messageFlowRefs_communication",
//     visibility: Private,
//     member_end: (
//         "ConversationNode-messageFlowRefs",
//         "A_messageFlowRefs_communication-communication",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_messageFlowRefs_communication-communication",
//                 name: "communication",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ConversationNode",
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
//                 owning_association: "A_messageFlowRefs_communication",
//                 association: Some(
//                     "A_messageFlowRefs_communication",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

