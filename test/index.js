// // This test file uses the tape testing framework.
// // To learn more, go here: https://github.com/substack/tape
// const test = require('tape');

// const { Config, Conductor } = require("@holochain/holochain-nodejs")

// const dnaPath = "./dist/bundle.json"

// // closure to keep config-only stuff out of test scope
// const conductor = (() => {
//     const agentAlice = Config.agent("alice")

//     const dna = Config.dna(dnaPath)

//     const instanceAlice = Config.instance(agentAlice, dna)

//     const conductorConfig = Config.conductor([instanceAlice])
//     return new Conductor(conductorConfig)
// })()

// // Initialize the Conductor
// conductor.start()

// const alice = conductor.makeCaller('alice', dnaPath)

const path = require('path')
const { Config, Conductor, Scenario } = require('../../holochain-rust/nodejs_conductor')
Scenario.setTape(require('tape'))

const dnaPath = path.join(__dirname, "../dist/bundle.json")
const dna = Config.dna(dnaPath, 'servicelogger')
const agentAlice = Config.agent("alice")

const instanceAlice = Config.instance(agentAlice, dna)

const scenario = new Scenario([instanceAlice])

scenario.runTape('agentId', async (t, { alice }) => {
  t.ok(alice.agentId)
})

scenario.runTape('example', async (t, { alice }) => {
  // Make a call to a Zome function
  // indicating the capability and function, and passing it an input
  const addr = alice.call("service", "create_request", {"entry" : {"content":"sample content"}})

  const result = alice.call("service", "get_request", {"address": addr.Ok})

  // check for equality of the actual and expected results
  t.deepEqual(result, { Ok: { App: [ 'client_request', '{"content":"sample content"}' ] } })
})