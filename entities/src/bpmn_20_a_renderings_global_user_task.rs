//! bpmn_20_association_a_renderings_global_user_task

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_renderings_globalUserTask",
//     name: "A_renderings_globalUserTask",
//     visibility: Private,
//     member_end: (
//         "GlobalUserTask-renderings",
//         "A_renderings_globalUserTask-globalUserTask",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_renderings_globalUserTask-globalUserTask",
//                 name: "globalUserTask",
//                 visibility: Public,
//                 simple_type: Some(
//                     "GlobalUserTask",
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
//                 owning_association: "A_renderings_globalUserTask",
//                 association: Some(
//                     "A_renderings_globalUserTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

