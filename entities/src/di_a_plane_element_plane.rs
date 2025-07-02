//! di_association_a_plane_element_plane

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_planeElement_plane",
//     name: "A_planeElement_plane",
//     visibility: Private,
//     member_end: (
//         "Plane-planeElement",
//         "A_planeElement_plane-plane",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_planeElement_plane-plane",
//                 name: "plane",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Plane",
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
//                 subsetted_property: Some(
//                     "DiagramElement-owningElement",
//                 ),
//                 owning_association: "A_planeElement_plane",
//                 association: Some(
//                     "A_planeElement_plane",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

