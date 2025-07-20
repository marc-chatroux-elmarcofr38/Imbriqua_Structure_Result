//! bpmn_20_class_signal

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_signal")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : RootElement
    pub super_root_element: i64,
    /// COMPLEX FIELD : BPMN20-Signal-structureRef
    pub structure_ref: Option<i64>,
    /// SIMPLE FIELD : BPMN20-Signal-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Signal need ONE RootElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id",
        on_delete = "Cascade"
    )]
    RootElement,
}

// SUPER : ONE Signal need ONE RootElement
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Signal" (bpmn_20_class_signal)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "BPMN20-Signal-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __ItemDefinition__ (__ItemDefinitionModel__) from A_structureRef_signal
    ///   * one-to-many link : (0-1) __Signal__ need (0-inf) __ItemDefinition__)
    ///   * callable using find_with_related(__ItemDefinitionModel__) from __Signal__
    /// 
    /// ## Direct Super :
    /// * __RootElement__ (__RootElementModel__)
    ///   * one-to-one link : one __Signal__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __Signal__
    ///   * saved in __super_root_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Signal" (bpmn_20_class_signal)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "BPMN20-Signal-name")
  * type : __std::string::String__


## Relation : One To Many :
* __ItemDefinition__ (__ItemDefinitionModel__) from A_structureRef_signal
  * one-to-many link : (0-1) __Signal__ need (0-inf) __ItemDefinition__)
  * callable using find_with_related(__ItemDefinitionModel__) from __Signal__

## Direct Super :
* __RootElement__ (__RootElementModel__)
  * one-to-one link : one __Signal__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __Signal__
  * saved in __super_root_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "Signal",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Signal",
//     is_abstract: false,
//     super_class: [
//         "RootElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Signal-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Signal-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
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
//         "-Signal-structureRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Signal-structureRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "structureRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemDefinition",
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
//                     "A_structureRef_signal",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Signal",
//     table_name: "bpmn_20_signal",
//     model_name: "Signal",
//     full_name: "bpmn_20_class_signal",
// }

