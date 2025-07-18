//! bpmn_20_class_timer_event_definition

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_timer_event_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : EventDefinition
    pub super_event_definition: i64,
    /// COMPLEX FIELD : BPMN20-TimerEventDefinition-timeCycle
    pub time_cycle: Option<i64>,
    /// COMPLEX FIELD : BPMN20-TimerEventDefinition-timeDate
    pub time_date: Option<i64>,
    /// COMPLEX FIELD : BPMN20-TimerEventDefinition-timeDuration
    pub time_duration: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE TimerEventDefinition need ONE EventDefinition
    #[sea_orm(
        belongs_to = "super::bpmn_20_event_definition::Entity",
        from = "Column::SuperEventDefinition",
        to = "super::bpmn_20_event_definition::Column::Id",
        on_delete = "Cascade"
    )]
    EventDefinition,
}

// SUPER : ONE TimerEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventDefinition.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "TimerEventDefinition" (bpmn_20_class_timer_event_definition)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __Expression__ (__ExpressionModel__) from A_timeCycle_timerEventDefinition
    ///   * one-to-one link : (0-1) __TimerEventDefinition__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __TimerEventDefinition__
    ///   * saved in __time_cycle__ field as foreing key
    /// * __Expression__ (__ExpressionModel__) from A_timeDate_timerEventDefinition
    ///   * one-to-one link : (0-1) __TimerEventDefinition__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __TimerEventDefinition__
    ///   * saved in __time_date__ field as foreing key
    /// * __Expression__ (__ExpressionModel__) from A_timeDuration_timerEventDefinition
    ///   * one-to-one link : (0-1) __TimerEventDefinition__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __TimerEventDefinition__
    ///   * saved in __time_duration__ field as foreing key
    /// 
    /// 
    /// ## Direct Super :
    /// * __EventDefinition__ (__EventDefinitionModel__)
    ///   * one-to-one link : one __TimerEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __TimerEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "TimerEventDefinition" (bpmn_20_class_timer_event_definition)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __Expression__ (__ExpressionModel__) from A_timeCycle_timerEventDefinition
  * one-to-one link : (0-1) __TimerEventDefinition__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __TimerEventDefinition__
  * saved in __time_cycle__ field as foreing key
* __Expression__ (__ExpressionModel__) from A_timeDate_timerEventDefinition
  * one-to-one link : (0-1) __TimerEventDefinition__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __TimerEventDefinition__
  * saved in __time_date__ field as foreing key
* __Expression__ (__ExpressionModel__) from A_timeDuration_timerEventDefinition
  * one-to-one link : (0-1) __TimerEventDefinition__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __TimerEventDefinition__
  * saved in __time_duration__ field as foreing key


## Direct Super :
* __EventDefinition__ (__EventDefinitionModel__)
  * one-to-one link : one __TimerEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __TimerEventDefinition__
  * saved in __super_event_definition__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "TimerEventDefinition",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "TimerEventDefinition",
//     is_abstract: false,
//     super_class: [
//         "EventDefinition",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-TimerEventDefinition-timeCycle": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "TimerEventDefinition-timeCycle",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "timeCycle",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Expression",
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
//                     "A_timeCycle_timerEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-TimerEventDefinition-timeDate": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "TimerEventDefinition-timeDate",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "timeDate",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Expression",
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
//                     "A_timeDate_timerEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-TimerEventDefinition-timeDuration": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "TimerEventDefinition-timeDuration",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "timeDuration",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Expression",
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
//                     "A_timeDuration_timerEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#TimerEventDefinition",
//     table_name: "bpmn_20_timer_event_definition",
//     model_name: "TimerEventDefinition",
//     full_name: "bpmn_20_class_timer_event_definition",
// }

