//! bpmn_20_association_a_partition_element_lane

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_partitionElement_lane",
//     name: "A_partitionElement_lane",
//     visibility: Private,
//     member_end: (
//         "Lane-partitionElement",
//         "A_partitionElement_lane-lane",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_partitionElement_lane-lane",
//                 name: "lane",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Lane",
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
//                 owning_association: "A_partitionElement_lane",
//                 association: Some(
//                     "A_partitionElement_lane",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

