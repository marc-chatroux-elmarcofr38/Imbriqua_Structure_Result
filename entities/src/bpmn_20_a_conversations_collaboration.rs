//! bpmn_20_association_a_conversations_collaboration

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_conversations_collaboration",
//     name: "A_conversations_collaboration",
//     visibility: Private,
//     member_end: (
//         "Collaboration-conversations",
//         "A_conversations_collaboration-collaboration",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_conversations_collaboration-collaboration",
//                 name: "collaboration",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Collaboration",
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
//                 owning_association: "A_conversations_collaboration",
//                 association: Some(
//                     "A_conversations_collaboration",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

