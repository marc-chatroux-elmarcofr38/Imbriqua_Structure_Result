//! bpmn_20_association_a_flow_elements_container

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_flowElements_container",
//     name: "A_flowElements_container",
//     visibility: Private,
//     member_end: (
//         "FlowElementsContainer-flowElements",
//         "A_flowElements_container-container",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_flowElements_container-container",
//                 name: "container",
//                 visibility: Public,
//                 simple_type: Some(
//                     "FlowElementsContainer",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
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
//                 owning_association: "A_flowElements_container",
//                 association: Some(
//                     "A_flowElements_container",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

