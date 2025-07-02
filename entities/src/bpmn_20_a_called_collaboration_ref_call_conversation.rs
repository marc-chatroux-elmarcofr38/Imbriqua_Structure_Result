//! bpmn_20_association_a_called_collaboration_ref_call_conversation

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_calledCollaborationRef_callConversation",
//     name: "A_calledCollaborationRef_callConversation",
//     visibility: Private,
//     member_end: (
//         "CallConversation-calledCollaborationRef",
//         "A_calledCollaborationRef_callConversation-callConversation",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_calledCollaborationRef_callConversation-callConversation",
//                 name: "callConversation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CallConversation",
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
//                 owning_association: "A_calledCollaborationRef_callConversation",
//                 association: Some(
//                     "A_calledCollaborationRef_callConversation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

