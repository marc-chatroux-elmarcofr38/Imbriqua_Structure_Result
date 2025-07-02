//! di_association_a_owned_label_owning_shape

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_ownedLabel_owningShape",
//     name: "A_ownedLabel_owningShape",
//     visibility: Private,
//     member_end: (
//         "LabeledShape-ownedLabel",
//         "A_ownedLabel_owningShape-owningShape",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_ownedLabel_owningShape-owningShape",
//                 name: "owningShape",
//                 visibility: Public,
//                 simple_type: Some(
//                     "LabeledShape",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: true,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: Some(
//                     "DiagramElement-owningElement",
//                 ),
//                 owning_association: "A_ownedLabel_owningShape",
//                 association: Some(
//                     "A_ownedLabel_owningShape",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

