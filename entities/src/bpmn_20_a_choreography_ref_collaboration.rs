//! bpmn_20_association_a_choreography_ref_collaboration

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_choreographyRef_collaboration",
//     name: "A_choreographyRef_collaboration",
//     visibility: Private,
//     member_end: (
//         "Collaboration-choreographyRef",
//         "A_choreographyRef_collaboration-collaboration",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_choreographyRef_collaboration-collaboration",
//                 name: "collaboration",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Collaboration",
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
//                 owning_association: "A_choreographyRef_collaboration",
//                 association: Some(
//                     "A_choreographyRef_collaboration",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

