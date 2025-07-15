//! bpmn_20_class_receive_task

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_receive_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Task
    pub super_task: i64,
    /// COMPLEX FIELD : ReceiveTask-operationRef
    pub operation_ref: Option<i64>,
    /// COMPLEX FIELD : ReceiveTask-messageRef
    pub message_ref: Option<i64>,
    /// SIMPLE FIELD : ReceiveTask-implementation
    pub implementation: std::string::String,
    /// SIMPLE FIELD : ReceiveTask-instantiate
    #[sea_orm(default_value = "false")]
    pub instantiate: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ReceiveTask need ONE Task
    #[sea_orm(
        belongs_to = "super::bpmn_20_task::Entity",
        from = "Column::SuperTask",
        to = "super::bpmn_20_task::Column::Id",
        on_delete = "Cascade"
    )]
    Task,
}

// SUPER : ONE ReceiveTask need ONE Task
impl Related<super::bpmn_20_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ReceiveTask" (bpmn_20_class_receive_task)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __implementation__ (xmi_id : "ReceiveTask-implementation")
    ///   * type : __std::string::String__
    /// * __instantiate__ (xmi_id : "ReceiveTask-instantiate")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Message__ (__MessageModel__) from A_messageRef_receiveTask
    ///   * one-to-many link : (0-1) __ReceiveTask__ need (0-inf) __Message__)
    ///   * callable using find_with_related(__MessageModel__) from __ReceiveTask__
    /// * __Operation__ (__OperationModel__) from A_operationRef_receiveTask
    ///   * one-to-many link : (0-1) __ReceiveTask__ need (0-inf) __Operation__)
    ///   * callable using find_with_related(__OperationModel__) from __ReceiveTask__
    /// 
    /// ## Direct Super :
    /// * __Task__ (__TaskModel__)
    ///   * one-to-one link : one __ReceiveTask__ need one __Task__)
    ///   * callable using find_also_related(__TaskModel__) from __ReceiveTask__
    ///   * saved in __super_task__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ReceiveTask" (bpmn_20_class_receive_task)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __implementation__ (xmi_id : "ReceiveTask-implementation")
  * type : __std::string::String__
* __instantiate__ (xmi_id : "ReceiveTask-instantiate")
  * type : __std::primitive::bool__
  * default : "false"


## Relation : One To Many :
* __Message__ (__MessageModel__) from A_messageRef_receiveTask
  * one-to-many link : (0-1) __ReceiveTask__ need (0-inf) __Message__)
  * callable using find_with_related(__MessageModel__) from __ReceiveTask__
* __Operation__ (__OperationModel__) from A_operationRef_receiveTask
  * one-to-many link : (0-1) __ReceiveTask__ need (0-inf) __Operation__)
  * callable using find_with_related(__OperationModel__) from __ReceiveTask__

## Direct Super :
* __Task__ (__TaskModel__)
  * one-to-one link : one __ReceiveTask__ need one __Task__)
  * callable using find_also_related(__TaskModel__) from __ReceiveTask__
  * saved in __super_task__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ReceiveTask",
//     name: "ReceiveTask",
//     is_abstract: false,
//     super_class: [
//         "Task",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ReceiveTask-implementation",
//                 name: "implementation",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "ReceiveTask-instantiate",
//                 name: "instantiate",
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
//                 default: Some(
//                     "false",
//                 ),
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
//                 xmi_id: "ReceiveTask-operationRef",
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
//                     "A_operationRef_receiveTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ReceiveTask-messageRef",
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
//                     "A_messageRef_receiveTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

