use crate::math::Math;

pub mod test;
pub mod fjc;

pub trait Isometric
{
    fn init(number_of_links: u16, link_length: f64, hinge_mass: f64) -> Self;
    fn helmholtz_free_energy(&self, end_to_end_length: &f64, temperature: f64) -> f64;
    fn helmholtz_free_energy_per_link(&self, end_to_end_length: &f64, temperature: f64) -> f64;
    fn relative_helmholtz_free_energy(&self, end_to_end_length: &f64, temperature: f64) -> f64;
    fn relative_helmholtz_free_energy_per_link(&self, end_to_end_length: &f64, temperature: f64) -> f64;
    fn nondimensional_helmholtz_free_energy(&self, nondimensional_end_to_end_length_per_link: &f64, temperature: f64) -> f64;
    fn nondimensional_helmholtz_free_energy_per_link(&self, nondimensional_end_to_end_length_per_link: &f64, temperature: f64) -> f64;
    fn nondimensional_relative_helmholtz_free_energy(&self, nondimensional_end_to_end_length_per_link: &f64) -> f64;
    fn nondimensional_relative_helmholtz_free_energy_per_link(&self, end_to_end_length: &f64) -> f64;
    fn equilibrium_distribution(&self, end_to_end_length: &f64) -> f64;
    fn nondimensional_equilibrium_distribution(&self, end_to_end_length: &f64) -> f64;
    fn equilibrium_radial_distribution(&self, end_to_end_length: &f64) -> f64;
    fn nondimensional_equilibrium_radial_distribution(&self, nondimensional_end_to_end_length_per_link: &f64) -> f64;
}

pub trait Isotensional
{
    fn init(number_of_links: u16, link_length: f64, hinge_mass: f64) -> Self;
    fn end_to_end_length<T>(&self, force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn end_to_end_length_per_link<T>(&self, force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn nondimensional_end_to_end_length<T>(&self, nondimensional_force: &T) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn nondimensional_end_to_end_length_per_link<T>(&self, nondimensional_force: &T) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn gibbs_free_energy<T>(&self, force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn gibbs_free_energy_per_link<T>(&self, force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn relative_gibbs_free_energy<T>(&self, force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn relative_gibbs_free_energy_per_link<T>(&self, force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn nondimensional_gibbs_free_energy<T>(&self, nondimensional_force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn nondimensional_gibbs_free_energy_per_link<T>(&self, nondimensional_force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn nondimensional_relative_gibbs_free_energy<T>(&self, nondimensional_force: &T) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn nondimensional_relative_gibbs_free_energy_per_link<T>(&self, nondimensional_force: &T) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
}

pub trait IsometricLegendre
{
    fn init(number_of_links: u16, link_length: f64, hinge_mass: f64) -> Self;
    fn force(&self, end_to_end_length: &f64, temperature: f64) -> f64;
    fn nondimensional_force(&self, nondimensional_end_to_end_length_per_link: &f64) -> f64;
    fn helmholtz_free_energy(&self, end_to_end_length: &f64, temperature: f64) -> f64;
    fn helmholtz_free_energy_per_link(&self, end_to_end_length: &f64, temperature: f64) -> f64;
    fn relative_helmholtz_free_energy(&self, end_to_end_length: &f64, temperature: f64) -> f64;
    fn relative_helmholtz_free_energy_per_link(&self, end_to_end_length: &f64, temperature: f64) -> f64;
    fn nondimensional_helmholtz_free_energy(&self, nondimensional_end_to_end_length_per_link: &f64, temperature: f64) -> f64;
    fn nondimensional_helmholtz_free_energy_per_link(&self, nondimensional_end_to_end_length_per_link: &f64, temperature: f64) -> f64;
    fn nondimensional_relative_helmholtz_free_energy(&self, nondimensional_end_to_end_length_per_link: &f64) -> f64;
    fn nondimensional_relative_helmholtz_free_energy_per_link(&self, nondimensional_end_to_end_length_per_link: &f64) -> f64;
}

pub trait IsotensionalLegendre
{
    fn init(number_of_links: u16, link_length: f64, hinge_mass: f64) -> Self;
    fn helmholtz_free_energy<T>(&self, force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn helmholtz_free_energy_per_link<T>(&self, force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn relative_helmholtz_free_energy<T>(&self, force: &T, temperature: f64) -> T
    where T:
            Math<T> +
            std::marker::Copy +
            std::ops::Neg<Output = T> +
            std::ops::Mul<T, Output = T> +
            std::ops::Sub<T, Output = T> +
            std::ops::Add<f64, Output = T> +
            std::ops::Div<f64, Output = T> +
            std::ops::Mul<f64, Output = T>;
    fn relative_helmholtz_free_energy_per_link<T>(&self, force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn nondimensional_helmholtz_free_energy<T>(&self, nondimensional_force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn nondimensional_helmholtz_free_energy_per_link<T>(&self, nondimensional_force: &T, temperature: f64) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn nondimensional_relative_helmholtz_free_energy<T>(&self, nondimensional_force: &T) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
    fn nondimensional_relative_helmholtz_free_energy_per_link<T>(&self, nondimensional_force: &T) -> T
    where T:
        Math<T> +
        std::marker::Copy +
        std::ops::Neg<Output = T> +
        std::ops::Mul<T, Output = T> +
        std::ops::Sub<T, Output = T> +
        std::ops::Add<f64, Output = T> +
        std::ops::Div<f64, Output = T> +
        std::ops::Mul<f64, Output = T>;
}
