//! bpmn_20_class_data_store_reference

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_store_reference")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : ItemAwareElement
    pub super_item_aware_element: i32,
    /// SIMPLE FIELD : FlowElement
    pub super_flow_element: i32,
    /// COMPLEX FIELD : DataStoreReference-dataStoreRef
    pub data_store_ref: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_item_aware_element::Entity",
        from = "Column::SuperItemAwareElement",
        to = "super::bpmn_20_item_aware_element::Column::Id"
    )]
    ItemAwareElement,
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_element::Entity",
        from = "Column::SuperFlowElement",
        to = "super::bpmn_20_flow_element::Column::Id"
    )]
    FlowElement,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_item_aware_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemAwareElement.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_flow_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "DataStoreReference",
//     name: "DataStoreReference",
//     is_abstract: false,
//     super_class: [
//         "ItemAwareElement",
//         "FlowElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "DataStoreReference-dataStoreRef",
//                 name: "dataStoreRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataStore",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_dataStoreRef_dataStoreReference",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

