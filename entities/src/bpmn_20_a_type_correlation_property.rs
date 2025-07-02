//! bpmn_20_association_a_type_correlation_property

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_type_correlationProperty",
//     name: "A_type_correlationProperty",
//     visibility: Private,
//     member_end: (
//         "CorrelationProperty-type",
//         "A_type_correlationProperty-correlationProperty",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_type_correlationProperty-correlationProperty",
//                 name: "correlationProperty",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationProperty",
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
//                 owning_association: "A_type_correlationProperty",
//                 association: Some(
//                     "A_type_correlationProperty",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

