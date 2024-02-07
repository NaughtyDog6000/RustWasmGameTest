use std::convert::Infallible;
use std::{fmt, string};

use bracket_terminal::console;
use serde_json::{self, json};
use specs::prelude::*;
use specs::saveload::{
    DeserializeComponents, SerializeComponents, SimpleMarker, SimpleMarkerAllocator,
};

use crate::components::position::{FloatPosition, IntPosition};
use crate::components::velocity::Velocity;
use crate::NetworkSync;

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

    fn run(&mut self, (entities, velocities, fpositions, ipositions, markers): Self::SystemData) {
        let mut buf = Vec::new();
        let mut ser = serde_json::ser::Serializer::pretty(&mut buf);

        SerializeComponents::<Infallible, SimpleMarker<NetworkSync>>::serialize(
            &(&velocities, &fpositions, &ipositions),
            &entities,
            &markers,
            &mut ser,
        )
        .unwrap_or_else(|e| console::log(format!("Error: {}", e)));

        // TODO!() here we write to the file
        console::log(format!(
            "{}",
            String::from_utf8(buf).expect("should be utf-8")
        ));
    }
}

// LOAD SAVE
use bracket_terminal::EMBED;

bracket_terminal::embedded_resource!(RAW_FILE, "../../resources/fake_saves/save1.json");

pub struct Deserialize;

impl<'a> System<'a> for Deserialize {
    type SystemData = (
        Entities<'a>,
        Write<'a, SimpleMarkerAllocator<NetworkSync>>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, FloatPosition>,
        WriteStorage<'a, IntPosition>,
        WriteStorage<'a, SimpleMarker<NetworkSync>>,
    );

    fn run(
        &mut self,
        (
      entities, 
      mut allocator, 
      velocities, 
      fpositions, 
      ipositions, 
      mut markers
    ): Self::SystemData,
    ) {
        bracket_terminal::link_resource!(RAW_FILE, "../../resources/fake_saves/save1.json");

        let data = bracket_terminal::EMBED
            .lock()
            .get_resource("../../resources/fake_saves/save1.json".to_string())
            .unwrap();

        let json_string = std::str::from_utf8(&data).expect("Unable to convert to string.");

        let mut de = serde_json::de::Deserializer::from_str(json_string);

        DeserializeComponents::<Combined, _>::deserialize(
            &mut (velocities, fpositions, ipositions),
            &entities,
            &mut markers,
            &mut allocator,
            &mut de,
        )
        .unwrap_or_else(|e| console::log(format!("Error in Deserialize: {}", e)))
    }
}
