//! bpmn_20_class_escalation

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_escalation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// COMPLEX FIELD : BPMN20-Escalation-structureRef
    pub structure_ref: Option<i64>,
    /// SIMPLE FIELD : BPMN20-Escalation-escalationCode
    pub escalation_code: std::string::String,
    /// SIMPLE FIELD : BPMN20-Escalation-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Escalation" (bpmn_20_class_escalation)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __escalation_code__ (xmi_id : "BPMN20-Escalation-escalationCode")
    ///   * type : __std::string::String__
    /// * __name__ (xmi_id : "BPMN20-Escalation-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __ItemDefinition__ (__ItemDefinitionModel__) from A_structureRef_escalation
    ///   * one-to-many link : (0-1) __Escalation__ need (0-inf) __ItemDefinition__)
    ///   * callable using find_with_related(__ItemDefinitionModel__) from __Escalation__
    /// 
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Escalation" (bpmn_20_class_escalation)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __escalation_code__ (xmi_id : "BPMN20-Escalation-escalationCode")
  * type : __std::string::String__
* __name__ (xmi_id : "BPMN20-Escalation-name")
  * type : __std::string::String__


## Relation : One To Many :
* __ItemDefinition__ (__ItemDefinitionModel__) from A_structureRef_escalation
  * one-to-many link : (0-1) __Escalation__ need (0-inf) __ItemDefinition__)
  * callable using find_with_related(__ItemDefinitionModel__) from __Escalation__



"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "Escalation",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Escalation",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "-Escalation-escalationCode": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Escalation-escalationCode",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "escalationCode",
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
//         "-Escalation-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Escalation-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
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
//         "-Escalation-structureRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Escalation-structureRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "structureRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemDefinition",
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
//                     "A_structureRef_escalation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Escalation",
//     table_name: "bpmn_20_escalation",
//     model_name: "Escalation",
//     full_name: "bpmn_20_class_escalation",
// }

