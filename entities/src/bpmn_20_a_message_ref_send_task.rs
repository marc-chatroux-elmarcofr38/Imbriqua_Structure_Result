//! bpmn_20_association_a_message_ref_send_task

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_messageRef_sendTask",
//     name: "A_messageRef_sendTask",
//     visibility: Private,
//     member_end: (
//         "SendTask-messageRef",
//         "A_messageRef_sendTask-sendTask",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_messageRef_sendTask-sendTask",
//                 name: "sendTask",
//                 visibility: Public,
//                 simple_type: Some(
//                     "SendTask",
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
//                 owning_association: "A_messageRef_sendTask",
//                 association: Some(
//                     "A_messageRef_sendTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

