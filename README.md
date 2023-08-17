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
3. Mqtt "map" to read and write mqtt topics (single valuse) through the rAPI ["data-concentrator-mqtt"](https://github.com/dcrntn/data-concentrator-mqtt)

### Inter communication between portocols
Mapping multiple protocols to one rAPI "data node" makes it possible to do communication between protocols. 

#### Example
A microcontroller writes values through mqtt to a specific "data node" (one value in the rAPI), a PLC through modbus reads the same "data node", thus a data transfer is present, even though the devices do not use the same comm. protocol. Nor they have any form of direct connection setup between each other.

Simple data flow:
MCU (MQTT)  -> Data concentrator -> PLC (MODBUS TCP)

Achieved data transfer:
MCU (MQTT) -> PLC (MODBUS TCP)


## Installation
TBA

## Usage
TBA
