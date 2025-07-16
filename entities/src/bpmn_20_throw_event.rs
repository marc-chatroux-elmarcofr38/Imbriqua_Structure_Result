//! bpmn_20_class_throw_event

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_throw_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Event
    pub super_event: i64,
    /// COMPLEX FIELD : ThrowEvent-inputSet
    pub input_set: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ThrowEvent need ONE Event
    #[sea_orm(
        belongs_to = "super::bpmn_20_event::Entity",
        from = "Column::SuperEvent",
        to = "super::bpmn_20_event::Column::Id",
        on_delete = "Cascade"
    )]
    Event,
    // SUPER : ONE EndEvent need ONE ThrowEvent
    #[sea_orm(has_one = "super::bpmn_20_end_event::Entity")]
    EndEvent,
    // SUPER : ONE ImplicitThrowEvent need ONE ThrowEvent
    #[sea_orm(has_one = "super::bpmn_20_implicit_throw_event::Entity")]
    ImplicitThrowEvent,
    // SUPER : ONE IntermediateThrowEvent need ONE ThrowEvent
    #[sea_orm(has_one = "super::bpmn_20_intermediate_throw_event::Entity")]
    IntermediateThrowEvent,
}

// SUPER : ONE ThrowEvent need ONE Event
impl Related<super::bpmn_20_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

// SUPER : ONE EndEvent need ONE ThrowEvent
impl Related<super::bpmn_20_end_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EndEvent.def()
    }
}

// SUPER : ONE ImplicitThrowEvent need ONE ThrowEvent
impl Related<super::bpmn_20_implicit_throw_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ImplicitThrowEvent.def()
    }
}

// SUPER : ONE IntermediateThrowEvent need ONE ThrowEvent
impl Related<super::bpmn_20_intermediate_throw_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IntermediateThrowEvent.def()
    }
}

// ManyToMany : with EventDefinition using A_eventDefinitionRefs_throwEvent
impl Related<super::bpmn_20_a_event_definition_refs_throw_event::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_event_definition_refs_throw_event::Relation::EventDefinition.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_event_definition_refs_throw_event::Relation::ThrowEvent
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ThrowEvent" (bpmn_20_class_throw_event)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __InputSet__ (__InputSetModel__) from A_inputSet_throwEvent
    ///   * one-to-one link : (0-1) __ThrowEvent__ need (0-1) __InputSet__)
    ///   * callable using find_also_related(__InputSetModel__) from __ThrowEvent__
    ///   * saved in __input_set__ field as foreing key
    /// 
    /// 
    /// ## Direct Super :
    /// * __Event__ (__EventModel__)
    ///   * one-to-one link : one __ThrowEvent__ need one __Event__)
    ///   * callable using find_also_related(__EventModel__) from __ThrowEvent__
    ///   * saved in __super_event__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __EndEvent__ (__EndEventModel__)
    ///   * one-to-one link (reverse) : one __EndEvent__ need one __ThrowEvent__)
    ///   * callable using find_also_related(__ThrowEventModel__) from __EndEvent__
    ///   * saved in __super_throw_event__ field as foreing key in __EndEventModel__
    /// * __ImplicitThrowEvent__ (__ImplicitThrowEventModel__)
    ///   * one-to-one link (reverse) : one __ImplicitThrowEvent__ need one __ThrowEvent__)
    ///   * callable using find_also_related(__ThrowEventModel__) from __ImplicitThrowEvent__
    ///   * saved in __super_throw_event__ field as foreing key in __ImplicitThrowEventModel__
    /// * __IntermediateThrowEvent__ (__IntermediateThrowEventModel__)
    ///   * one-to-one link (reverse) : one __IntermediateThrowEvent__ need one __ThrowEvent__)
    ///   * callable using find_also_related(__ThrowEventModel__) from __IntermediateThrowEvent__
    ///   * saved in __super_throw_event__ field as foreing key in __IntermediateThrowEventModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ThrowEvent" (bpmn_20_class_throw_event)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __InputSet__ (__InputSetModel__) from A_inputSet_throwEvent
  * one-to-one link : (0-1) __ThrowEvent__ need (0-1) __InputSet__)
  * callable using find_also_related(__InputSetModel__) from __ThrowEvent__
  * saved in __input_set__ field as foreing key


## Direct Super :
* __Event__ (__EventModel__)
  * one-to-one link : one __ThrowEvent__ need one __Event__)
  * callable using find_also_related(__EventModel__) from __ThrowEvent__
  * saved in __super_event__ field as foreing key

## Reverse Super :
* __EndEvent__ (__EndEventModel__)
  * one-to-one link (reverse) : one __EndEvent__ need one __ThrowEvent__)
  * callable using find_also_related(__ThrowEventModel__) from __EndEvent__
  * saved in __super_throw_event__ field as foreing key in __EndEventModel__
* __ImplicitThrowEvent__ (__ImplicitThrowEventModel__)
  * one-to-one link (reverse) : one __ImplicitThrowEvent__ need one __ThrowEvent__)
  * callable using find_also_related(__ThrowEventModel__) from __ImplicitThrowEvent__
  * saved in __super_throw_event__ field as foreing key in __ImplicitThrowEventModel__
* __IntermediateThrowEvent__ (__IntermediateThrowEventModel__)
  * one-to-one link (reverse) : one __IntermediateThrowEvent__ need one __ThrowEvent__)
  * callable using find_also_related(__ThrowEventModel__) from __IntermediateThrowEvent__
  * saved in __super_throw_event__ field as foreing key in __IntermediateThrowEventModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ThrowEvent",
//     name: "ThrowEvent",
//     is_abstract: true,
//     super_class: [
//         "Event",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "ThrowEvent-dataInputAssociation": Property(
//             CMOFProperty {
//                 xmi_id: "ThrowEvent-dataInputAssociation",
//                 name: "dataInputAssociation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataInputAssociation",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_dataInputAssociation_throwEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "ThrowEvent-dataInputs": Property(
//             CMOFProperty {
//                 xmi_id: "ThrowEvent-dataInputs",
//                 name: "dataInputs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataInput",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_dataInputs_throwEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "ThrowEvent-eventDefinitionRefs": Property(
//             CMOFProperty {
//                 xmi_id: "ThrowEvent-eventDefinitionRefs",
//                 name: "eventDefinitionRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "EventDefinition",
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
//                     "A_eventDefinitionRefs_throwEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "ThrowEvent-eventDefinitions": Property(
//             CMOFProperty {
//                 xmi_id: "ThrowEvent-eventDefinitions",
//                 name: "eventDefinitions",
//                 visibility: Public,
//                 simple_type: Some(
//                     "EventDefinition",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_eventDefinitions_throwEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "ThrowEvent-inputSet": Property(
//             CMOFProperty {
//                 xmi_id: "ThrowEvent-inputSet",
//                 name: "inputSet",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InputSet",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
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
//                     "A_inputSet_throwEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ThrowEvent",
//     table_name: "bpmn_20_throw_event",
//     model_name: "ThrowEvent",
//     full_name: "bpmn_20_class_throw_event",
// }

