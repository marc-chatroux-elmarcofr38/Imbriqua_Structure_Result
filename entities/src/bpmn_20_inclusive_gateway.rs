//! bpmn_20_class_inclusive_gateway

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_inclusive_gateway")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Gateway
    pub super_gateway: i64,
    /// COMPLEX FIELD : BPMN20-InclusiveGateway-default
    pub default: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE InclusiveGateway need ONE Gateway
    #[sea_orm(
        belongs_to = "super::bpmn_20_gateway::Entity",
        from = "Column::SuperGateway",
        to = "super::bpmn_20_gateway::Column::Id",
        on_delete = "Cascade"
    )]
    Gateway,
}

// SUPER : ONE InclusiveGateway need ONE Gateway
impl Related<super::bpmn_20_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gateway.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "InclusiveGateway" (bpmn_20_class_inclusive_gateway)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __SequenceFlow__ (__SequenceFlowModel__) from A_default_inclusiveGateway
    ///   * one-to-many link : (0-1) __InclusiveGateway__ need (0-inf) __SequenceFlow__)
    ///   * callable using find_with_related(__SequenceFlowModel__) from __InclusiveGateway__
    /// 
    /// ## Direct Super :
    /// * __Gateway__ (__GatewayModel__)
    ///   * one-to-one link : one __InclusiveGateway__ need one __Gateway__)
    ///   * callable using find_also_related(__GatewayModel__) from __InclusiveGateway__
    ///   * saved in __super_gateway__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "InclusiveGateway" (bpmn_20_class_inclusive_gateway)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __SequenceFlow__ (__SequenceFlowModel__) from A_default_inclusiveGateway
  * one-to-many link : (0-1) __InclusiveGateway__ need (0-inf) __SequenceFlow__)
  * callable using find_with_related(__SequenceFlowModel__) from __InclusiveGateway__

## Direct Super :
* __Gateway__ (__GatewayModel__)
  * one-to-one link : one __InclusiveGateway__ need one __Gateway__)
  * callable using find_also_related(__GatewayModel__) from __InclusiveGateway__
  * saved in __super_gateway__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "InclusiveGateway",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "InclusiveGateway",
//     is_abstract: false,
//     super_class: [
//         "Gateway",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "InclusiveGateway-default": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "InclusiveGateway-default",
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
//                     "A_default_inclusiveGateway",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#InclusiveGateway",
//     table_name: "bpmn_20_inclusive_gateway",
//     model_name: "InclusiveGateway",
//     full_name: "bpmn_20_class_inclusive_gateway",
// }

