# Picasso

[![CI](https://github.com/martinabeleda/picasso/workflows/ci/badge.svg?event=push)](https://github.com/martinabeleda/picasso/actions?query=event%3Apush+branch%3Amain+workflow%3Aci)
[![code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/ambv/black)

Fast Python geometry manipulation using the Rust [geo](https://github.com/georust/geo) library

The geo crate provides geospatial primitive types such as Point, LineString, and Polygon, and provides algorithms and operations such as:

- Area and centroid calculation
- Simplification and convex hull operations
- Euclidean and Haversine distance measurement
- Intersection checks
- Affine transforms such as rotation and translation.
- Please refer to the documentation for a complete list.

The primitive types also provide the basis for other functionality in the Geo ecosystem, including:

- Coordinate transformation and projection
- Serialization to and from GeoJSON and WKT
- Geocoding
- Working with GPS data