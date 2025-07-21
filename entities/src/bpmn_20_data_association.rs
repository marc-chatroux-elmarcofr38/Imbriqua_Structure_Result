//! bpmn_20_class_data_association

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_association")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : BPMN20-DataAssociation-targetRef
    pub target_ref: i64,
    /// COMPLEX FIELD : BPMN20-DataAssociation-transformation
    pub transformation: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE DataAssociation need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
    // SUPER : ONE DataInputAssociation need ONE DataAssociation
    #[sea_orm(has_one = "super::bpmn_20_data_input_association::Entity")]
    DataInputAssociation,
    // SUPER : ONE DataOutputAssociation need ONE DataAssociation
    #[sea_orm(has_one = "super::bpmn_20_data_output_association::Entity")]
    DataOutputAssociation,
}

// SUPER : ONE DataAssociation need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE DataInputAssociation need ONE DataAssociation
impl Related<super::bpmn_20_data_input_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataInputAssociation.def()
    }
}

// SUPER : ONE DataOutputAssociation need ONE DataAssociation
impl Related<super::bpmn_20_data_output_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataOutputAssociation.def()
    }
}

// ManyToMany : with ItemAwareElement using A_sourceRef_dataAssociation
impl Related<super::bpmn_20_a_source_ref_data_association::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_source_ref_data_association::Relation::ItemAwareElement.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_source_ref_data_association::Relation::DataAssociation
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "DataAssociation" (bpmn_20_class_data_association)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __FormalExpression__ (__FormalExpressionModel__) from A_transformation_dataAssociation
    ///   * one-to-one link : (0-1) __DataAssociation__ need (0-1) __FormalExpression__)
    ///   * callable using find_also_related(__FormalExpressionModel__) from __DataAssociation__
    ///   * saved in __transformation__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __ItemAwareElement__ (__ItemAwareElementModel__) from A_targetRef_dataAssociation
    ///   * one-to-many link : (1-1) __DataAssociation__ need (0-inf) __ItemAwareElement__)
    ///   * callable using find_with_related(__ItemAwareElementModel__) from __DataAssociation__
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __DataAssociation__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __DataAssociation__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __DataInputAssociation__ (__DataInputAssociationModel__)
    ///   * one-to-one link (reverse) : one __DataInputAssociation__ need one __DataAssociation__)
    ///   * callable using find_also_related(__DataAssociationModel__) from __DataInputAssociation__
    ///   * saved in __super_data_association__ field as foreing key in __DataInputAssociationModel__
    /// * __DataOutputAssociation__ (__DataOutputAssociationModel__)
    ///   * one-to-one link (reverse) : one __DataOutputAssociation__ need one __DataAssociation__)
    ///   * callable using find_also_related(__DataAssociationModel__) from __DataOutputAssociation__
    ///   * saved in __super_data_association__ field as foreing key in __DataOutputAssociationModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "DataAssociation" (bpmn_20_class_data_association)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __FormalExpression__ (__FormalExpressionModel__) from A_transformation_dataAssociation
  * one-to-one link : (0-1) __DataAssociation__ need (0-1) __FormalExpression__)
  * callable using find_also_related(__FormalExpressionModel__) from __DataAssociation__
  * saved in __transformation__ field as foreing key

## Relation : One To Many :
* __ItemAwareElement__ (__ItemAwareElementModel__) from A_targetRef_dataAssociation
  * one-to-many link : (1-1) __DataAssociation__ need (0-inf) __ItemAwareElement__)
  * callable using find_with_related(__ItemAwareElementModel__) from __DataAssociation__

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __DataAssociation__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __DataAssociation__
  * saved in __super_base_element__ field as foreing key

## Reverse Super :
* __DataInputAssociation__ (__DataInputAssociationModel__)
  * one-to-one link (reverse) : one __DataInputAssociation__ need one __DataAssociation__)
  * callable using find_also_related(__DataAssociationModel__) from __DataInputAssociation__
  * saved in __super_data_association__ field as foreing key in __DataInputAssociationModel__
* __DataOutputAssociation__ (__DataOutputAssociationModel__)
  * one-to-one link (reverse) : one __DataOutputAssociation__ need one __DataAssociation__)
  * callable using find_also_related(__DataAssociationModel__) from __DataOutputAssociation__
  * saved in __super_data_association__ field as foreing key in __DataOutputAssociationModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "DataAssociation",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "DataAssociation",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "DataAssociation-assignment": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "DataAssociation-assignment",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "assignment",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Assignment",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_assignment_dataAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "DataAssociation-sourceRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "DataAssociation-sourceRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "sourceRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemAwareElement",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_sourceRef_dataAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "DataAssociation-targetRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "DataAssociation-targetRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "targetRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemAwareElement",
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
//                 association: Some(
//                     "A_targetRef_dataAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "DataAssociation-transformation": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "DataAssociation-transformation",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "transformation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "FormalExpression",
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
//                     "A_transformation_dataAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#DataAssociation",
//     table_name: "bpmn_20_data_association",
//     model_name: "DataAssociation",
//     full_name: "bpmn_20_class_data_association",
// }

