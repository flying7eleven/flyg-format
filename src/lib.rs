#![deny(clippy::all)]
#![deny(clippy::pedantic)]

/// The `FlygFlight` struct is the root of any recorded flight.
pub struct FlygFlight {
    /// The static information about the used plane.
    pub plane_information: PlaneInformation,
    /// The touch down speed of the plane in feet per second.
    pub landing_speed: f32,
    /// The important time recording of the flight.
    pub times: Times,
}

/// All important information about the plane which was used to perform the flight, are
/// stored in the `PlaneInformation` structure. This is mostly static information which is
/// not changed during the course of the flight.
pub struct PlaneInformation {
    /// The name of the plane which was used to perform the flight (provided by the simulator).
    pub name: String,
    /// The amount of over all fuel the plane can carry (in gallons).
    pub fuel_capacity: u32,
    /// The number of engines the plane had when performing the flight.
    pub number_of_engines: u8,
}

/// There are four different phases of flight. The first one is the block-off time. This is the time
/// the plane started moving for the first time of the flight. The second time is the takeoff time,
/// which is the time the plane left the ground and became airborne. The second-last time is the
/// landing time, which is the time when the wheels contacted with ground after a flight. The last of
/// the four options is the block-on time. This is the time the plane arrived at the final position and
/// shut down its engines. All those times are stored in the `Times` data structure.
pub struct Times {
    /// The time when the plane started taxing from the gate to runway.
    pub block_off_time: String,
    /// The time when the plane arrived at the gate after a landing.
    pub block_on_time: String,
    /// The time the plane landed on the runway.
    pub landing_time: String,
    /// The time the plane took off from the runway.
    pub takeoff_time: String,
}

/// The `FlygFormatError` enum holds all possible errors which can occur when processing a file
/// with flight data recordings.
#[derive(Debug)]
pub enum FlygFormatError {
    /// The content of the supplied file could not be interpreted.
    FileFormatNotRecognized,
}

impl std::fmt::Display for FlygFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlygFormatError::FileFormatNotRecognized => {
                write!(f, "Content of supplied file is not recognized")
            }
        }
    }
}

impl std::error::Error for FlygFormatError {}
