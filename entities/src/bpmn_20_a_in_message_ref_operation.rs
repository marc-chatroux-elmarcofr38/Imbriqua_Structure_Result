//! bpmn_20_association_a_in_message_ref_operation

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_inMessageRef_operation",
//     name: "A_inMessageRef_operation",
//     visibility: Private,
//     member_end: (
//         "Operation-inMessageRef",
//         "A_inMessageRef_operation-operation",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_inMessageRef_operation-operation",
//                 name: "operation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Operation",
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
//                 owning_association: "A_inMessageRef_operation",
//                 association: Some(
//                     "A_inMessageRef_operation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

