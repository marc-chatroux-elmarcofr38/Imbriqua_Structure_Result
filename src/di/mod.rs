//! di

use derive_builder::Builder;
/// Link from _packageImport.0 (PackageImport)
use crate::dc;

/// Class : DiagramElement
pub mod DiagramElement;

/// Class : Node
pub mod Node;

/// Class : Edge
pub mod Edge;

/// Class : Diagram
pub mod Diagram;

/// Class : Shape
pub mod Shape;

/// Class : Plane
pub mod Plane;

/// Class : LabeledEdge
pub mod LabeledEdge;

/// Class : LabeledShape
pub mod LabeledShape;

/// Class : Label
pub mod Label;

/// Class : Style
pub mod Style;

/// Association : A_target_targetEdge
pub mod A_target_targetEdge;

/// Association : A_source_sourceEdge
pub mod A_source_sourceEdge;

/// Association : A_ownedElement_owningElement
pub mod A_ownedElement_owningElement;

/// Association : A_modelElement_diagramElement
pub mod A_modelElement_diagramElement;

/// Association : A_rootElement_owningDiagram
pub mod A_rootElement_owningDiagram;

/// Association : A_ownedLabel_owningEdge
pub mod A_ownedLabel_owningEdge;

/// Association : A_planeElement_plane
pub mod A_planeElement_plane;

/// Association : A_style_diagramElement
pub mod A_style_diagramElement;

/// Association : A_ownedStyle_owningDiagram
pub mod A_ownedStyle_owningDiagram;

/// Association : A_ownedLabel_owningShape
pub mod A_ownedLabel_owningShape;
