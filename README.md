# buffering
A library for handling buffer operations

## Purpose
This library is primarily aimed at simple network serialization and deserialization for a variety
of struct types. It provides a copy-based Cursor solution for more complicated network data structures
or a macro to generate a union type that allows access to fields for inspection and the underlying
buffer for network transfer.
