# Manchester Baby Emulator Server

This is a simple server wrapper for [baby-emulator](https://github.com/jasonalexander-ja/SSEMBabyEmulator)
with the ability to assemble and run code on an emulated Manchester Baby, executing one instruction 
every second and sending a serialised state of emulated memory and register to a 8*84 flip dot display
over MQTT. 

## Configuration

All the settings are set via the `.env` file in the root of this crate. 

- The socket address where the server binds to  
    ```text
    LISTEN="127.0.0.1:8080" 
    ```
- The MQTT broker server
    ```
    ADDRESS="10.0.0.4"
    ```
- The MQTT topic the emulator should pubish the state to 
    ```
    TOPIC="nh/flipdot/comfy/raw" 
    ```

## Endpoints 

### /run

Takes a given JSON serialised Manchester Baby model and runs it either until a `HALT` instruction is reached or a cancel is sent. 

```http
POST /run HTTP/1.1

{
    "main_store": [71, 38, 96, 5, -32, 1, 1, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    "accumulator": 0,
    "instruction_address": 0,
    "instruction": 71
}
```

**Model:**

- [BabyModel](###babymodel)

**Returns:** 
- `200 OK`: request has been accepted and the emulation is running, sending a second run request when one is running returns;
- `423 LOCKED`: an emulation is already running 
- `400 BAD REQUEST`: invalid emulation model submitted 
- `500 INTERNAL SERVER ERROR`: emulation service has stopped 
- `503 SERVICE UNAVAILABLE`: too many requests have been sent and the communication channels are full 

### /assemble_run

Takes an assembly listing and assembles it into a Baby model and runs it either until a `HALT` instruction is reached or a cancel is sent. 

```http
POST /assemble_run HTTP/1.1

{
    "listing": "Assembly Listing",
    "og_notation": false,
}
```

**Model:**

- [Assembly](###assembly)

**Returns:** 
- `200 OK`: request has been accepted and the emulation is running, sending a second run request when one is running returns;
- `423 LOCKED`: an emulation is already running 
- `400 BAD REQUEST`: invalid emulation model submitted, or an error was encountered when assembling `listing` 
- `500 INTERNAL SERVER ERROR`: emulation service has stopped 
- `503 SERVICE UNAVAILABLE`: too many requests have been sent and the communication channels are full 

### /assemble

Takes an assembly listing, and attempts to assemble it, storing the result in the returned [BabyModel](###babymodel) ready to be ran. 

```http
POST /assemble HTTP/1.1

{
    "listing": "Assembly Listing",
    "og_notation": false,
}
```

**Model:**

- [Assembly](###assembly)

**Returns:** 
- `200 OK`: sucessfully assembled and returned a [BabyModel](###babymodel)
- `400 BAD REQUEST`: invalid emulation model submitted, or an error was encountered when assembling `listing` (body will contain the error message) 
- `500 INTERNAL SERVER ERROR` 

### /cancel

Cancels a currently running emulation, if not emulation is running it will just return OK. 

```http
POST /cancel HTTP/1.1

```

**Returns:** 

- `200 OK`: halt request sucessfully sent 
- `500 INTERNAL SERVER ERROR`: emulation service has disconnected for some reason 


## Models

### BabyModel

**Example:**
```json
{
    "main_store": [71, 38, 96, 5, -32, 1, 1, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    "accumulator": 0,
    "instruction_address": 0,
    "instruction": 71
}
```
**Fields:**
- `main_store`: an array of 32 x 8 bit numbers
- `accumulator`: initial state of the accumulator
- `instruction_address`: index of the first instruction to execute in the `main_store`
- `instruction`: initial instruction to be executed 

### Assembly

**Example:**
```json
{
    "listing": "Assembly Listing",
    "og_notation": false,
}
```
**Fields:**
- `listing`: the assembly listing to be executed, for example code, see [SSEMExample](https://github.com/jasonalexander-ja/SSEMExample)
- `og_notation`: boolean of whether modern or original mnemonics are used in the assembly listing 
