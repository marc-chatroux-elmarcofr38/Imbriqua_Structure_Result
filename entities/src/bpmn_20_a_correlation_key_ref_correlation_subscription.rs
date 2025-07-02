//! bpmn_20_association_a_correlation_key_ref_correlation_subscription

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_correlationKeyRef_correlationSubscription",
//     name: "A_correlationKeyRef_correlationSubscription",
//     visibility: Private,
//     member_end: (
//         "CorrelationSubscription-correlationKeyRef",
//         "A_correlationKeyRef_correlationSubscription-correlationSubscription",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_correlationKeyRef_correlationSubscription-correlationSubscription",
//                 name: "correlationSubscription",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationSubscription",
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
//                 owning_association: "A_correlationKeyRef_correlationSubscription",
//                 association: Some(
//                     "A_correlationKeyRef_correlationSubscription",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

