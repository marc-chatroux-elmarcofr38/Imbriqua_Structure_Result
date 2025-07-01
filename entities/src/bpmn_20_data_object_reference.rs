//! bpmn_20_class_data_object_reference

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_object_reference")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub pk_id: i32,
    /// SIMPLE FIELD : ItemAwareElement
    pub super_item_aware_element: i64,
    /// SIMPLE FIELD : FlowElement
    pub super_flow_element: i64,
    /// COMPLEX FIELD : DataObjectReference-dataObjectRef
    pub data_object_ref: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "DataObjectReference",
//     name: "DataObjectReference",
//     is_abstract: false,
//     super_class: [
//         "ItemAwareElement",
//         "FlowElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "DataObjectReference-dataObjectRef",
//                 name: "dataObjectRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataObject",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_dataObjectRef_dataObject",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

