//! bpmn_20_class_conditional_event_definition

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_conditional_event_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : EventDefinition
    pub super_event_definition: i64,
    /// COMPLEX FIELD : ConditionalEventDefinition-condition
    pub condition: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ConditionalEventDefinition need ONE EventDefinition
    #[sea_orm(
        belongs_to = "super::bpmn_20_event_definition::Entity",
        from = "Column::SuperEventDefinition",
        to = "super::bpmn_20_event_definition::Column::Id",
        on_delete = "Cascade"
    )]
    EventDefinition,
}

// SUPER : ONE ConditionalEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventDefinition.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ConditionalEventDefinition" (bpmn_20_class_conditional_event_definition)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __Expression__ (__ExpressionModel__) from A_condition_conditionalEventDefinition
    ///   * one-to-one link : (1-1) __ConditionalEventDefinition__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __ConditionalEventDefinition__
    ///   * saved in __condition__ field as foreing key
    /// 
    /// 
    /// ## Direct Super :
    /// * __EventDefinition__ (__EventDefinitionModel__)
    ///   * one-to-one link : one __ConditionalEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __ConditionalEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ConditionalEventDefinition" (bpmn_20_class_conditional_event_definition)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __Expression__ (__ExpressionModel__) from A_condition_conditionalEventDefinition
  * one-to-one link : (1-1) __ConditionalEventDefinition__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __ConditionalEventDefinition__
  * saved in __condition__ field as foreing key


## Direct Super :
* __EventDefinition__ (__EventDefinitionModel__)
  * one-to-one link : one __ConditionalEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __ConditionalEventDefinition__
  * saved in __super_event_definition__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ConditionalEventDefinition",
//     name: "ConditionalEventDefinition",
//     is_abstract: false,
//     super_class: [
//         "EventDefinition",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "ConditionalEventDefinition-condition": Property(
//             CMOFProperty {
//                 xmi_id: "ConditionalEventDefinition-condition",
//                 name: "condition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Expression",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_condition_conditionalEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ConditionalEventDefinition",
//     table_name: "bpmn_20_conditional_event_definition",
//     model_name: "ConditionalEventDefinition",
//     full_name: "bpmn_20_class_conditional_event_definition",
// }

