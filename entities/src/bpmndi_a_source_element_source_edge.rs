//! bpmndi_association_a_source_element_source_edge

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_sourceElement_sourceEdge",
//     name: "A_sourceElement_sourceEdge",
//     visibility: Private,
//     member_end: (
//         "BPMNEdge-sourceElement",
//         "A_sourceElement_sourceEdge-sourceEdge",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_sourceElement_sourceEdge-sourceEdge",
//                 name: "sourceEdge",
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
//                 owning_association: "A_sourceElement_sourceEdge",
//                 association: Some(
//                     "A_sourceElement_sourceEdge",
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         RedefinedProperty {
//                             href: "DI.cmof#A_source_sourceEdge-sourceEdge",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

