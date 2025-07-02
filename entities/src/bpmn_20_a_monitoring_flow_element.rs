//! bpmn_20_association_a_monitoring_flow_element

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_monitoring_flowElement",
//     name: "A_monitoring_flowElement",
//     visibility: Private,
//     member_end: (
//         "FlowElement-monitoring",
//         "A_monitoring_flowElement-flowElement",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_monitoring_flowElement-flowElement",
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
//                 owning_association: "A_monitoring_flowElement",
//                 association: Some(
//                     "A_monitoring_flowElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

