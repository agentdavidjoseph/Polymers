use std::f64::consts::PI;
use crate::math::
{
    Math,
    ln,
    ln_sinhc,
    approximate_inverse_langevin,
    factorial
};
use crate::physics::
{
    PLANCK_CONSTANT,
    BOLTZMANN_CONSTANT
};
use crate::physics::single_chain::
{
    Isometric,
    IsometricLegendre
};

pub mod test;

pub struct FJC
{
    pub hinge_mass: f64,
    pub link_length: f64,
    pub number_of_links: u16,
    pub number_of_links_f64: f64,
    pub contour_length: f64,
    pub legendre: FJCLegendre
}

impl Isometric for FJC
{
    fn init(number_of_links: u16, link_length: f64, hinge_mass: f64) -> FJC
    {
        FJC
        {
            hinge_mass,
            link_length,
            number_of_links,
            number_of_links_f64: number_of_links as f64,
            contour_length: (number_of_links as f64)*link_length,
            legendre: FJCLegendre::init(number_of_links, link_length, hinge_mass)
        }
    }
    fn helmholtz_free_energy(&self, end_to_end_length: &f64, temperature: f64) -> f64
    {
        self.helmholtz_free_energy(end_to_end_length, temperature)*self.number_of_links_f64
    }
    fn helmholtz_free_energy_per_link(&self, end_to_end_length: &f64, temperature: f64) -> f64
    {
        self.nondimensional_helmholtz_free_energy_per_link(&(*end_to_end_length/self.contour_length), temperature)*BOLTZMANN_CONSTANT*temperature
    }
    fn relative_helmholtz_free_energy(&self, end_to_end_length: &f64, temperature: f64) -> f64
    {
        self.relative_helmholtz_free_energy_per_link(end_to_end_length, temperature)*self.number_of_links_f64
    }
    fn relative_helmholtz_free_energy_per_link(&self, end_to_end_length: &f64, temperature: f64) -> f64
    {
        self.nondimensional_relative_helmholtz_free_energy_per_link(&(*end_to_end_length/self.contour_length))*BOLTZMANN_CONSTANT*temperature
    }
    fn nondimensional_helmholtz_free_energy(&self, nondimensional_end_to_end_length_per_link: &f64, temperature: f64) -> f64
    {
        -ln(&self.equilibrium_distribution(&(*nondimensional_end_to_end_length_per_link*self.contour_length))) - self.number_of_links_f64*ln(&(8.0*PI.powf(2.0)*self.hinge_mass*self.link_length.powf(2.0)*BOLTZMANN_CONSTANT*temperature/PLANCK_CONSTANT.powf(2.0)))
    }
    fn nondimensional_helmholtz_free_energy_per_link(&self, nondimensional_end_to_end_length_per_link: &f64, temperature: f64) -> f64
    {
        self.nondimensional_helmholtz_free_energy(nondimensional_end_to_end_length_per_link, temperature)/self.number_of_links_f64
    }
    fn nondimensional_relative_helmholtz_free_energy(&self, nondimensional_end_to_end_length_per_link: &f64) -> f64
    {
        ln(&(self.nondimensional_equilibrium_distribution(&1e-8)/self.nondimensional_equilibrium_distribution(nondimensional_end_to_end_length_per_link)))
    }
    fn nondimensional_relative_helmholtz_free_energy_per_link(&self, nondimensional_end_to_end_length_per_link: &f64) -> f64
    {
        self.nondimensional_relative_helmholtz_free_energy(nondimensional_end_to_end_length_per_link)/self.number_of_links_f64
    }
    fn equilibrium_distribution(&self, end_to_end_length: &f64) -> f64
    {
        self.nondimensional_equilibrium_distribution(&(*end_to_end_length/self.contour_length))/self.contour_length.powf(3.0)
    }
    fn nondimensional_equilibrium_distribution(&self, nondimensional_end_to_end_length_per_link: &f64) -> f64
    {
        let mut sum: f64 = 0.0;
        let n = self.number_of_links as u128;
        let p = self.number_of_links_f64 - 2.0;
        let m = -*nondimensional_end_to_end_length_per_link*0.5 + 0.5;
        let k = (self.number_of_links_f64*m).ceil() as u128;
        for s in 0..k
        {
            sum += (-1.0_f64).powf(s as f64)*((factorial(n.into())/factorial(s.into())/factorial((n - s).into())) as f64)*(m - (s as f64)/self.number_of_links_f64).powf(p);
        }
        0.125/PI/nondimensional_end_to_end_length_per_link*(n.pow(n as u32) as f64)/(factorial((n - 2).into()) as f64)*sum
    }
    fn equilibrium_radial_distribution(&self, end_to_end_length: &f64) -> f64
    {
        self.nondimensional_equilibrium_radial_distribution(&(*end_to_end_length/self.contour_length))/self.contour_length
    }
    fn nondimensional_equilibrium_radial_distribution(&self, nondimensional_end_to_end_length_per_link: &f64) -> f64
    {
        let mut sum: f64 = 0.0;
        let n = self.number_of_links as u128;
        let p = self.number_of_links_f64 - 2.0;
        let m = -*nondimensional_end_to_end_length_per_link*0.5 + 0.5;
        let k = (self.number_of_links_f64*m).ceil() as u128;
        for s in 0..k
        {
            sum += (-1.0_f64).powf(s as f64)*((factorial(n.into())/factorial(s.into())/factorial((n - s).into())) as f64)*(m - (s as f64)/self.number_of_links_f64).powf(p);
        }
        0.5*nondimensional_end_to_end_length_per_link*(n.pow(n as u32) as f64)/(factorial((n - 2).into()) as f64)*sum
    }
}

pub struct FJCLegendre
{
    pub hinge_mass: f64,
    pub link_length: f64,
    pub number_of_links: u16,
    pub number_of_links_f64: f64,
    pub contour_length: f64
}

impl IsometricLegendre for FJCLegendre
{
    fn init(number_of_links: u16, link_length: f64, hinge_mass: f64) -> FJCLegendre
    {
        FJCLegendre
        {
            hinge_mass,
            link_length,
            number_of_links,
            number_of_links_f64: number_of_links as f64,
            contour_length: (number_of_links as f64)*link_length
        }
    }
    fn force(&self, end_to_end_length: &f64, temperature: f64) -> f64
    {
        approximate_inverse_langevin(&(*end_to_end_length/self.contour_length))*BOLTZMANN_CONSTANT*temperature/self.link_length
    }
    fn nondimensional_force(&self, nondimensional_end_to_end_length_per_link: &f64) -> f64
    {
        approximate_inverse_langevin(nondimensional_end_to_end_length_per_link)
    }
    fn helmholtz_free_energy(&self, end_to_end_length: &f64, temperature: f64) -> f64
    {
        self.helmholtz_free_energy_per_link(end_to_end_length, temperature)*self.number_of_links_f64
    }
    fn helmholtz_free_energy_per_link(&self, end_to_end_length: &f64, temperature: f64) -> f64
    {
        self.nondimensional_helmholtz_free_energy_per_link(&(*end_to_end_length/self.contour_length), temperature)*BOLTZMANN_CONSTANT*temperature
    }
    fn relative_helmholtz_free_energy(&self, end_to_end_length: &f64, temperature: f64) -> f64
    {
        self.relative_helmholtz_free_energy_per_link(end_to_end_length, temperature)*self.number_of_links_f64
    }
    fn relative_helmholtz_free_energy_per_link(&self, end_to_end_length: &f64, temperature: f64) -> f64
    {
        self.nondimensional_relative_helmholtz_free_energy_per_link(&(*end_to_end_length/self.contour_length))*BOLTZMANN_CONSTANT*temperature
    }
    fn nondimensional_helmholtz_free_energy(&self, nondimensional_end_to_end_length_per_link: &f64, temperature: f64) -> f64
    {
        self.nondimensional_helmholtz_free_energy_per_link(nondimensional_end_to_end_length_per_link, temperature)*self.number_of_links_f64
    }
    fn nondimensional_helmholtz_free_energy_per_link(&self, nondimensional_end_to_end_length_per_link: &f64, temperature: f64) -> f64
    {
        self.nondimensional_relative_helmholtz_free_energy_per_link(nondimensional_end_to_end_length_per_link) - ln(&(*nondimensional_end_to_end_length_per_link*0.0 + 8.0*PI.powf(2.0)*self.hinge_mass*self.link_length.powf(2.0)*BOLTZMANN_CONSTANT*temperature/PLANCK_CONSTANT.powf(2.0)))
    }
    fn nondimensional_relative_helmholtz_free_energy(&self, nondimensional_end_to_end_length_per_link: &f64) -> f64
    {
        self.nondimensional_relative_helmholtz_free_energy_per_link(nondimensional_end_to_end_length_per_link)*self.number_of_links_f64
    }
    fn nondimensional_relative_helmholtz_free_energy_per_link(&self, nondimensional_end_to_end_length_per_link: &f64) -> f64
    {
        let nondimensional_force = self.nondimensional_force(nondimensional_end_to_end_length_per_link);
        nondimensional_force**nondimensional_end_to_end_length_per_link - ln_sinhc(&nondimensional_force)
    }
}
