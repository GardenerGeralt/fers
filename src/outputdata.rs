use nalgebra as na;

pub struct OutputData {
    pub displacements: na::DVector<f32>,
    pub reaction_forces: na::DVector<f32>,
    pub strains: na::DVector<f32>,
    pub stresses: na::DVector<f32>
}