//! bpmn_20_association_a_error_ref_error_event_definition

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_errorRef_errorEventDefinition",
//     name: "A_errorRef_errorEventDefinition",
//     visibility: Private,
//     member_end: (
//         "ErrorEventDefinition-errorRef",
//         "A_errorRef_errorEventDefinition-errorEventDefinition",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_errorRef_errorEventDefinition-errorEventDefinition",
//                 name: "errorEventDefinition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ErrorEventDefinition",
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
//                 owning_association: "A_errorRef_errorEventDefinition",
//                 association: Some(
//                     "A_errorRef_errorEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

