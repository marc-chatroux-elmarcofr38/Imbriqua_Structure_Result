//! bpmn_20_class_signal_event_definition

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_signal_event_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : EventDefinition
    pub super_event_definition: i64,
    /// COMPLEX FIELD : BPMN20-SignalEventDefinition-signalRef
    pub signal_ref: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE SignalEventDefinition need ONE EventDefinition
    #[sea_orm(
        belongs_to = "super::bpmn_20_event_definition::Entity",
        from = "Column::SuperEventDefinition",
        to = "super::bpmn_20_event_definition::Column::Id",
        on_delete = "Cascade"
    )]
    EventDefinition,
}

// SUPER : ONE SignalEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventDefinition.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "SignalEventDefinition" (bpmn_20_class_signal_event_definition)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Signal__ (__SignalModel__) from A_signalRef_signalEventDefinition
    ///   * one-to-many link : (0-1) __SignalEventDefinition__ need (0-inf) __Signal__)
    ///   * callable using find_with_related(__SignalModel__) from __SignalEventDefinition__
    /// 
    /// ## Direct Super :
    /// * __EventDefinition__ (__EventDefinitionModel__)
    ///   * one-to-one link : one __SignalEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __SignalEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "SignalEventDefinition" (bpmn_20_class_signal_event_definition)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __Signal__ (__SignalModel__) from A_signalRef_signalEventDefinition
  * one-to-many link : (0-1) __SignalEventDefinition__ need (0-inf) __Signal__)
  * callable using find_with_related(__SignalModel__) from __SignalEventDefinition__

## Direct Super :
* __EventDefinition__ (__EventDefinitionModel__)
  * one-to-one link : one __SignalEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __SignalEventDefinition__
  * saved in __super_event_definition__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "SignalEventDefinition",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "SignalEventDefinition",
//     is_abstract: false,
//     super_class: [
//         "EventDefinition",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-SignalEventDefinition-signalRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "SignalEventDefinition-signalRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "signalRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Signal",
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
//                     "A_signalRef_signalEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#SignalEventDefinition",
//     table_name: "bpmn_20_signal_event_definition",
//     model_name: "SignalEventDefinition",
//     full_name: "bpmn_20_class_signal_event_definition",
// }

