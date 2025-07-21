//! bpmn_20_class_data_state

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_state")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SIMPLE FIELD : BPMN20-DataState-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE DataState need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE DataState need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "DataState" (bpmn_20_class_data_state)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "BPMN20-DataState-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __DataState__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __DataState__
    ///   * saved in __super_base_element__ field as foreing key
    /// ## Reverse One To One :
    /// * __ItemAwareElement__ (__ItemAwareElementModel__) from A_dataState_itemAwareElement
    ///   * one-to-one link : (0-1) __ItemAwareElement__ need (1-1) __DataState__)
    ///   * callable using find_also_related(__DataStateModel__) from __ItemAwareElement__
    ///   * saved in __data_state__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "DataState" (bpmn_20_class_data_state)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "BPMN20-DataState-name")
  * type : __std::string::String__



## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __DataState__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __DataState__
  * saved in __super_base_element__ field as foreing key
## Reverse One To One :
* __ItemAwareElement__ (__ItemAwareElementModel__) from A_dataState_itemAwareElement
  * one-to-one link : (0-1) __ItemAwareElement__ need (1-1) __DataState__)
  * callable using find_also_related(__DataStateModel__) from __ItemAwareElement__
  * saved in __data_state__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "DataState",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "DataState",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "DataState-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "DataState-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Weak ref of 'DC-String' (loaded : true)",
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
//     technical_name: "BPMN20.cmof#DataState",
//     table_name: "bpmn_20_data_state",
//     model_name: "DataState",
//     full_name: "bpmn_20_class_data_state",
// }

