//! bpmn_20_association_a_auditing_flow_element

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_auditing_flowElement",
//     name: "A_auditing_flowElement",
//     visibility: Private,
//     member_end: (
//         "FlowElement-auditing",
//         "A_auditing_flowElement-flowElement",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_auditing_flowElement-flowElement",
//                 name: "flowElement",
//                 visibility: Public,
//                 simple_type: Some(
//                     "FlowElement",
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
//                 owning_association: "A_auditing_flowElement",
//                 association: Some(
//                     "A_auditing_flowElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

