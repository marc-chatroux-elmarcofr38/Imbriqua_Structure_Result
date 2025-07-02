//! bpmndi_association_a_target_element_target_edge

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_targetElement_targetEdge",
//     name: "A_targetElement_targetEdge",
//     visibility: Private,
//     member_end: (
//         "BPMNEdge-targetElement",
//         "A_targetElement_targetEdge-targetEdge",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_targetElement_targetEdge-targetEdge",
//                 name: "targetEdge",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BPMNEdge",
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
//                 owning_association: "A_targetElement_targetEdge",
//                 association: Some(
//                     "A_targetElement_targetEdge",
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         RedefinedProperty {
//                             href: "DI.cmof#A_target_targetEdge-targetEdge",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

