//! bpmndi_association_a_label_shape

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_label_shape",
//     name: "A_label_shape",
//     visibility: Private,
//     member_end: (
//         "BPMNShape-label",
//         "A_label_shape-shape",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_label_shape-shape",
//                 name: "shape",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BPMNShape",
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
//                 owning_association: "A_label_shape",
//                 association: Some(
//                     "A_label_shape",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: Some(
//                     Property(
//                         SubsettedProperty {
//                             href: "DI.cmof#A_ownedLabel_owningShape-owningShape",
//                         },
//                     ),
//                 ),
//             },
//         ),
//     ],
//     is_derived: false,
// }

