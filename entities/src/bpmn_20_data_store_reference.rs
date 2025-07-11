//! bpmn_20_class_data_store_reference

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_store_reference")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : FlowElement
    pub super_flow_element: i64,
    /// SUPER FIELD : ItemAwareElement
    pub super_item_aware_element: i64,
    /// COMPLEX FIELD : DataStoreReference-dataStoreRef
    pub data_store_ref: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE DataStoreReference need ONE FlowElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_element::Entity",
        from = "Column::SuperFlowElement",
        to = "super::bpmn_20_flow_element::Column::Id",
        on_delete = "Cascade"
    )]
    FlowElement,
    // SUPER : ONE DataStoreReference need ONE ItemAwareElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_item_aware_element::Entity",
        from = "Column::SuperItemAwareElement",
        to = "super::bpmn_20_item_aware_element::Column::Id",
        on_delete = "Cascade"
    )]
    ItemAwareElement,
}

// SUPER : ONE DataStoreReference need ONE FlowElement
impl Related<super::bpmn_20_flow_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElement.def()
    }
}

// SUPER : ONE DataStoreReference need ONE ItemAwareElement
impl Related<super::bpmn_20_item_aware_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemAwareElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "DataStoreReference" (bpmn_20_class_data_store_reference)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __FlowElement__ (__FlowElementModel__)
    ///   * one-to-one link : one __DataStoreReference__ need one __FlowElement__)
    ///   * callable using find_also_related(__FlowElementModel__) from __DataStoreReference__
    ///   * saved in __super_flow_element__ field as foreing key
    /// * __ItemAwareElement__ (__ItemAwareElementModel__)
    ///   * one-to-one link : one __DataStoreReference__ need one __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __DataStoreReference__
    ///   * saved in __super_item_aware_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "DataStoreReference" (bpmn_20_class_data_store_reference)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Direct Super :
* __FlowElement__ (__FlowElementModel__)
  * one-to-one link : one __DataStoreReference__ need one __FlowElement__)
  * callable using find_also_related(__FlowElementModel__) from __DataStoreReference__
  * saved in __super_flow_element__ field as foreing key
* __ItemAwareElement__ (__ItemAwareElementModel__)
  * one-to-one link : one __DataStoreReference__ need one __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __DataStoreReference__
  * saved in __super_item_aware_element__ field as foreing key


"#
    }
}

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

