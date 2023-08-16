# Data concentrator software
The project's original intent was a minimal communication API for the 
["Microcontroller manager software"](https://github.com/dcrntn/mcu-manager)
(it still is). However this project had the "potential" to morph into a standalone project. The goal is to have a REST-like interface to read, write values and if needed "map" these values to "soft-industrial" protocols (e.g: Modbus TCP / RTU, OPC UA, MQTT etc...), thus it'd be easier to communicate with field devices, and even "devices on different protocols" can exchange data between each other.

_Soft-industrial's meaning in this context: protocols that are usually used for transfering data to the industrial edge from the field._

## Note
This is __still__ a get to know RUST project, the implementation and approach might not please everyone.

This project is in a very early stage.

## Features
### Implemented
1. Basic rAPI to read and write values.
2. Modbus TCP "map" to read and write Modbus registers through the rAPI
["data-concentrator-mbtcp"](https://github.com/dcrntn/data-concentrator-mbtcp)

## Installation
1. Clone or download this repo.
2. Build with Cargo
3. Run

## Usage
TBA
