//! bpmn_20_association_a_item_ref_message

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_itemRef_message",
//     name: "A_itemRef_message",
//     visibility: Private,
//     member_end: (
//         "Message-itemRef",
//         "A_itemRef_message-message",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_itemRef_message-message",
//                 name: "message",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Message",
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
//                 owning_association: "A_itemRef_message",
//                 association: Some(
//                     "A_itemRef_message",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

