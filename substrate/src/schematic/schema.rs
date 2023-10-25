//! Traits and types for specifying formats for storing Substrate schematics.
use std::any::Any;

pub trait Schema:
    scir::schema::Schema<Primitive = <Self as Schema>::Primitive> + Send + Sync + Any
{
    type Primitive: Primitive;
}

impl<T: scir::schema::Schema<Primitive = impl Primitive> + Send + Sync + Any> Schema for T {
    type Primitive = <T as scir::schema::Schema>::Primitive;
}

pub trait Primitive: Clone + Send + Sync + Any {}

impl<T: Clone + Send + Sync + Any> Primitive for T {}

pub trait FromSchema<S: Schema>: Schema + scir::schema::FromSchema<S> {}

impl<S1: Schema, S2: Schema + scir::schema::FromSchema<S1>> FromSchema<S1> for S2 {}
