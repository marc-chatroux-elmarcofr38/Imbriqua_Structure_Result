//! bpmn_20_association_a_conversation_nodes_sub_conversation

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_conversationNodes_subConversation",
//     name: "A_conversationNodes_subConversation",
//     visibility: Private,
//     member_end: (
//         "SubConversation-conversationNodes",
//         "A_conversationNodes_subConversation-subConversation",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_conversationNodes_subConversation-subConversation",
//                 name: "subConversation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "SubConversation",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "A_conversationNodes_subConversation",
//                 association: Some(
//                     "A_conversationNodes_subConversation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

