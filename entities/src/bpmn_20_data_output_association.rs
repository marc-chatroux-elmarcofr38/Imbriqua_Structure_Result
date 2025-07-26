//! bpmn_20_class_data_output_association

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_output_association")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperDataAssociation
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

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-DataOutputAssociation',
//     name: "DataOutputAssociation",
//     is_abstract: false,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-DataAssociation',
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#DataOutputAssociation",
//     table_name: "bpmn_20_data_output_association",
//     model_name: "DataOutputAssociation",
//     full_name: "bpmn_20_class_data_output_association",
//     reverse_super: RefCell {
//         value: [],
//     },
// }

