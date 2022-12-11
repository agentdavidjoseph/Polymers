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
    pub well_width_reference: f64,
    pub well_width_scale: f64,
    pub nondimensional_well_width_small: f64,
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
            well_width_reference: 99e-2,
            well_width_scale: 5e-1,
            nondimensional_well_width_small: 1e-2,
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
mod fjc
{
    mod swfjc
    {
        use rand::Rng;
        use crate::physics::single_chain::test::Parameters;
        use crate::physics::single_chain::fjc::FJC;
        use crate::physics::single_chain::fjc::thermodynamics::Isotensional as IsotensionalFJC;
        use crate::physics::single_chain::swfjc::SWFJC;
        use crate::physics::single_chain::swfjc::thermodynamics::Isotensional as IsotensionalSWFJC;
        #[test]
        fn nondimensional_end_to_end_length_per_link()
        {
            let mut rng = rand::thread_rng();
            let parameters = Parameters::default();
            for _ in 0..parameters.number_of_loops
            {
                let number_of_links: u8 = rng.gen_range(parameters.number_of_links_minimum..parameters.number_of_links_maximum);
                let hinge_mass = parameters.hinge_mass_reference + parameters.hinge_mass_scale*(0.5 - rng.gen::<f64>());
                let link_length = rng.gen::<f64>();
                let well_width = parameters.nondimensional_well_width_small*link_length;
                let nondimensional_force = parameters.nondimensional_force_reference + parameters.nondimensional_force_scale*(0.5 - rng.gen::<f64>());
                let fjc = FJC::init(parameters.number_of_links_minimum, parameters.link_length_reference, parameters.hinge_mass_reference);
                let swfjc = SWFJC::init(number_of_links, link_length, hinge_mass, well_width);
                let nondimensional_end_to_end_length_per_link_fjc = fjc.thermodynamics.isotensional.nondimensional_end_to_end_length_per_link(&nondimensional_force);
                let nondimensional_end_to_end_length_per_link_swfjc = swfjc.thermodynamics.isotensional.nondimensional_end_to_end_length_per_link(&nondimensional_force);
                let residual = &nondimensional_end_to_end_length_per_link_fjc - &nondimensional_end_to_end_length_per_link_swfjc;
                assert!(residual.abs() <= parameters.nondimensional_well_width_small);
            }
        }
    }
}
