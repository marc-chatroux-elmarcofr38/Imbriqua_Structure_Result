//! bpmn_label

use crate::bpmndi::*;
use crate::Builder;

/// Conversion of BPMNLabel (Class : BPMNLabel)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNLabel",
///     name: "BPMNLabel",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#Label",
///             },
///         ),
///     ),
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNLabel-labelStyle",
///                 name: "labelStyle",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BPMNLabelStyle",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
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
///                 association: "A_labelStyle_label",
///                 redefined_property_link: Some(
///                     Property(
///                         RedefinedProperty {
///                             href: "DI.cmof#DiagramElement-style",
///                         },
///                     ),
///                 ),
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct BPMNLabel<'a> {
    #[builder(setter(into, strip_option), default)]
    pub label_style: Option<&'a BPMNLabelStyle<'a>>,
}

