//! bpmn_20_association_a_source_ref_message_flow

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_sourceRef_messageFlow",
//     name: "A_sourceRef_messageFlow",
//     visibility: Private,
//     member_end: (
//         "MessageFlow-sourceRef",
//         "A_sourceRef_messageFlow-messageFlow",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_sourceRef_messageFlow-messageFlow",
//                 name: "messageFlow",
//                 visibility: Public,
//                 simple_type: Some(
//                     "MessageFlow",
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
//                 owning_association: "A_sourceRef_messageFlow",
//                 association: Some(
//                     "A_sourceRef_messageFlow",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

