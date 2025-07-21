//! bpmn_20_class_message_event_definition

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_message_event_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : EventDefinition
    pub super_event_definition: i64,
    /// COMPLEX FIELD : BPMN20-MessageEventDefinition-messageRef
    pub message_ref: Option<i64>,
    /// COMPLEX FIELD : BPMN20-MessageEventDefinition-operationRef
    pub operation_ref: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE MessageEventDefinition need ONE EventDefinition
    #[sea_orm(
        belongs_to = "super::bpmn_20_event_definition::Entity",
        from = "Column::SuperEventDefinition",
        to = "super::bpmn_20_event_definition::Column::Id",
        on_delete = "Cascade"
    )]
    EventDefinition,
}

// SUPER : ONE MessageEventDefinition need ONE EventDefinition
impl Related<super::bpmn_20_event_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventDefinition.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "MessageEventDefinition" (bpmn_20_class_message_event_definition)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Message__ (__MessageModel__) from A_messageRef_messageEventDefinition
    ///   * one-to-many link : (0-1) __MessageEventDefinition__ need (0-inf) __Message__)
    ///   * callable using find_with_related(__MessageModel__) from __MessageEventDefinition__
    /// * __Operation__ (__OperationModel__) from A_operationRef_messageEventDefinition
    ///   * one-to-many link : (0-1) __MessageEventDefinition__ need (0-inf) __Operation__)
    ///   * callable using find_with_related(__OperationModel__) from __MessageEventDefinition__
    /// 
    /// ## Direct Super :
    /// * __EventDefinition__ (__EventDefinitionModel__)
    ///   * one-to-one link : one __MessageEventDefinition__ need one __EventDefinition__)
    ///   * callable using find_also_related(__EventDefinitionModel__) from __MessageEventDefinition__
    ///   * saved in __super_event_definition__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "MessageEventDefinition" (bpmn_20_class_message_event_definition)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __Message__ (__MessageModel__) from A_messageRef_messageEventDefinition
  * one-to-many link : (0-1) __MessageEventDefinition__ need (0-inf) __Message__)
  * callable using find_with_related(__MessageModel__) from __MessageEventDefinition__
* __Operation__ (__OperationModel__) from A_operationRef_messageEventDefinition
  * one-to-many link : (0-1) __MessageEventDefinition__ need (0-inf) __Operation__)
  * callable using find_with_related(__OperationModel__) from __MessageEventDefinition__

## Direct Super :
* __EventDefinition__ (__EventDefinitionModel__)
  * one-to-one link : one __MessageEventDefinition__ need one __EventDefinition__)
  * callable using find_also_related(__EventDefinitionModel__) from __MessageEventDefinition__
  * saved in __super_event_definition__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "MessageEventDefinition",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "MessageEventDefinition",
//     is_abstract: false,
//     super_class: [
//         "EventDefinition",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "MessageEventDefinition-messageRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "MessageEventDefinition-messageRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "messageRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Message",
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
//                     "A_messageRef_messageEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "MessageEventDefinition-operationRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "MessageEventDefinition-operationRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "operationRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Operation",
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
//                     "A_operationRef_messageEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#MessageEventDefinition",
//     table_name: "bpmn_20_message_event_definition",
//     model_name: "MessageEventDefinition",
//     full_name: "bpmn_20_class_message_event_definition",
// }

