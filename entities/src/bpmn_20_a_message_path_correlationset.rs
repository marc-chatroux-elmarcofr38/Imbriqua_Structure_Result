//! bpmn_20_association_a_message_path_correlationset

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_messagePath_correlationset",
//     name: "A_messagePath_correlationset",
//     visibility: Private,
//     member_end: (
//         "CorrelationPropertyRetrievalExpression-messagePath",
//         "A_messagePath_correlationset-correlationset",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_messagePath_correlationset-correlationset",
//                 name: "correlationset",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationPropertyRetrievalExpression",
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
//                 owning_association: "A_messagePath_correlationset",
//                 association: Some(
//                     "A_messagePath_correlationset",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

