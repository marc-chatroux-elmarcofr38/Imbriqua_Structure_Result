//! bpmn_20_class_data_object

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_object")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : FlowElement
    pub super_flow_element: i64,
    /// SUPER FIELD : ItemAwareElement
    pub super_item_aware_element: i64,
    /// SIMPLE FIELD : DataObject-isCollection
    #[sea_orm(default_value = "false")]
    pub is_collection: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE DataObject need ONE FlowElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_element::Entity",
        from = "Column::SuperFlowElement",
        to = "super::bpmn_20_flow_element::Column::Id",
        on_delete = "Cascade"
    )]
    FlowElement,
    // SUPER : ONE DataObject need ONE ItemAwareElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_item_aware_element::Entity",
        from = "Column::SuperItemAwareElement",
        to = "super::bpmn_20_item_aware_element::Column::Id",
        on_delete = "Cascade"
    )]
    ItemAwareElement,
}

// SUPER : ONE DataObject need ONE FlowElement
impl Related<super::bpmn_20_flow_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElement.def()
    }
}

// SUPER : ONE DataObject need ONE ItemAwareElement
impl Related<super::bpmn_20_item_aware_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemAwareElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "DataObject" (bpmn_20_class_data_object)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __is_collection__ (xmi_id : "DataObject-isCollection")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __FlowElement__ (__FlowElementModel__)
    ///   * one-to-one link : one __DataObject__ need one __FlowElement__)
    ///   * callable using find_also_related(__FlowElementModel__) from __DataObject__
    ///   * saved in __super_flow_element__ field as foreing key
    /// * __ItemAwareElement__ (__ItemAwareElementModel__)
    ///   * one-to-one link : one __DataObject__ need one __ItemAwareElement__)
    ///   * callable using find_also_related(__ItemAwareElementModel__) from __DataObject__
    ///   * saved in __super_item_aware_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "DataObject" (bpmn_20_class_data_object)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __is_collection__ (xmi_id : "DataObject-isCollection")
  * type : __std::primitive::bool__
  * default : "false"



## Direct Super :
* __FlowElement__ (__FlowElementModel__)
  * one-to-one link : one __DataObject__ need one __FlowElement__)
  * callable using find_also_related(__FlowElementModel__) from __DataObject__
  * saved in __super_flow_element__ field as foreing key
* __ItemAwareElement__ (__ItemAwareElementModel__)
  * one-to-one link : one __DataObject__ need one __ItemAwareElement__)
  * callable using find_also_related(__ItemAwareElementModel__) from __DataObject__
  * saved in __super_item_aware_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "DataObject",
//     name: "DataObject",
//     is_abstract: false,
//     super_class: [
//         "FlowElement",
//         "ItemAwareElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "DataObject-isCollection",
//                 name: "isCollection",
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
//                     "false",
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
//     ],
//     owned_rule: [],
// }

