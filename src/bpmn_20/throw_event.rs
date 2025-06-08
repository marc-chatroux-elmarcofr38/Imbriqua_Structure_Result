//! throw_event

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ThrowEvent (Class : ThrowEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ThrowEvent",
///     name: "ThrowEvent",
///     is_abstract: true,
///     super_class: Some(
///         "Event",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ThrowEvent-inputSet",
///                 name: "inputSet",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
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
///                 association: "A_inputSet_throwEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ThrowEvent-eventDefinitionRefs",
///                 name: "eventDefinitionRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EventDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
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
///                 association: "A_eventDefinitionRefs_throwEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ThrowEvent-dataInputAssociation",
///                 name: "dataInputAssociation",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataInputAssociation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
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
///                 association: "A_dataInputAssociation_throwEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ThrowEvent-dataInputs",
///                 name: "dataInputs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataInput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
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
///                 association: "A_dataInputs_throwEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ThrowEvent-eventDefinitions",
///                 name: "eventDefinitions",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EventDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
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
///                 association: "A_eventDefinitions_throwEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct ThrowEvent<'a> {
    #[builder(setter(into, strip_option), default)]
    pub input_set: Option<&'a InputSet<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub event_definition_refs: Option<Vec<&'a EventDefinition<'a>>>,
    #[builder(setter(into, strip_option), default)]
    pub data_input_association: Option<Vec<&'a DataInputAssociation<'a>>>,
    #[builder(setter(into, strip_option), default)]
    pub data_inputs: Option<Vec<&'a DataInput<'a>>>,
    #[builder(setter(into, strip_option), default)]
    pub event_definitions: Option<Vec<&'a EventDefinition<'a>>>,
}

