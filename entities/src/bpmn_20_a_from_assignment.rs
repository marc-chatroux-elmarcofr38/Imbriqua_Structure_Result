//! bpmn_20_association_a_from_assignment

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_from_assignment",
//     name: "A_from_assignment",
//     visibility: Private,
//     member_end: (
//         "Assignment-from",
//         "A_from_assignment-assignment",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_from_assignment-assignment",
//                 name: "assignment",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Assignment",
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
//                 owning_association: "A_from_assignment",
//                 association: Some(
//                     "A_from_assignment",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

