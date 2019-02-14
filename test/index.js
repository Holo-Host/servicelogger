const path = require('path')
const { Config, Conductor, Scenario } = require('../../holochain-rust/nodejs_conductor')
Scenario.setTape(require('tape'))

const dnaPath = path.join(__dirname, "../dist/bundle.json")
const dna = Config.dna(dnaPath, 'servicelogger')
const agentApp = Config.agent("app")

const instanceApp = Config.instance(agentApp, dna)

const scenario = new Scenario([instanceApp])

// Basic agentId check
scenario.runTape('agentId', async (t, { app }) => {
  t.ok(app.agentId)
})

const payment_prefs = {
  provider_address: "QmUMwQthHNKSjoHpvxtxPPMA8qiMNytwBQEgVXHXjZvZRb",
  dna_bundle_hash: "QmfAzihC8RVNLCwtDeeUH8eSAACweFq77KBK4e1bJWmU8A",
  max_fuel_per_invoice: 1.0,
  max_unpaid_value: 2.0,
}

// 1. The ServiceLog has been initiated, now it requires: PaymentPrefs and a dna_bundle_hash to be set
scenario.runTape('can do initial setup', async (t, { app }) => {
  const addr = app.call("service", "set_payment_prefs", {"entry" : payment_prefs})

  t.ok(addr.Ok, "Address is set")
})

// 2. The client starts a new request for hosting, based on a call from the HC Interceptor
scenario.runTape('can log a client request', async (t, { app }) => {
  const sample_request = {
    agent_id: "QmUMwQthHNKSjoHpvxtxPPMA8qiMNytwBQEgVXHXjZvZRb",
    zome_call_spec: "blog/create_post",
    dna_hash: "QmfAzihC8RVNLCwtDeeUH8eSAACweFq77KBK4e1bJWmU8A",
    client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1taKk",
  }

  const addr = app.call("service", "log_request", {"entry" : sample_request})

  const result = app.call("service", "get_request", {"address": addr.Ok})

  t.deepEqual(result, { Ok: { App: [ 'client_request', JSON.stringify(sample_request) ] } })
})

// 3. The Conductor wants to record a HostResponse, indicating some hosting was done

// 4. With the client signature on that HostResponse, the Conductor creates a ServiceLog, that is a billable log

// 5. On the UI, list all billable ServiceLogs, filter by start and end time (for pagination)

// 6. Generate an invoice based on the selected ServiceLogs

// 7. Checks if the unpaid value is greater than the PaymentPrefs, then call stop_hosting() on Hosting App
