//! bpmn_20_association_a_outer_conversation_node_ref_conversation_association

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_outerConversationNodeRef_conversationAssociation",
//     name: "A_outerConversationNodeRef_conversationAssociation",
//     visibility: Private,
//     member_end: (
//         "ConversationAssociation-outerConversationNodeRef",
//         "A_outerConversationNodeRef_conversationAssociation-conversationAssociation",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_outerConversationNodeRef_conversationAssociation-conversationAssociation",
//                 name: "conversationAssociation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ConversationAssociation",
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
//                 owning_association: "A_outerConversationNodeRef_conversationAssociation",
//                 association: Some(
//                     "A_outerConversationNodeRef_conversationAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

