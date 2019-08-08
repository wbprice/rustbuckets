use crate::{entities::{Coordinates, Heading}};

#[derive(Debug)]
pub struct Ship {
    origin: Coordinates,
    heading: Heading
}