# Imbriqua Structure Result : Interpreter of BPMN model files (in UML notation) for Imbriqua Engine project

## Context

The Object Management Group® (OMG®) is an international, open membership, not-for-profit technology standards consortium, who create and make evolve specification for modeling language.

Updated in 2010, the BPMN™ (Business Process Model and Notation) specification propose a normative language for business process modeling. This notation is schematic-oriented and highly adaptive. BPMN project can be exchange in XML-structured files. This repository proposes to create a interpreter who generates BPMN class and function code (in RUST) from BPMN metamodel files.

## Use case

This repository allowing work-check of Imbriqua Structure Loader package result

By adding Imbriqua Structure Loader package result (Rust file), we can check module syntax, documentation syntax, and test succes

Result is acceptable after running following command without error :
 * Run `cargo test --all-features --no-run --lib --package=imbriqua_structure_result`
 * Run `cargo doc --no-deps`

## Copyright

Copyright 2023-2024 CHATROUX MARC

This file is part of Imbriqua Structure, a interpreter of BPMN model files (in UML notation) for Imbriqua Engine project
Imbriqua Structure is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
Imbriqua Structure is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Imbriqua Structure. If not, see <https://www.gnu.org/licenses/>.
