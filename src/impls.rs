use crate::{NoUnit, SIUnit, Value};

use core::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use self::sealed::Unit;

pub(crate) mod sealed {
    pub trait Unit {}
}

impl<
        const SECOND: i32,
        const METRE: i32,
        const KILOGRAM: i32,
        const AMPERE: i32,
        const KELVIN: i32,
        const MOL: i32,
        const CANDELA: i32,
    > sealed::Unit
    for SIUnit<{ SECOND }, { METRE }, { KILOGRAM }, { AMPERE }, { KELVIN }, { MOL }, { CANDELA }>
{
}
struct Help<const V: i32>;

impl<
        const SECOND1: i32,
        const METRE1: i32,
        const KILOGRAM1: i32,
        const AMPERE1: i32,
        const KELVIN1: i32,
        const MOL1: i32,
        const CANDELA1: i32,
        const SECOND2: i32,
        const METRE2: i32,
        const KILOGRAM2: i32,
        const AMPERE2: i32,
        const KELVIN2: i32,
        const MOL2: i32,
        const CANDELA2: i32,
    >
    Mul<
        SIUnit<
            { SECOND2 },
            { METRE2 },
            { KILOGRAM2 },
            { AMPERE2 },
            { KELVIN2 },
            { MOL2 },
            { CANDELA2 },
        >,
    >
    for SIUnit<
        { SECOND1 },
        { METRE1 },
        { KILOGRAM1 },
        { AMPERE1 },
        { KELVIN1 },
        { MOL1 },
        { CANDELA1 },
    >
where
    Help<{ SECOND1 + SECOND2 }>:,
    Help<{ METRE1 + METRE2 }>:,
    Help<{ KILOGRAM1 + KILOGRAM2 }>:,
    Help<{ AMPERE1 + AMPERE2 }>:,
    Help<{ KELVIN1 + KELVIN2 }>:,
    Help<{ MOL1 + MOL2 }>:,
    Help<{ CANDELA1 + CANDELA2 }>:,
{
    type Output = SIUnit<
        { SECOND1 + SECOND2 },
        { METRE1 + METRE2 },
        { KILOGRAM1 + KILOGRAM2 },
        { AMPERE1 + AMPERE2 },
        { KELVIN1 + KELVIN2 },
        { MOL1 + MOL2 },
        { CANDELA1 + CANDELA2 },
    >;

    fn mul(
        self,
        _: SIUnit<
            { SECOND2 },
            { METRE2 },
            { KILOGRAM2 },
            { AMPERE2 },
            { KELVIN2 },
            { MOL2 },
            { CANDELA2 },
        >,
    ) -> Self::Output {
        SIUnit
    }
}

impl<
        const SECOND1: i32,
        const METRE1: i32,
        const KILOGRAM1: i32,
        const AMPERE1: i32,
        const KELVIN1: i32,
        const MOL1: i32,
        const CANDELA1: i32,
        const SECOND2: i32,
        const METRE2: i32,
        const KILOGRAM2: i32,
        const AMPERE2: i32,
        const KELVIN2: i32,
        const MOL2: i32,
        const CANDELA2: i32,
    >
    Div<
        SIUnit<
            { SECOND2 },
            { METRE2 },
            { KILOGRAM2 },
            { AMPERE2 },
            { KELVIN2 },
            { MOL2 },
            { CANDELA2 },
        >,
    >
    for SIUnit<
        { SECOND1 },
        { METRE1 },
        { KILOGRAM1 },
        { AMPERE1 },
        { KELVIN1 },
        { MOL1 },
        { CANDELA1 },
    >
where
    Help<{ SECOND1 - SECOND2 }>:,
    Help<{ METRE1 - METRE2 }>:,
    Help<{ KILOGRAM1 - KILOGRAM2 }>:,
    Help<{ AMPERE1 - AMPERE2 }>:,
    Help<{ KELVIN1 - KELVIN2 }>:,
    Help<{ MOL1 - MOL2 }>:,
    Help<{ CANDELA1 - CANDELA2 }>:,
{
    type Output = SIUnit<
        { SECOND1 - SECOND2 },
        { METRE1 - METRE2 },
        { KILOGRAM1 - KILOGRAM2 },
        { AMPERE1 - AMPERE2 },
        { KELVIN1 - KELVIN2 },
        { MOL1 - MOL2 },
        { CANDELA1 - CANDELA2 },
    >;

    fn div(
        self,
        _: SIUnit<
            { SECOND2 },
            { METRE2 },
            { KILOGRAM2 },
            { AMPERE2 },
            { KELVIN2 },
            { MOL2 },
            { CANDELA2 },
        >,
    ) -> Self::Output {
        SIUnit
    }
}

impl<A, B, U: Unit> Add<Value<A, U>> for Value<B, U>
where
    B: Add<A>,
{
    type Output = Value<B::Output, U>;

    fn add(self, rhs: Value<A, U>) -> Self::Output {
        Value {
            value: self.value.add(rhs.value),
            unit: PhantomData,
        }
    }
}

impl<A, B, U: Unit> AddAssign<Value<A, U>> for Value<B, U>
where
    B: AddAssign<A>,
{
    fn add_assign(&mut self, rhs: Value<A, U>) {
        self.value.add_assign(rhs.value);
    }
}

impl<A, B, U: Unit> Sub<Value<A, U>> for Value<B, U>
where
    B: Sub<A>,
{
    type Output = Value<B::Output, U>;

    fn sub(self, rhs: Value<A, U>) -> Self::Output {
        Value {
            value: self.value.sub(rhs.value),
            unit: PhantomData,
        }
    }
}

impl<A, B, U: Unit> SubAssign<Value<A, U>> for Value<B, U>
where
    B: SubAssign<A>,
{
    fn sub_assign(&mut self, rhs: Value<A, U>) {
        self.value.sub_assign(rhs.value);
    }
}

impl<A, B, U1: Unit, U2: Unit> Mul<Value<A, U1>> for Value<B, U2>
where
    B: Mul<A>,
    U2: Mul<U1>,
    U2::Output: Unit,
{
    type Output = Value<B::Output, U2::Output>;

    fn mul(self, rhs: Value<A, U1>) -> Self::Output {
        Value {
            value: self.value.mul(rhs.value),
            unit: PhantomData,
        }
    }
}

impl<A, B, U: Unit> MulAssign<Value<A, NoUnit>> for Value<B, U>
where
    B: MulAssign<A>,
{
    fn mul_assign(&mut self, rhs: Value<A, NoUnit>) {
        self.value.mul_assign(rhs.value);
    }
}

impl<A, B, U1: Unit, U2: Unit> Div<Value<A, U1>> for Value<B, U2>
where
    B: Div<A>,
    U2: Div<U1>,
    U2::Output: Unit,
{
    type Output = Value<B::Output, U2::Output>;

    fn div(self, rhs: Value<A, U1>) -> Self::Output {
        Value {
            value: self.value.div(rhs.value),
            unit: PhantomData,
        }
    }
}

impl<A, B, U: Unit> DivAssign<Value<A, NoUnit>> for Value<B, U>
where
    B: DivAssign<A>,
{
    fn div_assign(&mut self, rhs: Value<A, NoUnit>) {
        self.value.div_assign(rhs.value);
    }
}

impl<T, U: Unit> From<T> for Value<T, U> {
    fn from(value: T) -> Self {
        Value {
            value,
            unit: PhantomData,
        }
    }
}
