//! bpmn_20_association_a_correlation_property_binding_correlation_subscription

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_correlationPropertyBinding_correlationSubscription",
//     name: "A_correlationPropertyBinding_correlationSubscription",
//     visibility: Private,
//     member_end: (
//         "CorrelationSubscription-correlationPropertyBinding",
//         "A_correlationPropertyBinding_correlationSubscription-correlationSubscription",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_correlationPropertyBinding_correlationSubscription-correlationSubscription",
//                 name: "correlationSubscription",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationSubscription",
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
//                 owning_association: "A_correlationPropertyBinding_correlationSubscription",
//                 association: Some(
//                     "A_correlationPropertyBinding_correlationSubscription",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

