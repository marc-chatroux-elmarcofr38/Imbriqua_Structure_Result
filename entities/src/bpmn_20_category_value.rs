//! bpmn_20_class_category_value

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_category_value")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SIMPLE FIELD : BPMN20-CategoryValue-value
    pub value: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CategoryValue need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE CategoryValue need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// ManyToMany : with FlowElement using A_categorizedFlowElements_categoryValueRef
impl Related<super::bpmn_20_a_categorized_flow_elements_category_value_ref::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_categorized_flow_elements_category_value_ref::Relation::FlowElement.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_categorized_flow_elements_category_value_ref::Relation::CategoryValue
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "CategoryValue" (bpmn_20_class_category_value)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __value__ (xmi_id : "BPMN20-CategoryValue-value")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Category__ (__CategoryModel__) from A_categoryValue_category
    ///   * one-to-many link : (1-1) __CategoryValue__ need (0-inf) __Category__)
    ///   * callable using find_with_related(__CategoryModel__) from __CategoryValue__
    ///   * named category in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __CategoryValue__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __CategoryValue__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "CategoryValue" (bpmn_20_class_category_value)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __value__ (xmi_id : "BPMN20-CategoryValue-value")
  * type : __std::string::String__


## Relation : One To Many :
* __Category__ (__CategoryModel__) from A_categoryValue_category
  * one-to-many link : (1-1) __CategoryValue__ need (0-inf) __Category__)
  * callable using find_with_related(__CategoryModel__) from __CategoryValue__
  * named category in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __CategoryValue__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __CategoryValue__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "CategoryValue",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "CategoryValue",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-CategoryValue-categorizedFlowElements": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "CategoryValue-categorizedFlowElements",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "categorizedFlowElements",
//                 visibility: Public,
//                 simple_type: Some(
//                     "FlowElement",
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
//                 is_derived: true,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_categorizedFlowElements_categoryValueRef",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-CategoryValue-value": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "CategoryValue-value",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "value",
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#CategoryValue",
//     table_name: "bpmn_20_category_value",
//     model_name: "CategoryValue",
//     full_name: "bpmn_20_class_category_value",
// }

