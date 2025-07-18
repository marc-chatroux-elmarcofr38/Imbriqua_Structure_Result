//! bpmn_20_class_data_store

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_store")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ItemAwareElement
    pub super_item_aware_element: i64,
    /// SUPER FIELD : RootElement
    pub super_root_element: i64,
    /// SIMPLE FIELD : BPMN20-DataStore-capacity
    pub capacity: std::primitive::u64,
    /// SIMPLE FIELD : BPMN20-DataStore-isUnlimited
    #[sea_orm(default_value = "true")]
    pub is_unlimited: std::primitive::bool,
    /// SIMPLE FIELD : BPMN20-DataStore-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE DataStore need ONE ItemAwareElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_item_aware_element::Entity",
        from = "Column::SuperItemAwareElement",
        to = "super::bpmn_20_item_aware_element::Column::Id",
        on_delete = "Cascade"
    )]
    ItemAwareElement,
    // SUPER : ONE DataStore need ONE RootElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id",
        on_delete = "Cascade"
    )]
    RootElement,
}

// SUPER : ONE DataStore need ONE ItemAwareElement
impl Related<super::bpmn_20_item_aware_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemAwareElement.def()
    }
}

// SUPER : ONE DataStore need ONE RootElement
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "DataStore" (bpmn_20_class_data_store)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __capacity__ (xmi_id : "BPMN20-DataStore-capacity")
    ///   * type : __std::primitive::u64__
    /// * __is_unlimited__ (xmi_id : "BPMN20-DataStore-isUnlimited")
    ///   * type : __std::primitive::bool__
    ///   * default : "true"
    /// * __name__ (xmi_id : "BPMN20-DataStore-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __ItemAwareElement__ (__ItemAwareElementModel__)
    ///   * one-to-one link : one __DataStore__ need one __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __DataStore__
    ///   * saved in __super_item_aware_element__ field as foreing key
    /// * __RootElement__ (__RootElementModel__)
    ///   * one-to-one link : one __DataStore__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __DataStore__
    ///   * saved in __super_root_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "DataStore" (bpmn_20_class_data_store)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __capacity__ (xmi_id : "BPMN20-DataStore-capacity")
  * type : __std::primitive::u64__
* __is_unlimited__ (xmi_id : "BPMN20-DataStore-isUnlimited")
  * type : __std::primitive::bool__
  * default : "true"
* __name__ (xmi_id : "BPMN20-DataStore-name")
  * type : __std::string::String__



## Direct Super :
* __ItemAwareElement__ (__ItemAwareElementModel__)
  * one-to-one link : one __DataStore__ need one __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __DataStore__
  * saved in __super_item_aware_element__ field as foreing key
* __RootElement__ (__RootElementModel__)
  * one-to-one link : one __DataStore__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __DataStore__
  * saved in __super_root_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "DataStore",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "DataStore",
//     is_abstract: false,
//     super_class: [
//         "RootElement",
//         "ItemAwareElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-DataStore-capacity": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "DataStore-capacity",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "capacity",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Integer",
//                         },
//                     ),
//                 ),
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-DataStore-isUnlimited": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "DataStore-isUnlimited",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "isUnlimited",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Boolean",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "true",
//                 ),
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-DataStore-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "DataStore-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
//                         },
//                     ),
//                 ),
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#DataStore",
//     table_name: "bpmn_20_data_store",
//     model_name: "DataStore",
//     full_name: "bpmn_20_class_data_store",
// }

