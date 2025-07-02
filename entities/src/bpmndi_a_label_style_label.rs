//! bpmndi_association_a_label_style_label

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_labelStyle_label",
//     name: "A_labelStyle_label",
//     visibility: Private,
//     member_end: (
//         "BPMNLabel-labelStyle",
//         "A_labelStyle_label-label",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_labelStyle_label-label",
//                 name: "label",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BPMNLabel",
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
//                 owning_association: "A_labelStyle_label",
//                 association: Some(
//                     "A_labelStyle_label",
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         RedefinedProperty {
//                             href: "DI.cmof#A_style_diagramElement-diagramElement",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

