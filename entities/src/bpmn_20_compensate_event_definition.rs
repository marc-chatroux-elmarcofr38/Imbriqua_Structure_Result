//! bpmn_20_class_compensate_event_definition

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_compensate_event_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : EventDefinition
    pub super_event_definition: i64,
    /// COMPLEX FIELD : BPMN20-CompensateEventDefinition-activityRef
    pub activity_ref: Option<i64>,
    /// SIMPLE FIELD : BPMN20-CompensateEventDefinition-waitForCompletion
    pub wait_for_completion: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CompensateEventDefinition need ONE EventDefinition
    #[sea_orm(
        belongs_to = "super::bpmn_20_event_definition::Entity",
        from = "Column::SuperEventDefinition",
        to = "super::bpmn_20_event_definition::Column::Id",
        on_delete = "Cascade"
    )]
    EventDefinition,
}

// SUPER : ONE CompensateEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventDefinition.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "CompensateEventDefinition" (bpmn_20_class_compensate_event_definition)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __wait_for_completion__ (xmi_id : "BPMN20-CompensateEventDefinition-waitForCompletion")
    ///   * type : __std::primitive::bool__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Activity__ (__ActivityModel__) from A_activityRef_compensateEventDefinition
    ///   * one-to-many link : (0-1) __CompensateEventDefinition__ need (0-inf) __Activity__)
    ///   * callable using find_with_related(__ActivityModel__) from __CompensateEventDefinition__
    /// 
    /// ## Direct Super :
    /// * __EventDefinition__ (__EventDefinitionModel__)
    ///   * one-to-one link : one __CompensateEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __CompensateEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "CompensateEventDefinition" (bpmn_20_class_compensate_event_definition)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __wait_for_completion__ (xmi_id : "BPMN20-CompensateEventDefinition-waitForCompletion")
  * type : __std::primitive::bool__


## Relation : One To Many :
* __Activity__ (__ActivityModel__) from A_activityRef_compensateEventDefinition
  * one-to-many link : (0-1) __CompensateEventDefinition__ need (0-inf) __Activity__)
  * callable using find_with_related(__ActivityModel__) from __CompensateEventDefinition__

## Direct Super :
* __EventDefinition__ (__EventDefinitionModel__)
  * one-to-one link : one __CompensateEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __CompensateEventDefinition__
  * saved in __super_event_definition__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "CompensateEventDefinition",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "CompensateEventDefinition",
//     is_abstract: false,
//     super_class: [
//         "EventDefinition",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "CompensateEventDefinition-activityRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "CompensateEventDefinition-activityRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "activityRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Activity",
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
//                     "A_activityRef_compensateEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "CompensateEventDefinition-waitForCompletion": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "CompensateEventDefinition-waitForCompletion",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "waitForCompletion",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Weak ref of 'DC-Boolean' (loaded : true)",
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
//     technical_name: "BPMN20.cmof#CompensateEventDefinition",
//     table_name: "bpmn_20_compensate_event_definition",
//     model_name: "CompensateEventDefinition",
//     full_name: "bpmn_20_class_compensate_event_definition",
// }

