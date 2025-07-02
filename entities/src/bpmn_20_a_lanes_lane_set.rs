//! bpmn_20_association_a_lanes_lane_set

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_lanes_laneSet",
//     name: "A_lanes_laneSet",
//     visibility: Private,
//     member_end: (
//         "LaneSet-lanes",
//         "A_lanes_laneSet-laneSet",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_lanes_laneSet-laneSet",
//                 name: "laneSet",
//                 visibility: Public,
//                 simple_type: Some(
//                     "LaneSet",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
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
//                 owning_association: "A_lanes_laneSet",
//                 association: Some(
//                     "A_lanes_laneSet",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

