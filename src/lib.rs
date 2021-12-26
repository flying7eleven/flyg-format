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
    /// All fuel related dynamic information during the flight.
    pub fuel_records: Vec<FuelRecord>,
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
    /// The weight of one gallon of fuel for the plane in pounds.
    pub fuel_weight: f32,
    /// The amount of fuel (in gallons) which is not usable by the plane.
    pub unusable_fuel_quantity: f32,
}

/// The [`FuelRecord`] struct holds all information regarding fuel and the flight of the plane (e.g.
/// the amount of fuel which is currently burned per hour or the remaining fuel in the planes tanks).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuelRecord {
    /// The remaining fuel which the plane is carrying.
    fuel_quantity: f32,
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
    /// Could not decompress the file which was provided.
    DecompressionFailed,
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
            FlygFormatError::DecompressionFailed => {
                write!(f, "Could not decompress file")
            }
        }
    }
}

impl std::error::Error for FlygFormatError {}

#[cfg(feature = "compression")]
fn load_flight_from_compressed_file(filename: &str) -> Result<FlygFlight, FlygFormatError> {
    use libflate::gzip::Decoder;

    match std::fs::File::open(filename) {
        Ok(file_handle) => match Decoder::new(file_handle) {
            Ok(decoder) => match serde_json::from_reader(decoder) {
                Ok(read_obj) => Ok(read_obj),
                Err(_) => Err(FlygFormatError::FileFormatNotRecognized),
            },
            Err(_) => Err(FlygFormatError::DecompressionFailed),
        },
        Err(_) => Err(FlygFormatError::CouldNotOpenFile),
    }
}

/// Load stored flight information from a file.
///
/// # Errors
/// If the content of the supplied file is not known to the method, a
/// [`FlygFormatError::FileFormatNotRecognized`] error will be returned.
pub fn load_flight_information_from_file(filename: &str) -> Result<FlygFlight, FlygFormatError> {
    use std::io::BufReader;

    // if the file ends with .cflyg, we assume it is compressed and we can redirect the open
    // request to the corresponding helper method
    #[cfg(feature = "compression")]
    if filename
        .rsplit('.')
        .next()
        .map(|ext| ext.eq_ignore_ascii_case("cflyg"))
        == Some(true)
    {
        return load_flight_from_compressed_file(filename);
    }

    // for all other cases, we assume its a non-compressed file and we can handle it directly
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

    #[test]
    #[cfg(feature = "compression")]
    fn loading_a_valid_compressed_file_works() {
        // the path to the uncompressed flyg data file which should be used for tests
        let path_to_input_file = "test_data/compressed.cflyg";

        // try to load the file into memory
        let result = load_flight_information_from_file(path_to_input_file);

        // check if the file was loaded as expected
        assert_eq!(true, result.is_ok());
    }
}
