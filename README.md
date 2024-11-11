# iLEAP Data Model Extension – version 0.2.0

This repository contains the SCF iLEAP [Data Model
Extension](https://wbcsd.github.io/data-model-extensions/) to the [PACT Data
Model](https://wbcsd.github.io/tr/data-exchange-protocol/#data-model). This extension enables the integration of Logistics Emissions Data Exchange into the Pathfinder Network.

## Technical Specifications

The technical foundations of the iLEAP Data Model Extension, including considerations about its
motivation and use, can be found [here](https://sine-fdn.github.io/ileap-extension/).

## SQL-based Data Model

You can access a SQL-based realization of the iLEAP data model in the file [ileap.sql](sql-example/ileap.sql).

## Rust Implementation

SINE provides also Rust implementations of
- the PACT Data Model,
- the iLEAP Data Model, and
- a [demo API](https://api.ileap.sine.dev) conforming to the iLEAP Technical Specifications.
These can be found in the [`sine-fdn/impact-protocols`](https://github.com/sine-fdn/impact-protocols) repository.

## SQL Example Implementation

An SQLITE3-compliant SQL data model of the iLEAP data model extension can also be found in [`sine-fdn/impact-protocols`](https://github.com/sine-fdn/impact-protocols/tree/main/ileap-data-model/sql-example)

## Contribute

We welcome external contributions.

[CONTRIBUTING.md](CONTRIBUTING.md) contains more details.
