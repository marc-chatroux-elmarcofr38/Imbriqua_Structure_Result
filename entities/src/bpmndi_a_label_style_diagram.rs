//! bpmndi_association_a_label_style_diagram

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_labelStyle_diagram",
//     name: "A_labelStyle_diagram",
//     visibility: Private,
//     member_end: (
//         "BPMNDiagram-labelStyle",
//         "A_labelStyle_diagram-diagram",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_labelStyle_diagram-diagram",
//                 name: "diagram",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BPMNDiagram",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
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
//                 owning_association: "A_labelStyle_diagram",
//                 association: Some(
//                     "A_labelStyle_diagram",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: Some(
//                     Property(
//                         SubsettedProperty {
//                             href: "DI.cmof#A_ownedStyle_owningDiagram-owningDiagram",
//                         },
//                     ),
//                 ),
//             },
//         ),
//     ],
//     is_derived: false,
// }

