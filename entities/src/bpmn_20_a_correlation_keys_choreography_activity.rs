//! bpmn_20_association_a_correlation_keys_choreography_activity

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_correlationKeys_choreographyActivity",
//     name: "A_correlationKeys_choreographyActivity",
//     visibility: Private,
//     member_end: (
//         "ChoreographyActivity-correlationKeys",
//         "A_correlationKeys_choreographyActivity-choreographyActivity",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_correlationKeys_choreographyActivity-choreographyActivity",
//                 name: "choreographyActivity",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ChoreographyActivity",
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
//                 owning_association: "A_correlationKeys_choreographyActivity",
//                 association: Some(
//                     "A_correlationKeys_choreographyActivity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

