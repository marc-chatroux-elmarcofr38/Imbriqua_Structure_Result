//! bpmn_20_association_a_resource_ref_activity_resource

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_resourceRef_activityResource",
//     name: "A_resourceRef_activityResource",
//     visibility: Private,
//     member_end: (
//         "ResourceRole-resourceRef",
//         "A_resourceRef_activityResource-activityResource",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_resourceRef_activityResource-activityResource",
//                 name: "activityResource",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ResourceRole",
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
//                 owning_association: "A_resourceRef_activityResource",
//                 association: Some(
//                     "A_resourceRef_activityResource",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

