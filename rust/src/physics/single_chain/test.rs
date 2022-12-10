#![cfg(test)]
pub struct Parameters
{
    pub abs_tol: f64,
    pub rel_tol: f64,
    pub rel_tol_thermodynamic_limit: f64,
    pub log_log_tol: f64,
    pub log_log_scale: f64,
    pub number_of_loops: u32,
    pub hinge_mass_reference: f64,
    pub hinge_mass_scale: f64,
    pub link_length_reference: f64,
    pub link_length_scale: f64,
    pub number_of_links_minimum: u8,
    pub number_of_links_maximum: u8,
    pub nondimensional_end_to_end_length_per_link_reference: f64,
    pub nondimensional_end_to_end_length_per_link_scale: f64,
    pub nondimensional_force_reference: f64,
    pub nondimensional_force_scale: f64,
    pub nondimensional_potential_distance_reference: f64,
    pub nondimensional_potential_distance_scale: f64,
    pub nondimensional_potential_distance_small: f64,
    pub nondimensional_potential_distance_large_1: f64,
    pub nondimensional_potential_distance_large_2: f64,
    pub nondimensional_potential_stiffness_reference: f64,
    pub nondimensional_potential_stiffness_scale: f64,
    pub nondimensional_potential_stiffness_small: f64,
    pub nondimensional_potential_stiffness_large: f64,
    pub temperature_reference: f64,
    pub temperature_scale: f64,
}
impl Default for Parameters
{
    fn default() -> Self
    {
        Self
        {
            abs_tol: 1e-8,
            rel_tol: 1e-6,
            rel_tol_thermodynamic_limit: 1e-1,
            log_log_tol: 5e-2,
            log_log_scale: 12e-1,
            number_of_loops: 8,
            hinge_mass_reference: 1e0,
            hinge_mass_scale: 1e0,
            link_length_reference: 1e0,
            link_length_scale: 1e0,
            number_of_links_minimum: 5,
            number_of_links_maximum: 25,
            nondimensional_end_to_end_length_per_link_reference: 5e-1,
            nondimensional_end_to_end_length_per_link_scale: 99e-2,
            nondimensional_force_reference: 5e1,
            nondimensional_force_scale: 1e2,
            nondimensional_potential_distance_reference: 1e0,
            nondimensional_potential_distance_scale: 2e0,
            nondimensional_potential_distance_small: 25e-2,
            nondimensional_potential_distance_large_1: 1e0,
            nondimensional_potential_distance_large_2: 1.25e0,
            nondimensional_potential_stiffness_reference: 5e1,
            nondimensional_potential_stiffness_scale: 1e2,
            nondimensional_potential_stiffness_small: 1e-2,
            nondimensional_potential_stiffness_large: 1e2,
            temperature_reference: 3e2,
            temperature_scale: 1e2,
        }
    }
}
