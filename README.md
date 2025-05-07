# iLEAP Data Model Extension

This repository contains the iLEAP Data Model and protocol for logistics emissions transparency.

It is realized as a so-called [Data Model Extension](https://wbcsd.github.io/data-model-extensions/)
to the [PACT Data Model](https://wbcsd.github.io/tr/data-exchange-protocol/#data-model).

This extension enables

1. transparency over logistics emissions and intensity data, based on the GLEC Framework v3.1 and ISO14083:2023
2. interoperable flow of such data between IT systems
3. interoperability with the PACT Data Model and protocol of Version 2


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
