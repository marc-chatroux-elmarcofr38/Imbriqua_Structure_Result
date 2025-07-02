//! bpmn_20_association_a_child_lane_set_parent_lane

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_childLaneSet_parentLane",
//     name: "A_childLaneSet_parentLane",
//     visibility: Private,
//     member_end: (
//         "Lane-childLaneSet",
//         "A_childLaneSet_parentLane-parentLane",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_childLaneSet_parentLane-parentLane",
//                 name: "parentLane",
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
//                 owning_association: "A_childLaneSet_parentLane",
//                 association: Some(
//                     "A_childLaneSet_parentLane",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

