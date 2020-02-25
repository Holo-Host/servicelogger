[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

# Service Logger

The service logger is an internal Holo component that measures and logs the hosting carried out by Hosts, on behalf of Clients, then generates a proof of service invoice via bridge to Holofuel.

**Clients** that want to have some hosting done will log a Request. Then, when the **Hosts** finished doing a block of hosting, they will require a signature from the Clients, to generate the signed service log. After that the Service Logger will count the number of unpaid logs until its value hits a threshold configured in the Hosting App, to then trigger the generation of an Invoice, via bridge to Holofuel


## How to run test?

Service Logger is best and most reliably built under a `nix-shell` environment.  Checkout the version
of `holochain-rust` consistent with the version specified in `zomes/service/code/Cargo.toml`,
eg. `git checkout v0.0.18-alpha1`.  Then run `nix-shell`.  When you get a prompt, cd into your clone
of `servicelogger`, and run the following commands:

### Using Make

```bash
make test
```

### Manually
First, install testing dependencies:

```bash
cd ./test/ & npm install
```

Then:

```bash
hc test
```

> Note since this repo is in dev mode.  If you want to test using the holochain-rust `develop` branch, first edit `test/index.js` to use a "relative" version of nodejs_conductor;  you would have to pull the [holochain-rust](https://github.com/holochain/holochain-rust) repo in the same folder you pull this repo into. This is required to run the hc test with the latest nodejs_conductor in the holochain-rust.

### Bridging Tests

Requires the HoloFuel, Holo-Hosting-App and service logger DNAs, built and/or copied to the local service logger `dist/` directory.

- Build `holofuel` DNA and copy `holofuel/dist/holofuel.dna.json` to `dist/holofuel.dna.json`
- Build `Holo-Hosting-App` and copy `../Holo-Hosting-App/dna-src/dist/Holo-Hosting-App.dna.json` to `dist/holohosting.dna.json`

Run the Bridging tests with:
```bash
hc test -t test/bridging.js
```


## How to build DNA?
```bash
hc package
```

`.dna.json` result will be in `./dist`.


## Documentaion:

- [Service Logger RFC](https://github.com/Holo-Host/rfcs/blob/master/service-logger/README.md) ([in-progress](https://github.com/Holo-Host/rfcs/pull/31))

### Consistent signatures using WASM key manager

```javascript
const crypto = require('crypto');
const JSONserialize = require('json-stable-stringify');

const { Codec, KeyManager } = require("@holo-host/chaperone");

const utf8 = new TextEncoder();

const keys = new KeyManager( crypto.randomBytes( 32 ) );
const client_agent_id = Codec.AgentId.encode( keys.publicKey() );
const payload = { "key": "value" };

const sig_bytes = keys.sign( utf8.encode(JSONserialize(payload)) );
const b64_sig = Codec.Signature.encode( sig_bytes );
```


## Built With
* [Holochain-rust](https://github.com/holochain/holochain-rust)
* [HDK](https://developer.holochain.org/api/latest/hdk/)

