//! bpmndi_association_a_label_edge

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_label_edge",
//     name: "A_label_edge",
//     visibility: Private,
//     member_end: (
//         "BPMNEdge-label",
//         "A_label_edge-edge",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_label_edge-edge",
//                 name: "edge",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BPMNEdge",
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
//                 owning_association: "A_label_edge",
//                 association: Some(
//                     "A_label_edge",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: Some(
//                     Property(
//                         SubsettedProperty {
//                             href: "DI.cmof#A_ownedLabel_owningEdge-owningEdge",
//                         },
//                     ),
//                 ),
//             },
//         ),
//     ],
//     is_derived: false,
// }

