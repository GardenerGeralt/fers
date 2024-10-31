use nalgebra as na;

use crate::material::Material;

pub struct Section<'a> {
    pub section_area: f32,
    pub material: &'a Material
}

impl Section<'_> {
    pub fn calc_local_stiffness_matrix(&self, element_length: f32) -> na::Matrix4<f32> {
        let equivalent_stiffness = self.material.elastic_modulus * self.section_area / element_length;
        
        na::matrix![
            equivalent_stiffness, 0., -equivalent_stiffness, 0.;
            0., 0., 0., 0.;
            -equivalent_stiffness, 0., equivalent_stiffness, 0.;
            0., 0., 0., 0.
        ]
    }
}
