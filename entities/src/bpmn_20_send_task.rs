//! bpmn_20_class_send_task

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_send_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Task
    pub super_task: i64,
    /// COMPLEX FIELD : BPMN20-SendTask-messageRef
    pub message_ref: Option<i64>,
    /// COMPLEX FIELD : BPMN20-SendTask-operationRef
    pub operation_ref: Option<i64>,
    /// SIMPLE FIELD : BPMN20-SendTask-implementation
    pub implementation: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE SendTask need ONE Task
    #[sea_orm(
        belongs_to = "super::bpmn_20_task::Entity",
        from = "Column::SuperTask",
        to = "super::bpmn_20_task::Column::Id",
        on_delete = "Cascade"
    )]
    Task,
}

// SUPER : ONE SendTask need ONE Task
impl Related<super::bpmn_20_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "SendTask" (bpmn_20_class_send_task)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __implementation__ (xmi_id : "BPMN20-SendTask-implementation")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Message__ (__MessageModel__) from A_messageRef_sendTask
    ///   * one-to-many link : (0-1) __SendTask__ need (0-inf) __Message__)
    ///   * callable using find_with_related(__MessageModel__) from __SendTask__
    /// * __Operation__ (__OperationModel__) from A_operationRef_sendTask
    ///   * one-to-many link : (0-1) __SendTask__ need (0-inf) __Operation__)
    ///   * callable using find_with_related(__OperationModel__) from __SendTask__
    /// 
    /// ## Direct Super :
    /// * __Task__ (__TaskModel__)
    ///   * one-to-one link : one __SendTask__ need one __Task__)
    ///   * callable using find_also_related(__TaskModel__) from __SendTask__
    ///   * saved in __super_task__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "SendTask" (bpmn_20_class_send_task)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __implementation__ (xmi_id : "BPMN20-SendTask-implementation")
  * type : __std::string::String__


## Relation : One To Many :
* __Message__ (__MessageModel__) from A_messageRef_sendTask
  * one-to-many link : (0-1) __SendTask__ need (0-inf) __Message__)
  * callable using find_with_related(__MessageModel__) from __SendTask__
* __Operation__ (__OperationModel__) from A_operationRef_sendTask
  * one-to-many link : (0-1) __SendTask__ need (0-inf) __Operation__)
  * callable using find_with_related(__OperationModel__) from __SendTask__

## Direct Super :
* __Task__ (__TaskModel__)
  * one-to-one link : one __SendTask__ need one __Task__)
  * callable using find_also_related(__TaskModel__) from __SendTask__
  * saved in __super_task__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "SendTask",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "SendTask",
//     is_abstract: false,
//     super_class: [
//         "Task",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-SendTask-implementation": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "SendTask-implementation",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "implementation",
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
//         "-SendTask-messageRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "SendTask-messageRef",
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
//                     "A_messageRef_sendTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-SendTask-operationRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "SendTask-operationRef",
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
//                     "A_operationRef_sendTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#SendTask",
//     table_name: "bpmn_20_send_task",
//     model_name: "SendTask",
//     full_name: "bpmn_20_class_send_task",
// }

