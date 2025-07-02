//! di_association_a_target_target_edge

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_target_targetEdge",
//     name: "A_target_targetEdge",
//     visibility: Private,
//     member_end: (
//         "Edge-target",
//         "A_target_targetEdge-targetEdge",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_target_targetEdge-targetEdge",
//                 name: "targetEdge",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Edge",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: true,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: None,
//                 owning_association: "A_target_targetEdge",
//                 association: Some(
//                     "A_target_targetEdge",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

