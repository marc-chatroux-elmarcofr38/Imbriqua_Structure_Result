//! bpmn_20_association_a_target_ref_data_association

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_targetRef_dataAssociation",
//     name: "A_targetRef_dataAssociation",
//     visibility: Private,
//     member_end: (
//         "DataAssociation-targetRef",
//         "A_targetRef_dataAssociation-dataAssociation",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_targetRef_dataAssociation-dataAssociation",
//                 name: "dataAssociation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataAssociation",
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
//                 owning_association: "A_targetRef_dataAssociation",
//                 association: Some(
//                     "A_targetRef_dataAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

