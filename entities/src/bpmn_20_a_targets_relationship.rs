//! bpmn_20_association_a_targets_relationship

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_targets_relationship",
//     name: "A_targets_relationship",
//     visibility: Private,
//     member_end: (
//         "Relationship-targets",
//         "A_targets_relationship-relationship",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_targets_relationship-relationship",
//                 name: "relationship",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Relationship",
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
//                 owning_association: "A_targets_relationship",
//                 association: Some(
//                     "A_targets_relationship",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

