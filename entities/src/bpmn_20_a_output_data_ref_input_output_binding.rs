//! bpmn_20_association_a_output_data_ref_input_output_binding

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_outputDataRef_inputOutputBinding",
//     name: "A_outputDataRef_inputOutputBinding",
//     visibility: Private,
//     member_end: (
//         "InputOutputBinding-outputDataRef",
//         "A_outputDataRef_inputOutputBinding-inputOutputBinding",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_outputDataRef_inputOutputBinding-inputOutputBinding",
//                 name: "inputOutputBinding",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InputOutputBinding",
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
//                 owning_association: "A_outputDataRef_inputOutputBinding",
//                 association: Some(
//                     "A_outputDataRef_inputOutputBinding",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

