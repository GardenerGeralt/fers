use nalgebra as na;

use crate::node::Node;
use crate::element::Element1D;
use crate::section::Section;
use crate::outputdata::OutputData;

pub struct Model<'a> {
    nodes: Vec<Node>,
    connectivity: Vec<Vec<usize>>,
    section_assignments: Vec<Section<'a>>,
    boundary_conditions: Vec<Vec<u32>>,
    loads: na::DVector<f32>,
    node_count: usize,
    ndof: usize
}

impl Model<'_> {
    pub fn new(
        nodes: Vec<Node>, 
        connectivity: Vec<Vec<usize>>,
        section_assignments: Vec<Section>,
        boundary_conditions: Vec<Vec<u32>>,
        loads: na::DVector<f32>
    ) -> Model {
        let node_count = nodes.len();
        let ndof = 2 * node_count;
        Model {
            nodes,
            connectivity,
            section_assignments,
            boundary_conditions,
            loads,
            node_count,
            ndof
        }
    }

    fn assemble_elements(&self) -> Vec<Element1D> {
        self.connectivity.iter().enumerate().map(|(i, v)| {
            Element1D::new(
                &self.nodes[v[0]], 
                &self.nodes[v[1]],
                Some(&self.section_assignments[i])
            )
        }).collect()
    }

    fn assemble_global_stiffness_matrix(
        &self, elements: &Vec<Element1D>
    ) -> na::DMatrix<f32> {
        let mut global_stiffness_matrix = na::DMatrix::from_element(self.ndof, self.ndof, 0.0);

        for elem in elements {
            let start_node_dof = elem.start_node_nr..(elem.start_node_nr + 2);
            let end_node_dof = elem.end_node_nr..(elem.end_node_nr + 2);
            let dof_indices: Vec<usize> = start_node_dof.chain(end_node_dof).collect();

            let elem_stiffness_matrix = elem.get_global_stiffness_matrix();

            for row in 0..4 {
                for col in 0..4 {
                    println!("{} {}", row, col);
                    global_stiffness_matrix[
                        (dof_indices[row], dof_indices[col])
                        ] += elem_stiffness_matrix[(row, col)];
                }
            }
        }
        global_stiffness_matrix
    }

    fn apply_boundary_conditions(
        &self,
        global_stiffness_matrix: na::DMatrix<f32>
    ) -> na::DMatrix<f32> {
        let rows_to_remove = self.boundary_conditions[0];
        let known_displacements = 
    }

    fn solve_displacements(
        &self, global_stiffness_matrix: na::DMatrix<f32>
    ) -> na::DVector<f32> {
        match global_stiffness_matrix.try_inverse() {
            Some(inverse_global_stiffness_matrix) => inverse_global_stiffness_matrix * self.loads.clone(),
            None => panic!("System linearly dependent -> Insufficient boundary conditions")
        }
    }
    
    fn calc_reaction_forces(&self, global_stiffness_matrix: na::DMatrix<f32>, 
        displacements: na::DVector<f32>
    ) -> na::DVector<f32> {
        global_stiffness_matrix * displacements - self.loads.clone()
    }

    fn calc_strains(&self, elements: Vec<Element1D>, displacements: na::DVector<f32>) -> na::DVector<f32> {
        na::DVector::from_iterator(
            elements.len(), 
            elements.iter().map(|elem| {
                let displacement_1: na::Vector2<f32> = displacements.fixed_rows::<2>(
                    2 * elem.start_node_nr
                ).into();
                let displacement_2: na::Vector2<f32> = displacements.fixed_rows::<2>(
                    2 * elem.end_node_nr
                ).into();
                elem.get_strain(displacement_1, displacement_2)
            })
        )
    }

    fn calc_stresses(&self, strains: na::DVector<f32>) -> na::DVector<f32> {
        na::DVector::from_iterator(strains.len(), strains.iter().enumerate().map(|(i, strain)| {
            strain * self.section_assignments[i].material.elastic_modulus
        }))
    }

    pub fn solve(&self) -> OutputData {
        let elements = self.assemble_elements();
        let global_stiffness_matrix = self.assemble_global_stiffness_matrix(
            &elements
        );

        let (reduced_global_stiffness_matrix, reduced_loads) = apply_boundary_conditions(
            global_stiffness_matrix
        );
        let displacements = self.solve_displacements(global_stiffness_matrix.clone());
        let reaction_forces = self.calc_reaction_forces(
            global_stiffness_matrix, displacements.clone()
        );
        let strains = self.calc_strains(elements, displacements.clone());
        let stresses = self.calc_stresses(strains.clone());

        OutputData {
            displacements: displacements,
            reaction_forces: reaction_forces,
            strains: strains,
            stresses: stresses
        }
    }
}
