//! di
/// Link from _packageImport.0 (PackageImport)
use crate::dc;

/// Class : DiagramElement
mod diagram_element;
pub use diagram_element::DiagramElement;

/// Class : Node
mod node;
pub use node::Node;

/// Class : Edge
mod edge;
pub use edge::Edge;

/// Class : Diagram
mod diagram;
pub use diagram::Diagram;

/// Class : Shape
mod shape;
pub use shape::Shape;

/// Class : Plane
mod plane;
pub use plane::Plane;

/// Class : LabeledEdge
mod labeled_edge;
pub use labeled_edge::LabeledEdge;

/// Class : LabeledShape
mod labeled_shape;
pub use labeled_shape::LabeledShape;

/// Class : Label
mod label;
pub use label::Label;

/// Class : Style
mod style;
pub use style::Style;

/// Association : A_target_targetEdge
mod a_target_target_edge;
pub use a_target_target_edge::ATargetTargetEdge;

/// Association : A_source_sourceEdge
mod a_source_source_edge;
pub use a_source_source_edge::ASourceSourceEdge;

/// Association : A_ownedElement_owningElement
mod a_owned_element_owning_element;
pub use a_owned_element_owning_element::AOwnedElementOwningElement;

/// Association : A_modelElement_diagramElement
mod a_model_element_diagram_element;
pub use a_model_element_diagram_element::AModelElementDiagramElement;

/// Association : A_rootElement_owningDiagram
mod a_root_element_owning_diagram;
pub use a_root_element_owning_diagram::ARootElementOwningDiagram;

/// Association : A_ownedLabel_owningEdge
mod a_owned_label_owning_edge;
pub use a_owned_label_owning_edge::AOwnedLabelOwningEdge;

/// Association : A_planeElement_plane
mod a_plane_element_plane;
pub use a_plane_element_plane::APlaneElementPlane;

/// Association : A_style_diagramElement
mod a_style_diagram_element;
pub use a_style_diagram_element::AStyleDiagramElement;

/// Association : A_ownedStyle_owningDiagram
mod a_owned_style_owning_diagram;
pub use a_owned_style_owning_diagram::AOwnedStyleOwningDiagram;

/// Association : A_ownedLabel_owningShape
mod a_owned_label_owning_shape;
pub use a_owned_label_owning_shape::AOwnedLabelOwningShape;
