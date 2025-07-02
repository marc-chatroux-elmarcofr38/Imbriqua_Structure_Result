//! bpmn_20_association_a_loop_condition_standard_loop_characteristics

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_loopCondition_standardLoopCharacteristics",
//     name: "A_loopCondition_standardLoopCharacteristics",
//     visibility: Private,
//     member_end: (
//         "StandardLoopCharacteristics-loopCondition",
//         "A_loopCondition_standardLoopCharacteristics-standardLoopCharacteristics",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_loopCondition_standardLoopCharacteristics-standardLoopCharacteristics",
//                 name: "standardLoopCharacteristics",
//                 visibility: Public,
//                 simple_type: Some(
//                     "StandardLoopCharacteristics",
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
//                 owning_association: "A_loopCondition_standardLoopCharacteristics",
//                 association: Some(
//                     "A_loopCondition_standardLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

