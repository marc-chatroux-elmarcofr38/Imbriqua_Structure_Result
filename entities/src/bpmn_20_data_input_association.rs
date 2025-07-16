//! bpmn_20_class_data_input_association

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_input_association")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : DataAssociation
    pub super_data_association: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE DataInputAssociation need ONE DataAssociation
    #[sea_orm(
        belongs_to = "super::bpmn_20_data_association::Entity",
        from = "Column::SuperDataAssociation",
        to = "super::bpmn_20_data_association::Column::Id",
        on_delete = "Cascade"
    )]
    DataAssociation,
}

// SUPER : ONE DataInputAssociation need ONE DataAssociation
impl Related<super::bpmn_20_data_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataAssociation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "DataInputAssociation" (bpmn_20_class_data_input_association)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __ThrowEvent__ (__ThrowEventModel__) from A_dataInputAssociation_throwEvent
    ///   * one-to-many link : (0-1) __DataInputAssociation__ need (0-inf) __ThrowEvent__)
    ///   * callable using find_with_related(__ThrowEventModel__) from __DataInputAssociation__
    ///   * named throw_event in BPMN
    /// * __Activity__ (__ActivityModel__) from A_dataInputAssociations_activity
    ///   * one-to-many link : (0-1) __DataInputAssociation__ need (0-inf) __Activity__)
    ///   * callable using find_with_related(__ActivityModel__) from __DataInputAssociation__
    ///   * named activity in BPMN
    /// 
    /// ## Direct Super :
    /// * __DataAssociation__ (__DataAssociationModel__)
    ///   * one-to-one link : one __DataInputAssociation__ need one __DataAssociation__)
    ///   * callable using find_also_related(__DataAssociationModel__) from __DataInputAssociation__
    ///   * saved in __super_data_association__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "DataInputAssociation" (bpmn_20_class_data_input_association)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __ThrowEvent__ (__ThrowEventModel__) from A_dataInputAssociation_throwEvent
  * one-to-many link : (0-1) __DataInputAssociation__ need (0-inf) __ThrowEvent__)
  * callable using find_with_related(__ThrowEventModel__) from __DataInputAssociation__
  * named throw_event in BPMN
* __Activity__ (__ActivityModel__) from A_dataInputAssociations_activity
  * one-to-many link : (0-1) __DataInputAssociation__ need (0-inf) __Activity__)
  * callable using find_with_related(__ActivityModel__) from __DataInputAssociation__
  * named activity in BPMN

## Direct Super :
* __DataAssociation__ (__DataAssociationModel__)
  * one-to-one link : one __DataInputAssociation__ need one __DataAssociation__)
  * callable using find_also_related(__DataAssociationModel__) from __DataInputAssociation__
  * saved in __super_data_association__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "DataInputAssociation",
//     name: "DataInputAssociation",
//     is_abstract: false,
//     super_class: [
//         "DataAssociation",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#DataInputAssociation",
//     table_name: "bpmn_20_data_input_association",
//     model_name: "DataInputAssociation",
//     full_name: "bpmn_20_class_data_input_association",
// }

