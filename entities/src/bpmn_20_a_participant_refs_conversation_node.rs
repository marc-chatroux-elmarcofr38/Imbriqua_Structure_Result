//! bpmn_20_association_a_participant_refs_conversation_node

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_participantRefs_conversationNode",
//     name: "A_participantRefs_conversationNode",
//     visibility: Private,
//     member_end: (
//         "ConversationNode-participantRefs",
//         "A_participantRefs_conversationNode-conversationNode",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_participantRefs_conversationNode-conversationNode",
//                 name: "conversationNode",
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
//                 owning_association: "A_participantRefs_conversationNode",
//                 association: Some(
//                     "A_participantRefs_conversationNode",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

