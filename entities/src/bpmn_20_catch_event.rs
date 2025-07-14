//! bpmn_20_class_catch_event

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_catch_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Event
    pub super_event: i64,
    /// COMPLEX FIELD : CatchEvent-outputSet
    pub output_set: Option<i64>,
    /// SIMPLE FIELD : CatchEvent-parallelMultiple
    pub parallel_multiple: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CatchEvent need ONE Event
    #[sea_orm(
        belongs_to = "super::bpmn_20_event::Entity",
        from = "Column::SuperEvent",
        to = "super::bpmn_20_event::Column::Id",
        on_delete = "Cascade"
    )]
    Event,
    // SUPER : ONE BoundaryEvent need ONE CatchEvent
    #[sea_orm(has_one = "super::bpmn_20_boundary_event::Entity")]
    BoundaryEvent,
    // SUPER : ONE IntermediateCatchEvent need ONE CatchEvent
    #[sea_orm(has_one = "super::bpmn_20_intermediate_catch_event::Entity")]
    IntermediateCatchEvent,
    // SUPER : ONE StartEvent need ONE CatchEvent
    #[sea_orm(has_one = "super::bpmn_20_start_event::Entity")]
    StartEvent,
}

// SUPER : ONE CatchEvent need ONE Event
impl Related<super::bpmn_20_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

// SUPER : ONE BoundaryEvent need ONE CatchEvent
impl Related<super::bpmn_20_boundary_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BoundaryEvent.def()
    }
}

// SUPER : ONE IntermediateCatchEvent need ONE CatchEvent
impl Related<super::bpmn_20_intermediate_catch_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IntermediateCatchEvent.def()
    }
}

// SUPER : ONE StartEvent need ONE CatchEvent
impl Related<super::bpmn_20_start_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StartEvent.def()
    }
}

// ManyToMany : with EventDefinition using A_eventDefinitionRefs_catchEvent
impl Related<super::bpmn_20_a_event_definition_refs_catch_event::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_event_definition_refs_catch_event::Relation::EventDefinition.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_event_definition_refs_catch_event::Relation::CatchEvent
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "CatchEvent" (bpmn_20_class_catch_event)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __parallel_multiple__ (xmi_id : "CatchEvent-parallelMultiple")
    ///   * type : __std::primitive::bool__
    /// 
    /// ## Direct One To One :
    /// * __OutputSet__ (__OutputSetModel__) from A_outputSet_catchEvent
    ///   * one-to-one link : (0-1) __CatchEvent__ need (0-1) __OutputSet__)
    ///   * callable using find_also_related(__OutputSetModel__) from __CatchEvent__
    ///   * saved in __output_set__ field as foreing key
    /// 
    /// 
    /// ## Direct Super :
    /// * __Event__ (__EventModel__)
    ///   * one-to-one link : one __CatchEvent__ need one __Event__)
    ///   * callable using find_also_related(__EventModel__) from __CatchEvent__
    ///   * saved in __super_event__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __BoundaryEvent__ (__BoundaryEventModel__)
    ///   * one-to-one link (reverse) : one __BoundaryEvent__ need one __CatchEvent__)
    ///   * callable using find_also_related(__CatchEventModel__) from __BoundaryEvent__
    ///   * saved in __super_catch_event__ field as foreing key in __BoundaryEventModel__
    /// * __IntermediateCatchEvent__ (__IntermediateCatchEventModel__)
    ///   * one-to-one link (reverse) : one __IntermediateCatchEvent__ need one __CatchEvent__)
    ///   * callable using find_also_related(__CatchEventModel__) from __IntermediateCatchEvent__
    ///   * saved in __super_catch_event__ field as foreing key in __IntermediateCatchEventModel__
    /// * __StartEvent__ (__StartEventModel__)
    ///   * one-to-one link (reverse) : one __StartEvent__ need one __CatchEvent__)
    ///   * callable using find_also_related(__CatchEventModel__) from __StartEvent__
    ///   * saved in __super_catch_event__ field as foreing key in __StartEventModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "CatchEvent" (bpmn_20_class_catch_event)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __parallel_multiple__ (xmi_id : "CatchEvent-parallelMultiple")
  * type : __std::primitive::bool__

## Direct One To One :
* __OutputSet__ (__OutputSetModel__) from A_outputSet_catchEvent
  * one-to-one link : (0-1) __CatchEvent__ need (0-1) __OutputSet__)
  * callable using find_also_related(__OutputSetModel__) from __CatchEvent__
  * saved in __output_set__ field as foreing key


## Direct Super :
* __Event__ (__EventModel__)
  * one-to-one link : one __CatchEvent__ need one __Event__)
  * callable using find_also_related(__EventModel__) from __CatchEvent__
  * saved in __super_event__ field as foreing key

## Reverse Super :
* __BoundaryEvent__ (__BoundaryEventModel__)
  * one-to-one link (reverse) : one __BoundaryEvent__ need one __CatchEvent__)
  * callable using find_also_related(__CatchEventModel__) from __BoundaryEvent__
  * saved in __super_catch_event__ field as foreing key in __BoundaryEventModel__
* __IntermediateCatchEvent__ (__IntermediateCatchEventModel__)
  * one-to-one link (reverse) : one __IntermediateCatchEvent__ need one __CatchEvent__)
  * callable using find_also_related(__CatchEventModel__) from __IntermediateCatchEvent__
  * saved in __super_catch_event__ field as foreing key in __IntermediateCatchEventModel__
* __StartEvent__ (__StartEventModel__)
  * one-to-one link (reverse) : one __StartEvent__ need one __CatchEvent__)
  * callable using find_also_related(__CatchEventModel__) from __StartEvent__
  * saved in __super_catch_event__ field as foreing key in __StartEventModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "CatchEvent",
//     name: "CatchEvent",
//     is_abstract: true,
//     super_class: [
//         "Event",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "CatchEvent-parallelMultiple",
//                 name: "parallelMultiple",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "CatchEvent-outputSet",
//                 name: "outputSet",
//                 visibility: Public,
//                 simple_type: Some(
//                     "OutputSet",
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
//                     "A_outputSet_catchEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "CatchEvent-eventDefinitionRefs",
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
//                     "A_eventDefinitionRefs_catchEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "CatchEvent-dataOutputAssociation",
//                 name: "dataOutputAssociation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataOutputAssociation",
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
//                     "A_dataOutputAssociation_catchEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "CatchEvent-dataOutputs",
//                 name: "dataOutputs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataOutput",
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
//                     "A_dataOutputs_catchEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "CatchEvent-eventDefinitions",
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
//                     "A_eventDefinitions_catchEvent",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

