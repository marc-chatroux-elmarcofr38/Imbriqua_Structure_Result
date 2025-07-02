//! bpmn_20_association_a_completion_condition_ad_hoc_sub_process

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_completionCondition_adHocSubProcess",
//     name: "A_completionCondition_adHocSubProcess",
//     visibility: Private,
//     member_end: (
//         "AdHocSubProcess-completionCondition",
//         "A_completionCondition_adHocSubProcess-adHocSubProcess",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_completionCondition_adHocSubProcess-adHocSubProcess",
//                 name: "adHocSubProcess",
//                 visibility: Public,
//                 simple_type: Some(
//                     "AdHocSubProcess",
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
//                 owning_association: "A_completionCondition_adHocSubProcess",
//                 association: Some(
//                     "A_completionCondition_adHocSubProcess",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

