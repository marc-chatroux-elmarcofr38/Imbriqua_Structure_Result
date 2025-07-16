//! bpmn_20_class_event_based_gateway

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_event_based_gateway")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Gateway
    pub super_gateway: i64,
    /// SIMPLE FIELD : EventBasedGateway-eventGatewayType
    pub event_gateway_type: EventBasedGatewayType,
    /// SIMPLE FIELD : EventBasedGateway-instantiate
    #[sea_orm(default_value = "false")]
    pub instantiate: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE EventBasedGateway need ONE Gateway
    #[sea_orm(
        belongs_to = "super::bpmn_20_gateway::Entity",
        from = "Column::SuperGateway",
        to = "super::bpmn_20_gateway::Column::Id",
        on_delete = "Cascade"
    )]
    Gateway,
}

// SUPER : ONE EventBasedGateway need ONE Gateway
impl Related<super::bpmn_20_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gateway.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "EventBasedGateway" (bpmn_20_class_event_based_gateway)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __event_gateway_type__ (xmi_id : "EventBasedGateway-eventGatewayType")
    ///   * type : __EventBasedGatewayType__
    /// * __instantiate__ (xmi_id : "EventBasedGateway-instantiate")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Gateway__ (__GatewayModel__)
    ///   * one-to-one link : one __EventBasedGateway__ need one __Gateway__)
    ///   * callable using find_also_related(__GatewayModel__) from __EventBasedGateway__
    ///   * saved in __super_gateway__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "EventBasedGateway" (bpmn_20_class_event_based_gateway)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __event_gateway_type__ (xmi_id : "EventBasedGateway-eventGatewayType")
  * type : __EventBasedGatewayType__
* __instantiate__ (xmi_id : "EventBasedGateway-instantiate")
  * type : __std::primitive::bool__
  * default : "false"



## Direct Super :
* __Gateway__ (__GatewayModel__)
  * one-to-one link : one __EventBasedGateway__ need one __Gateway__)
  * callable using find_also_related(__GatewayModel__) from __EventBasedGateway__
  * saved in __super_gateway__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "EventBasedGateway",
//     name: "EventBasedGateway",
//     is_abstract: false,
//     super_class: [
//         "Gateway",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "EventBasedGateway-eventGatewayType": Property(
//             CMOFProperty {
//                 xmi_id: "EventBasedGateway-eventGatewayType",
//                 name: "eventGatewayType",
//                 visibility: Public,
//                 simple_type: Some(
//                     "EventBasedGatewayType",
//                 ),
//                 complex_type: None,
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
//         "EventBasedGateway-instantiate": Property(
//             CMOFProperty {
//                 xmi_id: "EventBasedGateway-instantiate",
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#EventBasedGateway",
//     table_name: "bpmn_20_event_based_gateway",
//     model_name: "EventBasedGateway",
//     full_name: "bpmn_20_class_event_based_gateway",
// }

