//! di_association_a_model_element_diagram_element

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_modelElement_diagramElement",
//     name: "A_modelElement_diagramElement",
//     visibility: Private,
//     member_end: (
//         "DiagramElement-modelElement",
//         "A_modelElement_diagramElement-diagramElement",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_modelElement_diagramElement-diagramElement",
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
//                 owning_association: "A_modelElement_diagramElement",
//                 association: Some(
//                     "A_modelElement_diagramElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

