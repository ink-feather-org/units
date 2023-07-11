#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::{
    marker::PhantomData,
    ops::{Div, Mul},
};
mod impls;

use impls::sealed::Unit;
pub struct SIUnit<
    const SECOND: i32,
    const METRE: i32,
    const KILOGRAM: i32,
    const AMPERE: i32,
    const KELVIN: i32,
    const MOL: i32,
    const CANDELA: i32,
>;

pub type Second = SIUnit<1, 0, 0, 0, 0, 0, 0>;
pub type Meter = SIUnit<0, 1, 0, 0, 0, 0, 0>;
pub type Kilogram = SIUnit<0, 0, 1, 0, 0, 0, 0>;
pub type Ampere = SIUnit<0, 0, 0, 1, 0, 0, 0>;
pub type Kelvin = SIUnit<0, 0, 0, 0, 1, 0, 0>;
pub type Mol = SIUnit<0, 0, 0, 0, 0, 1, 0>;
pub type Candela = SIUnit<0, 0, 0, 0, 0, 0, 1>;

pub type MetersPerSecond = <Meter as Div<Second>>::Output;
pub type Coulomb = <Ampere as Mul<Second>>::Output;

pub struct Value<T, U: Unit> {
    pub value: T,
    unit: PhantomData<U>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let x: Value<f32, Second> = 5.0.into();
        let y: Value<f32, Second> = 5.0.into();

        let z: Value<f32, Second> = x + y;
        assert_eq!(z.value, 10.0);
    }
    #[test]
    fn sub() {
        let x: Value<f32, Second> = 5.0.into();
        let y: Value<f32, Second> = (-5.0).into();

        let z: Value<f32, Second> = x - y;
        assert_eq!(z.value, 10.0);
    }
    #[test]
    fn mul() {
        let x: Value<i32, Ampere> = 5.into();
        let y: Value<i32, Second> = 5.into();

        let z: Value<i32, Coulomb> = x * y;
        assert_eq!(z.value, 25);
    }
    #[test]
    fn div() {
        let x: Value<i32, Meter> = 5.into();
        let y: Value<i32, Second> = 5.into();

        let z: Value<i32, MetersPerSecond> = x / y;
        assert_eq!(z.value, 1);
    }
}
