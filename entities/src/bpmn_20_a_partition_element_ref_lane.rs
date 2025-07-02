//! bpmn_20_association_a_partition_element_ref_lane

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_partitionElementRef_lane",
//     name: "A_partitionElementRef_lane",
//     visibility: Private,
//     member_end: (
//         "Lane-partitionElementRef",
//         "A_partitionElementRef_lane-lane",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_partitionElementRef_lane-lane",
//                 name: "lane",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Lane",
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
//                 owning_association: "A_partitionElementRef_lane",
//                 association: Some(
//                     "A_partitionElementRef_lane",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

