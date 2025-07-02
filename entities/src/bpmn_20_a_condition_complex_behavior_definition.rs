//! bpmn_20_association_a_condition_complex_behavior_definition

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_condition_complexBehaviorDefinition",
//     name: "A_condition_complexBehaviorDefinition",
//     visibility: Private,
//     member_end: (
//         "ComplexBehaviorDefinition-condition",
//         "A_condition_complexBehaviorDefinition-complexBehaviorDefinition",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_condition_complexBehaviorDefinition-complexBehaviorDefinition",
//                 name: "complexBehaviorDefinition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ComplexBehaviorDefinition",
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
//                 owning_association: "A_condition_complexBehaviorDefinition",
//                 association: Some(
//                     "A_condition_complexBehaviorDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

