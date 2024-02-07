use std::convert::Infallible;
use std::fmt;

use bracket_terminal::console;
use serde_json::{self, json};
use specs::prelude::*;
use specs::saveload::{
    DeserializeComponents, SerializeComponents, SimpleMarker, SimpleMarkerAllocator,
};

use crate::components::position::{FloatPosition, IntPosition};
use crate::components::velocity::Velocity;
use crate::NetworkSync;

const ENTITIES: &str = r#"[
    {
      "marker": [
        0
      ],
      "components": [
        null,
        {
          "x": 32.0,
          "y": 12.0
        },
        null
      ]
    },
    {
      "marker": [
        1
      ],
      "components": [
        {
          "x_velocity": 2.0,
          "y_velocity": 1.0
        },
        {
          "x": 67.62347,
          "y": 43.81175
        },
        null
      ]
    },
    {
      "marker": [
        2
      ],
      "components": [
        {
          "x_velocity": 2.0,
          "y_velocity": 1.0
        },
        {
          "x": 73.62343,
          "y": 43.81175
        },
        null
      ]
    },
    {
      "marker": [
        3
      ],
      "components": [
        {
          "x_velocity": 2.0,
          "y_velocity": 1.0
        },
        {
          "x": 79.62343,
          "y": 43.81175
        },
        null
      ]
    },
    {
      "marker": [
        4
      ],
      "components": [
        {
          "x_velocity": 2.0,
          "y_velocity": 1.0
        },
        {
          "x": 85.62348,
          "y": 43.81175
        },
        null
      ]
    },
    {
      "marker": [
        5
      ],
      "components": [
        {
          "x_velocity": 2.0,
          "y_velocity": 1.0
        },
        {
          "x": 91.62349,
          "y": 43.81175
        },
        null
      ]
    },
    {
      "marker": [
        6
      ],
      "components": [
        {
          "x_velocity": 2.0,
          "y_velocity": 1.0
        },
        {
          "x": 97.62353,
          "y": 43.81175
        },
        null
      ]
    },
    {
      "marker": [
        7
      ],
      "components": [
        {
          "x_velocity": 2.0,
          "y_velocity": 1.0
        },
        {
          "x": 103.62362,
          "y": 43.81175
        },
        null
      ]
    },
    {
      "marker": [
        8
      ],
      "components": [
        {
          "x_velocity": 2.0,
          "y_velocity": 1.0
        },
        {
          "x": 109.62363,
          "y": 43.81175
        },
        null
      ]
    },
    {
      "marker": [
        9
      ],
      "components": [
        {
          "x_velocity": 2.0,
          "y_velocity": 1.0
        },
        {
          "x": 115.623634,
          "y": 43.81175
        },
        null
      ]
    },
    {
      "marker": [
        10
      ],
      "components": [
        {
          "x_velocity": 2.0,
          "y_velocity": 1.0
        },
        {
          "x": 121.623634,
          "y": 43.81175
        },
        null
      ]
    }
  ]"#;

// macro_rules! serialize_individually {
//     ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
//         $(
//         SerializeComponents::<NoError, SimpleMarker<SerializeMe>>::serialize(
//             &( $ecs.read_storage::<$type>(), ),
//             &$data.0,
//             &$data.1,
//             &mut $ser,
//         )
//         .unwrap();
//         )*
//     };
// }

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
        let mut de = serde_json::de::Deserializer::from_str(ENTITIES);

        DeserializeComponents::<Combined, _>::deserialize(
            &mut (velocities, fpositions, ipositions),
            &entities,
            &mut markers,
            &mut allocator,
            &mut de,
        )
        .unwrap_or_else(|e| console::log(format!("Error in Deserialize: {}", e)))

        // TODO!() here we write to the file
    }
}

// if on the web, we cannot save/load yet.
// #[cfg(target_arch = "wasm32")]
// pub fn save_game(_ecs : &mut World) {

// }

// #[cfg(not(target_arch = "wasm32"))]
// pub fn save_game(ecs : &mut World) {}
