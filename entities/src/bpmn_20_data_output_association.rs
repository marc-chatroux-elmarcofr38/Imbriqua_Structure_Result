//! bpmn_20_class_data_output_association

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_output_association")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : DataAssociation
    pub super_data_association: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE DataOutputAssociation need ONE DataAssociation
    #[sea_orm(
        belongs_to = "super::bpmn_20_data_association::Entity",
        from = "Column::SuperDataAssociation",
        to = "super::bpmn_20_data_association::Column::Id",
        on_delete = "Cascade"
    )]
    DataAssociation,
}

// SUPER : ONE DataOutputAssociation need ONE DataAssociation
impl Related<super::bpmn_20_data_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataAssociation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "DataOutputAssociation" (bpmn_20_class_data_output_association)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __CatchEvent__ (__CatchEventModel__) from A_dataOutputAssociation_catchEvent
    ///   * one-to-many link : (0-1) __DataOutputAssociation__ need (0-inf) __CatchEvent__)
    ///   * callable using find_with_related(__CatchEventModel__) from __DataOutputAssociation__
    ///   * named catch_event in BPMN
    /// * __Activity__ (__ActivityModel__) from A_dataOutputAssociations_activity
    ///   * one-to-many link : (0-1) __DataOutputAssociation__ need (0-inf) __Activity__)
    ///   * callable using find_with_related(__ActivityModel__) from __DataOutputAssociation__
    ///   * named activity in BPMN
    /// 
    /// ## Direct Super :
    /// * __DataAssociation__ (__DataAssociationModel__)
    ///   * one-to-one link : one __DataOutputAssociation__ need one __DataAssociation__)
    ///   * callable using find_also_related(__DataAssociationModel__) from __DataOutputAssociation__
    ///   * saved in __super_data_association__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "DataOutputAssociation" (bpmn_20_class_data_output_association)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __CatchEvent__ (__CatchEventModel__) from A_dataOutputAssociation_catchEvent
  * one-to-many link : (0-1) __DataOutputAssociation__ need (0-inf) __CatchEvent__)
  * callable using find_with_related(__CatchEventModel__) from __DataOutputAssociation__
  * named catch_event in BPMN
* __Activity__ (__ActivityModel__) from A_dataOutputAssociations_activity
  * one-to-many link : (0-1) __DataOutputAssociation__ need (0-inf) __Activity__)
  * callable using find_with_related(__ActivityModel__) from __DataOutputAssociation__
  * named activity in BPMN

## Direct Super :
* __DataAssociation__ (__DataAssociationModel__)
  * one-to-one link : one __DataOutputAssociation__ need one __DataAssociation__)
  * callable using find_also_related(__DataAssociationModel__) from __DataOutputAssociation__
  * saved in __super_data_association__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-DataOutputAssociation" (loaded : false)",
//     name: "DataOutputAssociation",
//     is_abstract: false,
//     super_class: [
//         "DataAssociation",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#DataOutputAssociation",
//     table_name: "bpmn_20_data_output_association",
//     model_name: "DataOutputAssociation",
//     full_name: "bpmn_20_class_data_output_association",
// }

