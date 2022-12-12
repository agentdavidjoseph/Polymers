pub mod test;
use std::f64::consts::PI;
use crate::physics::
{
    PLANCK_CONSTANT,
    BOLTZMANN_CONSTANT
};
use crate::physics::single_chain::fjc::ZERO;
pub struct FJC
{
    pub hinge_mass: f64,
    pub link_length: f64,
    pub number_of_links: u8,
    pub number_of_links_f64: f64,
    pub contour_length: f64
}
use super::WeakPotential;
impl WeakPotential for FJC
{
    fn init(number_of_links: u8, link_length: f64, hinge_mass: f64) -> FJC
    {
        FJC
        {
            hinge_mass,
            link_length,
            number_of_links,
            number_of_links_f64: number_of_links as f64,
            contour_length: (number_of_links as f64)*link_length
        }
    }
    fn force(&self, potential_distance: &f64, potential_stiffness: &f64) -> f64
    {
        potential_stiffness*potential_distance
    }
    fn nondimensional_force(&self, nondimensional_potential_distance: &f64, nondimensional_potential_stiffness: &f64) -> f64
    {
        nondimensional_potential_stiffness*nondimensional_potential_distance/self.number_of_links_f64
    }
    fn end_to_end_length(&self, potential_distance: &f64, potential_stiffness: &f64, temperature: &f64) -> f64
    {
        let nondimensional_force = potential_stiffness*potential_distance*self.link_length/BOLTZMANN_CONSTANT/temperature;
        self.contour_length*(1.0/nondimensional_force.tanh() - 1.0/nondimensional_force) - potential_stiffness*self.contour_length.powf(2.0)*self.link_length/BOLTZMANN_CONSTANT/temperature*((1.0/nondimensional_force.tanh() - 1.0/nondimensional_force)*(nondimensional_force.powf(-2.0) - (nondimensional_force.sinh()).powf(-2.0)) + ((nondimensional_force.sinh()).powf(-2.0)/nondimensional_force.tanh() - nondimensional_force.powf(-3.0))/self.number_of_links_f64)
    }
    fn end_to_end_length_per_link(&self, potential_distance: &f64, potential_stiffness: &f64, temperature: &f64) -> f64
    {
        let nondimensional_force = potential_stiffness*potential_distance*self.link_length/BOLTZMANN_CONSTANT/temperature;
        self.link_length*(1.0/nondimensional_force.tanh() - 1.0/nondimensional_force) - potential_stiffness*self.number_of_links_f64*self.link_length.powf(3.0)/BOLTZMANN_CONSTANT/temperature*((1.0/nondimensional_force.tanh() - 1.0/nondimensional_force)*(nondimensional_force.powf(-2.0) - (nondimensional_force.sinh()).powf(-2.0)) + ((nondimensional_force.sinh()).powf(-2.0)/nondimensional_force.tanh() - nondimensional_force.powf(-3.0))/self.number_of_links_f64)
    }
    fn nondimensional_end_to_end_length(&self, nondimensional_potential_distance: &f64, nondimensional_potential_stiffness: &f64) -> f64
    {
        let nondimensional_force = nondimensional_potential_stiffness*nondimensional_potential_distance/self.number_of_links_f64;
        self.number_of_links_f64*(1.0/nondimensional_force.tanh() - 1.0/nondimensional_force) - nondimensional_potential_stiffness*((1.0/nondimensional_force.tanh() - 1.0/nondimensional_force)*(nondimensional_force.powf(-2.0) - (nondimensional_force.sinh()).powf(-2.0)) + ((nondimensional_force.sinh()).powf(-2.0)/nondimensional_force.tanh() - nondimensional_force.powf(-3.0))/self.number_of_links_f64)
    }
    fn nondimensional_end_to_end_length_per_link(&self, nondimensional_potential_distance: &f64, nondimensional_potential_stiffness: &f64) -> f64
    {
        let nondimensional_force = nondimensional_potential_stiffness*nondimensional_potential_distance/self.number_of_links_f64;
        1.0/nondimensional_force.tanh() - 1.0/nondimensional_force - nondimensional_potential_stiffness*((1.0/nondimensional_force.tanh() - 1.0/nondimensional_force)*(nondimensional_force.powf(-2.0) - (nondimensional_force.sinh()).powf(-2.0)) + ((nondimensional_force.sinh()).powf(-2.0)/nondimensional_force.tanh() - nondimensional_force.powf(-3.0))/self.number_of_links_f64)/self.number_of_links_f64
    }
    fn gibbs_free_energy(&self, potential_distance: &f64, potential_stiffness: &f64, temperature: &f64) -> f64
    {
        let nondimensional_force = potential_stiffness*potential_distance*self.link_length/BOLTZMANN_CONSTANT/temperature;
        -self.number_of_links_f64*BOLTZMANN_CONSTANT*temperature*(nondimensional_force.sinh()/nondimensional_force).ln() + 0.5*potential_stiffness*self.contour_length.powf(2.0)*((1.0/nondimensional_force.tanh() - 1.0/nondimensional_force).powf(2.0) + (nondimensional_force.powf(-2.0) - (nondimensional_force.sinh()).powf(-2.0))/self.number_of_links_f64) - self.number_of_links_f64*BOLTZMANN_CONSTANT*temperature*(8.0*PI.powf(2.0)*self.hinge_mass*self.link_length.powf(2.0)*BOLTZMANN_CONSTANT*temperature/PLANCK_CONSTANT.powf(2.0)).ln()
    }
    fn gibbs_free_energy_per_link(&self, potential_distance: &f64, potential_stiffness: &f64, temperature: &f64) -> f64
    {
        let nondimensional_force = potential_stiffness*potential_distance*self.link_length/BOLTZMANN_CONSTANT/temperature;
        -BOLTZMANN_CONSTANT*temperature*(nondimensional_force.sinh()/nondimensional_force).ln() + 0.5*potential_stiffness*self.contour_length.powf(2.0)*((1.0/nondimensional_force.tanh() - 1.0/nondimensional_force).powf(2.0) + (nondimensional_force.powf(-2.0) - (nondimensional_force.sinh()).powf(-2.0))/self.number_of_links_f64)/self.number_of_links_f64 - BOLTZMANN_CONSTANT*temperature*(8.0*PI.powf(2.0)*self.hinge_mass*self.link_length.powf(2.0)*BOLTZMANN_CONSTANT*temperature/PLANCK_CONSTANT.powf(2.0)).ln()
    }
    fn relative_gibbs_free_energy(&self, potential_distance: &f64, potential_stiffness: &f64, temperature: &f64) -> f64
    {
        self.gibbs_free_energy(potential_distance, potential_stiffness, temperature) - self.gibbs_free_energy(&(ZERO*self.number_of_links_f64*self.link_length), potential_stiffness, temperature)
    }
    fn relative_gibbs_free_energy_per_link(&self, potential_distance: &f64, potential_stiffness: &f64, temperature: &f64) -> f64
    {
        self.gibbs_free_energy_per_link(potential_distance, potential_stiffness, temperature) - self.gibbs_free_energy_per_link(&(ZERO*self.number_of_links_f64*self.link_length), potential_stiffness, temperature)
    }
    fn nondimensional_gibbs_free_energy(&self, nondimensional_potential_distance: &f64, nondimensional_potential_stiffness: &f64, temperature: &f64) -> f64
    {
        let nondimensional_force = nondimensional_potential_stiffness*nondimensional_potential_distance/self.number_of_links_f64;
        -self.number_of_links_f64*(nondimensional_force.sinh()/nondimensional_force).ln() + 0.5*nondimensional_potential_stiffness*((1.0/nondimensional_force.tanh() - 1.0/nondimensional_force).powf(2.0) + (nondimensional_force.powf(-2.0) - (nondimensional_force.sinh()).powf(-2.0))/self.number_of_links_f64) - self.number_of_links_f64*(8.0*PI.powf(2.0)*self.hinge_mass*self.link_length.powf(2.0)*BOLTZMANN_CONSTANT*temperature/PLANCK_CONSTANT.powf(2.0)).ln()
    }
    fn nondimensional_gibbs_free_energy_per_link(&self, nondimensional_potential_distance: &f64, nondimensional_potential_stiffness: &f64, temperature: &f64) -> f64
    {
        let nondimensional_force = nondimensional_potential_stiffness*nondimensional_potential_distance/self.number_of_links_f64;
        -(nondimensional_force.sinh()/nondimensional_force).ln() + 0.5*nondimensional_potential_stiffness*((1.0/nondimensional_force.tanh() - 1.0/nondimensional_force).powf(2.0) + (nondimensional_force.powf(-2.0) - (nondimensional_force.sinh()).powf(-2.0))/self.number_of_links_f64)/self.number_of_links_f64 - (8.0*PI.powf(2.0)*self.hinge_mass*self.link_length.powf(2.0)*BOLTZMANN_CONSTANT*temperature/PLANCK_CONSTANT.powf(2.0)).ln()
    }
    fn nondimensional_relative_gibbs_free_energy(&self, nondimensional_potential_distance: &f64, nondimensional_potential_stiffness: &f64) -> f64
    {
        self.nondimensional_gibbs_free_energy(nondimensional_potential_distance, nondimensional_potential_stiffness, &300.0) - self.nondimensional_gibbs_free_energy(&ZERO, nondimensional_potential_stiffness, &300.0)
    }
    fn nondimensional_relative_gibbs_free_energy_per_link(&self, nondimensional_potential_distance: &f64, nondimensional_potential_stiffness: &f64) -> f64
    {
        self.nondimensional_gibbs_free_energy_per_link(nondimensional_potential_distance, nondimensional_potential_stiffness, &300.0) - self.nondimensional_gibbs_free_energy_per_link(&ZERO, nondimensional_potential_stiffness, &300.0)
    }
}
