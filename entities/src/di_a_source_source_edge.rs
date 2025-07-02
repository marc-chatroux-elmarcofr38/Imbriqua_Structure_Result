//! di_association_a_source_source_edge

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_source_sourceEdge",
//     name: "A_source_sourceEdge",
//     visibility: Private,
//     member_end: (
//         "Edge-source",
//         "A_source_sourceEdge-sourceEdge",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_source_sourceEdge-sourceEdge",
//                 name: "sourceEdge",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Edge",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: true,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: None,
//                 owning_association: "A_source_sourceEdge",
//                 association: Some(
//                     "A_source_sourceEdge",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

