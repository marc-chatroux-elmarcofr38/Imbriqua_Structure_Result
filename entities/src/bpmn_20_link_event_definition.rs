//! bpmn_20_class_link_event_definition

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_link_event_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : EventDefinition
    pub super_event_definition: i64,
    /// COMPLEX FIELD : BPMN20-LinkEventDefinition-target
    pub target: Option<i64>,
    /// SIMPLE FIELD : BPMN20-LinkEventDefinition-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE LinkEventDefinition need ONE EventDefinition
    #[sea_orm(
        belongs_to = "super::bpmn_20_event_definition::Entity",
        from = "Column::SuperEventDefinition",
        to = "super::bpmn_20_event_definition::Column::Id",
        on_delete = "Cascade"
    )]
    EventDefinition,
}

// SUPER : ONE LinkEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventDefinition.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "LinkEventDefinition" (bpmn_20_class_link_event_definition)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "BPMN20-LinkEventDefinition-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __LinkEventDefinition__ (__LinkEventDefinitionModel__) from A_target_source
    ///   * one-to-many link : (0-1) __LinkEventDefinition__ need (0-inf) __LinkEventDefinition__)
    ///   * callable using find_with_related(__LinkEventDefinitionModel__) from __LinkEventDefinition__
    ///   * named target in BPMN
    /// 
    /// ## Direct Super :
    /// * __EventDefinition__ (__EventDefinitionModel__)
    ///   * one-to-one link : one __LinkEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __LinkEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "LinkEventDefinition" (bpmn_20_class_link_event_definition)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "BPMN20-LinkEventDefinition-name")
  * type : __std::string::String__


## Relation : One To Many :
* __LinkEventDefinition__ (__LinkEventDefinitionModel__) from A_target_source
  * one-to-many link : (0-1) __LinkEventDefinition__ need (0-inf) __LinkEventDefinition__)
  * callable using find_with_related(__LinkEventDefinitionModel__) from __LinkEventDefinition__
  * named target in BPMN

## Direct Super :
* __EventDefinition__ (__EventDefinitionModel__)
  * one-to-one link : one __LinkEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __LinkEventDefinition__
  * saved in __super_event_definition__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-LinkEventDefinition" (loaded : false)",
//     name: "LinkEventDefinition",
//     is_abstract: false,
//     super_class: [
//         "EventDefinition",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-LinkEventDefinition-name": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-LinkEventDefinition-name" (loaded : false)",
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Weak ref of "DC-String" (loaded : false)",
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
//         "-LinkEventDefinition-source": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-LinkEventDefinition-source" (loaded : false)",
//                 name: "source",
//                 visibility: Public,
//                 simple_type: Some(
//                     "LinkEventDefinition",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_target_source",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-LinkEventDefinition-target": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-LinkEventDefinition-target" (loaded : false)",
//                 name: "target",
//                 visibility: Public,
//                 simple_type: Some(
//                     "LinkEventDefinition",
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
//                     "A_target_source",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#LinkEventDefinition",
//     table_name: "bpmn_20_link_event_definition",
//     model_name: "LinkEventDefinition",
//     full_name: "bpmn_20_class_link_event_definition",
// }

