//! bpmn_20_association_a_outer_message_flow_ref_message_flow_association

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_outerMessageFlowRef_messageFlowAssociation",
//     name: "A_outerMessageFlowRef_messageFlowAssociation",
//     visibility: Private,
//     member_end: (
//         "MessageFlowAssociation-outerMessageFlowRef",
//         "A_outerMessageFlowRef_messageFlowAssociation-messageFlowAssociation",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_outerMessageFlowRef_messageFlowAssociation-messageFlowAssociation",
//                 name: "messageFlowAssociation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "MessageFlowAssociation",
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
//                 owning_association: "A_outerMessageFlowRef_messageFlowAssociation",
//                 association: Some(
//                     "A_outerMessageFlowRef_messageFlowAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

