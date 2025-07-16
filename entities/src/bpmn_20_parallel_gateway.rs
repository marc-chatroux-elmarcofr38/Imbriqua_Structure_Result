//! bpmn_20_class_parallel_gateway

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_parallel_gateway")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Gateway
    pub super_gateway: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ParallelGateway need ONE Gateway
    #[sea_orm(
        belongs_to = "super::bpmn_20_gateway::Entity",
        from = "Column::SuperGateway",
        to = "super::bpmn_20_gateway::Column::Id",
        on_delete = "Cascade"
    )]
    Gateway,
}

// SUPER : ONE ParallelGateway need ONE Gateway
impl Related<super::bpmn_20_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gateway.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ParallelGateway" (bpmn_20_class_parallel_gateway)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Gateway__ (__GatewayModel__)
    ///   * one-to-one link : one __ParallelGateway__ need one __Gateway__)
    ///   * callable using find_also_related(__GatewayModel__) from __ParallelGateway__
    ///   * saved in __super_gateway__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ParallelGateway" (bpmn_20_class_parallel_gateway)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __Gateway__ (__GatewayModel__)
  * one-to-one link : one __ParallelGateway__ need one __Gateway__)
  * callable using find_also_related(__GatewayModel__) from __ParallelGateway__
  * saved in __super_gateway__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ParallelGateway",
//     name: "ParallelGateway",
//     is_abstract: false,
//     super_class: [
//         "Gateway",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ParallelGateway",
//     table_name: "bpmn_20_parallel_gateway",
//     model_name: "ParallelGateway",
//     full_name: "bpmn_20_class_parallel_gateway",
// }

