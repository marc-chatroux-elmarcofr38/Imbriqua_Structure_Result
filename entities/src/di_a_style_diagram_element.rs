//! di_association_a_style_diagram_element

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_style_diagramElement",
//     name: "A_style_diagramElement",
//     visibility: Private,
//     member_end: (
//         "DiagramElement-style",
//         "A_style_diagramElement-diagramElement",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_style_diagramElement-diagramElement",
//                 name: "diagramElement",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DiagramElement",
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
//                 owning_association: "A_style_diagramElement",
//                 association: Some(
//                     "A_style_diagramElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

