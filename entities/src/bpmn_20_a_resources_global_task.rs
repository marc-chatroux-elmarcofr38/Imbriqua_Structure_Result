//! bpmn_20_association_a_resources_global_task

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_resources_globalTask",
//     name: "A_resources_globalTask",
//     visibility: Private,
//     member_end: (
//         "GlobalTask-resources",
//         "A_resources_globalTask-globalTask",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_resources_globalTask-globalTask",
//                 name: "globalTask",
//                 visibility: Public,
//                 simple_type: Some(
//                     "GlobalTask",
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
//                 owning_association: "A_resources_globalTask",
//                 association: Some(
//                     "A_resources_globalTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

