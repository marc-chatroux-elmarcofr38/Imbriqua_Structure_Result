//! bpmn_20_association_a_parameter_ref_resource_parameter_binding

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_parameterRef_resourceParameterBinding",
//     name: "A_parameterRef_resourceParameterBinding",
//     visibility: Private,
//     member_end: (
//         "ResourceParameterBinding-parameterRef",
//         "A_parameterRef_resourceParameterBinding-resourceParameterBinding",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_parameterRef_resourceParameterBinding-resourceParameterBinding",
//                 name: "resourceParameterBinding",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ResourceParameterBinding",
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
//                 owning_association: "A_parameterRef_resourceParameterBinding",
//                 association: Some(
//                     "A_parameterRef_resourceParameterBinding",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

