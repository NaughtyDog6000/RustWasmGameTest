use std::convert::Infallible;
use std::fmt;

use bracket_terminal::console;
use serde_json;
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator, SerializeComponents, DeserializeComponents, MarkedBuilder};

use crate::components::position::{FloatPosition, IntPosition};
use crate::components::velocity::Velocity;
use crate::NetworkSync;


macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<NoError, SimpleMarker<SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

#[derive(Debug)]
enum Combined {
    Serde(serde_json::Error),
}

// Implementing the required `Display`-trait, by matching the `Combined` enum,
// allowing different error types to be displayed.
impl fmt::Display for Combined {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Combined::Serde(ref e) => write!(f, "{}", e),
        }
    }
}

// This returns the `ron::ser:Error` in form of the `Combined` enum, which can
// then be matched and displayed accordingly.
impl From<serde_json::Error> for Combined {
    fn from(x: serde_json::error::Error) -> Self {
        Combined::Serde(x)
    }
}

// This cannot be called.
impl From<Infallible> for Combined {
    fn from(e: Infallible) -> Self {
        match e {}
    }
}


pub struct Serialize;

impl<'a> System<'a> for Serialize {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Velocity>, 
        ReadStorage<'a, FloatPosition>,
        ReadStorage<'a, IntPosition>,
        ReadStorage<'a, SimpleMarker<NetworkSync>>,
    );

    fn run(&mut self, (
        entities,
        Velocities,
        FPositions,
        IPositions,
        markers
        ): Self::SystemData) {

        let mut buf = Vec::new();
        let mut ser = serde_json::ser::Serializer::pretty(&mut buf);

        SerializeComponents::<Infallible,SimpleMarker<NetworkSync>>::serialize(
            &(&Velocities, &FPositions, &IPositions),
             &entities, 
             &markers, 
             &mut ser
        ).unwrap_or_else(|e| console::log(format!("Error: {}", e)));
        
        // TODO!() here we write to the file  
        console::log(format!("{}", String::from_utf8(buf).expect("should be utf-8")));
    }
}


// if on the web, we cannot save/load yet.
// #[cfg(target_arch = "wasm32")]
// pub fn save_game(_ecs : &mut World) {

// }

// #[cfg(not(target_arch = "wasm32"))]
// pub fn save_game(ecs : &mut World) {}