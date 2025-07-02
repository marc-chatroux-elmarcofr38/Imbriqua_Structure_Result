//! bpmndi_association_a_bpmn_element_plane

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_bpmnElement_plane",
//     name: "A_bpmnElement_plane",
//     visibility: Private,
//     member_end: (
//         "BPMNPlane-bpmnElement",
//         "A_bpmnElement_plane-plane",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_bpmnElement_plane-plane",
//                 name: "plane",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BPMNPlane",
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
//                 owning_association: "A_bpmnElement_plane",
//                 association: Some(
//                     "A_bpmnElement_plane",
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         RedefinedProperty {
//                             href: "DI.cmof#A_modelElement_diagramElement-diagramElement",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

