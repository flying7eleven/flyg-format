# Changelog
All notable changes to this library will be documented in this file. The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Calendar Versioning](https://calver.org/).

## [Unreleased]

### Added
- Added the data type `VerticalSpeed`

### Changed
- The time representations in the flight data records are now actual time types and
  not just simple strings.

## [2021.12.26]

### Added
- Added the `FlygFlight` data structure
- Added the `PlaneInformation` data structure
- Added the `FuelRecord` data structure
- Added the `PositionRecord` data structure
- Added the `Times` data structure
- Added the new error type `FlygFormatError`
- Added a method (`load_flight_from_compressed_file`) for loading a Flyg data record


[unreleased]: https://github.com/flying7eleven/flyg-format/compare/2021.12.26...HEAD
[2021.12.26]: https://github.com/flying7eleven/flyg-format/releases/tag/2021.12.26