//! bpmn_20_association_a_lane_sets_flow_elements_container

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_laneSets_flowElementsContainer",
//     name: "A_laneSets_flowElementsContainer",
//     visibility: Private,
//     member_end: (
//         "FlowElementsContainer-laneSets",
//         "A_laneSets_flowElementsContainer-flowElementsContainer",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_laneSets_flowElementsContainer-flowElementsContainer",
//                 name: "flowElementsContainer",
//                 visibility: Public,
//                 simple_type: Some(
//                     "FlowElementsContainer",
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
//                 owning_association: "A_laneSets_flowElementsContainer",
//                 association: Some(
//                     "A_laneSets_flowElementsContainer",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

