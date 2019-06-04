# Holo ServiceLogger App


[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

**Status:** Closed-Alpha. Early development and testing.

The service logger is an internal Holo component that measures and logs the hosting carried out by Hosts, on behalf of Clients, then generates a proof of service invoice via bridge to Holofuel.

**Clients** that want to have some hosting done will log a Request. Then, when the **Hosts** finished doing a block of hosting, they will require a signature from the Clients, to generate the signed service log. After that the Service Logger will count the number of unpaid logs until its value hits a threshold configured in the Hosting App, to then trigger the generation of an Invoice, via bridge to Holofuel

## How to run test?

ServiceLogger is best and most reliably built under a `nix-shell` environment.  Checkout the version
of `holochain-rust` consistent with the version specified in `zomes/service/code/Cargo.toml`,
eg. `git checkout v0.0.18-alpha1`.  Then run `nix-shell`.  When you get a prompt, cd into your clone
of `servicelogger`, and run the following commands:

```
hc test
```
> Note since this repo is in dev mode.  If you want to test using the holochain-rust `develop` branch, first edit `test/index.js` to use a "relative" version of nodejs_conductor;  you would have to pull the [holochain-rust](https://github.com/holochain/holochain-rust) repo in the same folder you pull this repo into. This is required to run the hc test with the latest nodejs_conductor in the holochain-rust.

### Bridging Tests

Requires the HoloFuel, Holo-Hosting-App and servicelogger DNAs, built and/or copied to the local servicelogger `dist/` directory.

- Build `holofuel` DNA and copy `holofuel/dist/holofuel.dna.json` to `dist/holofuel.dna.json`
- Build `Holo-Hosting-App` and copy `../Holo-Hosting-App/dna-src/dist/Holo-Hosting-App.dna.json' to `dist/holohosting.dna.json`

Run the Bridging tests with:
```
hc test -t test/bridging.js
```

## How to build DNA?
After running the cmd bellow you will find the `.dna.json` in the `dna-src/dist` folder

```
hc package
```
# How to test the Service Logger without a Holo Envoy (using the debug UI)
> In order for this procedure to work you will need to download the Holofuel and Hosting App DNAs and place them inside the `dist` folder, also, if you want to test the Holofuel UI, you will need to have a `ui` folder inside the `dist` folder, that corresponds to the Holofuel UI.
### Steps to run:
  1. Edit `conductor-config.toml` to indicate to one of your keys (you can generate one with `hc keygen` if don't have one yet)

  2. Run the production conductor with the provided condiguration:
      ```
      holochain -c conductor-config.toml
      ```
  3. Open a browser pointing at https://localhost:8800 (ServiceLogger Debug UI)

  4. If you want to test Holofuel too, open a browser pointing at https://localhost:8300 (Holofuel UI) ** (if you copied the UI files properly)

## Documentaion:

**[ServiceLogger](https://hackmd.io/Eb6YwHlgSNqQXLJyf94_xQ?view)**


## Built With
* [Holochain-rust](https://github.com/holochain/holochain-rust)
* [HDK](https://developer.holochain.org/api/latest/hdk/)

## Authors
* **Julio Monteiro** - [juliolrmonteiro](https://github.com/juliolrmonteiro)

