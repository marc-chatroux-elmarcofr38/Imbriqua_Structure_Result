//! bpmn_20_association_a_correlation_property_ref_correlation_property_binding

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_correlationPropertyRef_correlationPropertyBinding",
//     name: "A_correlationPropertyRef_correlationPropertyBinding",
//     visibility: Private,
//     member_end: (
//         "CorrelationPropertyBinding-correlationPropertyRef",
//         "A_correlationPropertyRef_correlationPropertyBinding-correlationPropertyBinding",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_correlationPropertyRef_correlationPropertyBinding-correlationPropertyBinding",
//                 name: "correlationPropertyBinding",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationPropertyBinding",
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
//                 owning_association: "A_correlationPropertyRef_correlationPropertyBinding",
//                 association: Some(
//                     "A_correlationPropertyRef_correlationPropertyBinding",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

