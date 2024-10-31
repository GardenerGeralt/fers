use nalgebra as na;

use crate::node::Node;
use crate::section::Section;

pub struct Element1D<'a> {
    pub start_node_nr: usize,
    pub end_node_nr: usize,

    section: Option<&'a Section<'a>>,

    length: f32,
    angle: f32,

    local2global2: na::Matrix2<f32>,
    global2local2: na::Matrix2<f32>,

    local2global4: na::Matrix4<f32>,
    global2local4: na::Matrix4<f32>,
}

impl<'a> Element1D<'a> {
    pub fn new(start_node: &Node, end_node: &Node, section: Option<&'a Section>) -> Element1D<'a> {
        let [start_x, start_y] = start_node.xy();
        let [end_x, end_y] = end_node.xy();
        let x_element = end_x - start_x;
        let y_element = end_y - start_y;
        let length = (x_element.powf(2.0) + y_element.powf(2.0)).sqrt();
        let angle = y_element.atan2(x_element);

        // let local2global = na::Rotation::from_axis_angle(axis, angle);
        let local2global2 = na::matrix![
            angle.cos(), angle.sin();
            -angle.sin(), angle.cos()
        ];
        let global2local2 = local2global2.clone().transpose();

        let local2global4 = na::stack![
            local2global2, na::Matrix2::zeros();
            na::Matrix2::zeros(), local2global2
        ];
        let global2local4 = local2global4.clone().transpose();

        Element1D {
            start_node_nr: start_node.nr,
            end_node_nr: end_node.nr,

            section,

            length,
            angle,

            local2global2,
            global2local2,

            local2global4,
            global2local4,
        }
    }

    pub fn assign_section(&'a mut self, section_assignment: &'a Section) {
        self.section = Some(section_assignment);
    }

    pub fn get_global_stiffness_matrix(&self) -> na::Matrix4<f32> {
        let local_stiffness_matrix = {
            self.section
                .as_ref()
                .unwrap()
                .calc_local_stiffness_matrix(self.length)
        };

        self.local2global4 * &local_stiffness_matrix
    }

    pub fn get_strain(
        &self,
        global_displacement_1: na::Vector2<f32>,
        global_displacement_2: na::Vector2<f32>,
    ) -> f32 {
        let local_displacement_1 = self.global2local2 * &global_displacement_1;
        let local_displacement_2 = self.global2local2 * &global_displacement_2;
        let elongation = local_displacement_2[0] - local_displacement_1[0];
        elongation / self.length
    }
}

pub struct Element2D {
    pub node_a_nr: usize,
    pub node_b_nr: usize,
    pub node_c_nr: usize,
    pub node_d_nr: usize,

    thickness: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{element, material::Material};
    use std::f32::consts::PI;

    #[test]
    fn test_new() {
        let start_node = Node::new(1., 1., 0);
        let end_node = Node::new(2., 2., 0);

        let section_area = 0.01;
        let poisson_ratio = 0.3;
        let elastic_modulus = 70e9;
        let material = Material {
            poisson_ratio,
            elastic_modulus,
        };
        let section = Section {
            section_area,
            material: &material,
        };

        let element1d = Element1D::new(&start_node, &end_node, Some(&section));

        assert_eq!(element1d.start_node_nr, start_node.nr);
        assert_eq!(element1d.end_node_nr, end_node.nr);
        assert_eq!(element1d.length, (2 as f32).sqrt());
        assert_eq!(element1d.angle, PI / 4.);
        // assert_eq!(element1d.local2global2)
    }
}

