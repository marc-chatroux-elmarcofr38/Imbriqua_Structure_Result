//! bpmn_20_association_a_conversation_associations_converstaion_associations

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_conversationAssociations_converstaionAssociations",
//     name: "A_conversationAssociations_converstaionAssociations",
//     visibility: Private,
//     member_end: (
//         "Collaboration-conversationAssociations",
//         "A_conversationAssociations_converstaionAssociations-converstaionAssociations",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_conversationAssociations_converstaionAssociations-converstaionAssociations",
//                 name: "converstaionAssociations",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Collaboration",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
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
//                 owning_association: "A_conversationAssociations_converstaionAssociations",
//                 association: Some(
//                     "A_conversationAssociations_converstaionAssociations",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

