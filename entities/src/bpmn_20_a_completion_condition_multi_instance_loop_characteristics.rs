//! bpmn_20_association_a_completion_condition_multi_instance_loop_characteristics

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_completionCondition_multiInstanceLoopCharacteristics",
//     name: "A_completionCondition_multiInstanceLoopCharacteristics",
//     visibility: Private,
//     member_end: (
//         "MultiInstanceLoopCharacteristics-completionCondition",
//         "A_completionCondition_multiInstanceLoopCharacteristics-multiInstanceLoopCharacteristics",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_completionCondition_multiInstanceLoopCharacteristics-multiInstanceLoopCharacteristics",
//                 name: "multiInstanceLoopCharacteristics",
//                 visibility: Public,
//                 simple_type: Some(
//                     "MultiInstanceLoopCharacteristics",
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
//                 owning_association: "A_completionCondition_multiInstanceLoopCharacteristics",
//                 association: Some(
//                     "A_completionCondition_multiInstanceLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

