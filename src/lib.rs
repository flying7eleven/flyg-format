#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use serde::Deserialize;

/// The [`FlygFlight`] struct is the root of any recorded flight.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlygFlight {
    /// The static information about the used plane.
    pub plane_information: PlaneInformation,
    /// The touch down speed of the plane in feet per second.
    pub landing_speed: f32,
    /// The important time recording of the flight.
    pub times: Times,
}

/// All important information about the plane which was used to perform the flight, are
/// stored in the [`PlaneInformation`] structure. This is mostly static information which is
/// not changed during the course of the flight.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
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
/// shut down its engines. All those times are stored in the [`Times`] data structure.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// The [`FlygFormatError`] enum holds all possible errors which can occur when processing a file
/// with flight data recordings.
#[derive(Debug, Eq, PartialEq)]
pub enum FlygFormatError {
    /// The supplied file name could not be opened (could be permissions or an invalid file path).
    CouldNotOpenFile,
    /// The content of the supplied file could not be interpreted.
    FileFormatNotRecognized,
}

impl std::fmt::Display for FlygFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlygFormatError::CouldNotOpenFile => {
                write!(f, "The supplied file could not be opened")
            }
            FlygFormatError::FileFormatNotRecognized => {
                write!(f, "Content of supplied file is not recognized")
            }
        }
    }
}

impl std::error::Error for FlygFormatError {}

/// Load stored flight information from a file.
///
/// # Errors
/// If the content of the supplied file is not known to the method, a [`FileFormatNotRecognized`]
/// error will be returned.
pub fn load_flight_information_from_file(filename: &str) -> Result<FlygFlight, FlygFormatError> {
    use std::io::BufReader;

    match std::fs::File::open(filename) {
        Ok(file_handle) => {
            let buffered_reader = BufReader::new(file_handle);
            match serde_json::from_reader(buffered_reader) {
                Ok(read_obj) => Ok(read_obj),
                Err(_) => Err(FlygFormatError::FileFormatNotRecognized),
            }
        }
        Err(_) => Err(FlygFormatError::CouldNotOpenFile),
    }
}

#[cfg(test)]
mod tests {
    use crate::{load_flight_information_from_file, FlygFormatError};

    #[test]
    fn loading_a_non_existing_file_is_handled_correctly() {
        // a path to a file which is not existing
        let path_to_input_file = "test_data/this_file_does_not_exist.flyg";

        // try to load the file into memory
        let result = load_flight_information_from_file(path_to_input_file);

        // check if the case was handled gracefully
        assert_eq!(true, result.is_err());
        assert_eq!(FlygFormatError::CouldNotOpenFile, result.err().unwrap());
    }

    #[test]
    fn loading_a_valid_uncompressed_file_works() {
        // the path to the uncompressed flyg data file which should be used for tests
        let path_to_input_file = "test_data/uncompressed.flyg";

        // try to load the file into memory
        let result = load_flight_information_from_file(path_to_input_file);

        // check if the file was loaded as expected
        assert_eq!(true, result.is_ok());
    }
}
