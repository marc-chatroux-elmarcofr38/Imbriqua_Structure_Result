//! bpmn_20_association_a_input_sets_input_output_specification

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_inputSets_inputOutputSpecification",
//     name: "A_inputSets_inputOutputSpecification",
//     visibility: Private,
//     member_end: (
//         "InputOutputSpecification-inputSets",
//         "A_inputSets_inputOutputSpecification-inputOutputSpecification",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_inputSets_inputOutputSpecification-inputOutputSpecification",
//                 name: "inputOutputSpecification",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InputOutputSpecification",
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
//                 owning_association: "A_inputSets_inputOutputSpecification",
//                 association: Some(
//                     "A_inputSets_inputOutputSpecification",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

