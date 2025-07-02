//! bpmn_20_association_a_data_store_ref_data_store_reference

use crate::*;
use sea_orm::entity::prelude::*;


// RAW :
// CMOFAssociation {
//     xmi_id: "A_dataStoreRef_dataStoreReference",
//     name: "A_dataStoreRef_dataStoreReference",
//     visibility: Private,
//     member_end: (
//         "DataStoreReference-dataStoreRef",
//         "A_dataStoreRef_dataStoreReference-dataStoreReference",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_dataStoreRef_dataStoreReference-dataStoreReference",
//                 name: "dataStoreReference",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataStoreReference",
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
//                 owning_association: "A_dataStoreRef_dataStoreReference",
//                 association: Some(
//                     "A_dataStoreRef_dataStoreReference",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

