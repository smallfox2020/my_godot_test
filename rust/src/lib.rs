#![deny(missing_docs)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
// Precision casts will happen frequently in game logic;
// checking them is unnecessary.
#![allow(clippy::cast_possible_truncation)]

//! Godot extension for the Cancerette Hacker

use godot::prelude::*;

pub(crate) mod player;

struct CHExtension;

#[gdextension]
unsafe impl ExtensionLibrary for CHExtension {}
