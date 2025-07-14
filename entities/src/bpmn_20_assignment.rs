//! bpmn_20_class_assignment

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_assignment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : Assignment-from
    pub from: i64,
    /// COMPLEX FIELD : Assignment-to
    pub to: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Assignment need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE Assignment need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Assignment" (bpmn_20_class_assignment)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __Expression__ (__ExpressionModel__) from A_from_assignment
    ///   * one-to-one link : (1-1) __Assignment__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __Assignment__
    ///   * saved in __from__ field as foreing key
    /// * __Expression__ (__ExpressionModel__) from A_to_assignment
    ///   * one-to-one link : (1-1) __Assignment__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __Assignment__
    ///   * saved in __to__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __DataAssociation__ (__DataAssociationModel__) from A_assignment_dataAssociation
    ///   * one-to-many link : (1-1) __Assignment__ need (0-inf) __DataAssociation__)
    ///   * callable using find_with_related(__DataAssociationModel__) from __Assignment__
    ///   * named data_association in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __Assignment__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Assignment__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Assignment" (bpmn_20_class_assignment)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __Expression__ (__ExpressionModel__) from A_from_assignment
  * one-to-one link : (1-1) __Assignment__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __Assignment__
  * saved in __from__ field as foreing key
* __Expression__ (__ExpressionModel__) from A_to_assignment
  * one-to-one link : (1-1) __Assignment__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __Assignment__
  * saved in __to__ field as foreing key

## Relation : One To Many :
* __DataAssociation__ (__DataAssociationModel__) from A_assignment_dataAssociation
  * one-to-many link : (1-1) __Assignment__ need (0-inf) __DataAssociation__)
  * callable using find_with_related(__DataAssociationModel__) from __Assignment__
  * named data_association in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __Assignment__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Assignment__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Assignment",
//     name: "Assignment",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Assignment-from",
//                 name: "from",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Expression",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
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
//                     "A_from_assignment",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Assignment-to",
//                 name: "to",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Expression",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
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
//                     "A_to_assignment",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

