//! bpmn_20_association_a_called_element_ref_call_activity

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_calledElementRef_callActivity",
//     name: "A_calledElementRef_callActivity",
//     visibility: Private,
//     member_end: (
//         "CallActivity-calledElementRef",
//         "A_calledElementRef_callActivity-callActivity",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_calledElementRef_callActivity-callActivity",
//                 name: "callActivity",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CallActivity",
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
//                 owning_association: "A_calledElementRef_callActivity",
//                 association: Some(
//                     "A_calledElementRef_callActivity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

