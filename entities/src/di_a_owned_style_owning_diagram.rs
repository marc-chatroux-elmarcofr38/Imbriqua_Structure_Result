//! di_association_a_owned_style_owning_diagram

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_ownedStyle_owningDiagram",
//     name: "A_ownedStyle_owningDiagram",
//     visibility: Private,
//     member_end: (
//         "Diagram-ownedStyle",
//         "A_ownedStyle_owningDiagram-owningDiagram",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_ownedStyle_owningDiagram-owningDiagram",
//                 name: "owningDiagram",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Diagram",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
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
//                 subsetted_property: None,
//                 owning_association: "A_ownedStyle_owningDiagram",
//                 association: Some(
//                     "A_ownedStyle_owningDiagram",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

