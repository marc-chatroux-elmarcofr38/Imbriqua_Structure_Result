//! bpmn_20_class_complex_gateway

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_complex_gateway")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Gateway
    pub super_gateway: i64,
    /// COMPLEX FIELD : BPMN20-ComplexGateway-activationCondition
    pub activation_condition: Option<i64>,
    /// COMPLEX FIELD : BPMN20-ComplexGateway-default
    pub default: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ComplexGateway need ONE Gateway
    #[sea_orm(
        belongs_to = "super::bpmn_20_gateway::Entity",
        from = "Column::SuperGateway",
        to = "super::bpmn_20_gateway::Column::Id",
        on_delete = "Cascade"
    )]
    Gateway,
}

// SUPER : ONE ComplexGateway need ONE Gateway
impl Related<super::bpmn_20_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gateway.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ComplexGateway" (bpmn_20_class_complex_gateway)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __Expression__ (__ExpressionModel__) from A_activationCondition_complexGateway
    ///   * one-to-one link : (0-1) __ComplexGateway__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __ComplexGateway__
    ///   * saved in __activation_condition__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __SequenceFlow__ (__SequenceFlowModel__) from A_default_complexGateway
    ///   * one-to-many link : (0-1) __ComplexGateway__ need (0-inf) __SequenceFlow__)
    ///   * callable using find_with_related(__SequenceFlowModel__) from __ComplexGateway__
    /// 
    /// ## Direct Super :
    /// * __Gateway__ (__GatewayModel__)
    ///   * one-to-one link : one __ComplexGateway__ need one __Gateway__)
    ///   * callable using find_also_related(__GatewayModel__) from __ComplexGateway__
    ///   * saved in __super_gateway__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ComplexGateway" (bpmn_20_class_complex_gateway)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __Expression__ (__ExpressionModel__) from A_activationCondition_complexGateway
  * one-to-one link : (0-1) __ComplexGateway__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __ComplexGateway__
  * saved in __activation_condition__ field as foreing key

## Relation : One To Many :
* __SequenceFlow__ (__SequenceFlowModel__) from A_default_complexGateway
  * one-to-many link : (0-1) __ComplexGateway__ need (0-inf) __SequenceFlow__)
  * callable using find_with_related(__SequenceFlowModel__) from __ComplexGateway__

## Direct Super :
* __Gateway__ (__GatewayModel__)
  * one-to-one link : one __ComplexGateway__ need one __Gateway__)
  * callable using find_also_related(__GatewayModel__) from __ComplexGateway__
  * saved in __super_gateway__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "ComplexGateway",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "ComplexGateway",
//     is_abstract: false,
//     super_class: [
//         "Gateway",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-ComplexGateway-activationCondition": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "ComplexGateway-activationCondition",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "activationCondition",
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
//                     "A_activationCondition_complexGateway",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-ComplexGateway-default": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "ComplexGateway-default",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "default",
//                 visibility: Public,
//                 simple_type: Some(
//                     "SequenceFlow",
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
//                     "A_default_complexGateway",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ComplexGateway",
//     table_name: "bpmn_20_complex_gateway",
//     model_name: "ComplexGateway",
//     full_name: "bpmn_20_class_complex_gateway",
// }

