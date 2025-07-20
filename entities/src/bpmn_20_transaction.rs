//! bpmn_20_class_transaction

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_transaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SubProcess
    pub super_sub_process: i64,
    /// SIMPLE FIELD : BPMN20-Transaction-method
    pub method: std::string::String,
    /// SIMPLE FIELD : BPMN20-Transaction-protocol
    pub protocol: Option<std::string::String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Transaction need ONE SubProcess
    #[sea_orm(
        belongs_to = "super::bpmn_20_sub_process::Entity",
        from = "Column::SuperSubProcess",
        to = "super::bpmn_20_sub_process::Column::Id",
        on_delete = "Cascade"
    )]
    SubProcess,
}

// SUPER : ONE Transaction need ONE SubProcess
impl Related<super::bpmn_20_sub_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubProcess.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Transaction" (bpmn_20_class_transaction)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __method__ (xmi_id : "BPMN20-Transaction-method")
    ///   * type : __std::string::String__
    /// * __protocol__ (xmi_id : "BPMN20-Transaction-protocol")
    ///   * type : __Option<std::string::String>__
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __SubProcess__ (__SubProcessModel__)
    ///   * one-to-one link : one __Transaction__ need one __SubProcess__)
    ///   * callable using find_also_related(__SubProcessModel__) from __Transaction__
    ///   * saved in __super_sub_process__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Transaction" (bpmn_20_class_transaction)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __method__ (xmi_id : "BPMN20-Transaction-method")
  * type : __std::string::String__
* __protocol__ (xmi_id : "BPMN20-Transaction-protocol")
  * type : __Option<std::string::String>__



## Direct Super :
* __SubProcess__ (__SubProcessModel__)
  * one-to-one link : one __Transaction__ need one __SubProcess__)
  * callable using find_also_related(__SubProcessModel__) from __Transaction__
  * saved in __super_sub_process__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-Transaction" (loaded : false)",
//     name: "Transaction",
//     is_abstract: false,
//     super_class: [
//         "SubProcess",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Transaction-method": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-Transaction-method" (loaded : false)",
//                 name: "method",
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
//         "-Transaction-protocol": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-Transaction-protocol" (loaded : false)",
//                 name: "protocol",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Transaction",
//     table_name: "bpmn_20_transaction",
//     model_name: "Transaction",
//     full_name: "bpmn_20_class_transaction",
// }

