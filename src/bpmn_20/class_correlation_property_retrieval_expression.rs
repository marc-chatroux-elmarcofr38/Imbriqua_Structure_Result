//! CorrelationPropertyRetrievalExpression
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of CorrelationPropertyRetrievalExpression (Class : CorrelationPropertyRetrievalExpression)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationPropertyRetrievalExpression",
///     name: "CorrelationPropertyRetrievalExpression",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationPropertyRetrievalExpression-messagePath",
///                 name: "messagePath",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FormalExpression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_messagePath_correlationset",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationPropertyRetrievalExpression-messageRef",
///                 name: "messageRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Message",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_messageRef_correlationPropertyRetrievalExpression",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct CorrelationPropertyRetrievalExpression {
    #[builder(setter(into))]
    pub message_path: FormalExpression,
    #[builder(setter(into))]
    pub message_ref: Message,
}

