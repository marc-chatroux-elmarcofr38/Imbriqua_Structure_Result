//! bpmn_20_association_a_renderings_usertask

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_renderings_usertask",
//     name: "A_renderings_usertask",
//     visibility: Private,
//     member_end: (
//         "UserTask-renderings",
//         "A_renderings_usertask-usertask",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_renderings_usertask-usertask",
//                 name: "usertask",
//                 visibility: Public,
//                 simple_type: Some(
//                     "UserTask",
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
//                 owning_association: "A_renderings_usertask",
//                 association: Some(
//                     "A_renderings_usertask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

