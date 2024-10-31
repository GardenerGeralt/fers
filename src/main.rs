mod element;
mod material;
mod model;
mod node;
mod outputdata;
mod section;

use nalgebra as na;

fn main() {
    let poisson_ratio = 0.3;
    let elastic_modulus = 70e3;
    let aluminium = material::Material {
        poisson_ratio,
        elastic_modulus,
    };

    let boundary_conditions = vec![vec![2, 5, 7, 9, 10], vec![0, 0, 0, 0, 0]];
    let loads: na::DVector<f32> = na::dvector![0., 0., 4250., 0., -3000., -5000.];

    let connectivity_matrix = vec![
        vec![0, 1],
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![0, 4],
        vec![0, 3],
        vec![1, 3],
    ];
    let node_coordinates: Vec<na::Vector2<f32>> = vec![
        na::vector![0., 0.],
        na::vector![150., 150.],
        na::vector![500., 200.],
        na::vector![500., 0.],
        na::vector![500., -100.],
    ];
    let nodes = (0..node_coordinates.len())
        .map(|node_nr| {
            node::Node::new(
                node_coordinates[node_nr][0],
                node_coordinates[node_nr][1],
                node_nr,
            )
        })
        .collect();

    let beam_areas = na::matrix![20., 20., 30., 30., 20., 30., 30.];
    let beam_sections = beam_areas
        .clone()
        .iter()
        .map(|area| section::Section {
            section_area: *area,
            material: &aluminium,
        })
        .collect();

    let my_model = model::Model::new(
        nodes,
        connectivity_matrix,
        beam_sections,
        boundary_conditions,
        loads,
    );
    let output = my_model.solve();
}
