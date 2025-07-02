//! bpmn_20_association_a_structure_ref_signal

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_structureRef_signal",
//     name: "A_structureRef_signal",
//     visibility: Private,
//     member_end: (
//         "Signal-structureRef",
//         "A_structureRef_signal-signal",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_structureRef_signal-signal",
//                 name: "signal",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Signal",
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
//                 owning_association: "A_structureRef_signal",
//                 association: Some(
//                     "A_structureRef_signal",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

