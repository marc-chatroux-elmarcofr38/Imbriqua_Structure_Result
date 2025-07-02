//! bpmn_20_association_a_data_path_correlation_property_binding

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_dataPath_correlationPropertyBinding",
//     name: "A_dataPath_correlationPropertyBinding",
//     visibility: Private,
//     member_end: (
//         "CorrelationPropertyBinding-dataPath",
//         "A_dataPath_correlationPropertyBinding-correlationPropertyBinding",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_dataPath_correlationPropertyBinding-correlationPropertyBinding",
//                 name: "correlationPropertyBinding",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationPropertyBinding",
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
//                 owning_association: "A_dataPath_correlationPropertyBinding",
//                 association: Some(
//                     "A_dataPath_correlationPropertyBinding",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

