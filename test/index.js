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

const sample_request = {
  agent_id: "QmUMwQthHNKSjoHpvxtxPPMA8qiMNytwBQEgVXHXjZvZRb",
  zome_call_spec: "blog/create_post",
  dna_hash: "QmfAzihC8RVNLCwtDeeUH8eSAACweFq77KBK4e1bJWmU8A",
  client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1taKk",
}

const sample_response = {
  request_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1x4",
  hosting_stats: {
    cpu_seconds: 3.2,
    bytes_in: 12309,
    bytes_out: 7352,
  },
  response_log: '64.242.88.10 - - [07/Mar/2004:16:11:58 -0800] "GET /twiki/bin/view/TWiki/WikiSyntax HTTP/1.1" 200 7352',
  host_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1taKk"
}

const sample_response2 = {
  request_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1x4",
  hosting_stats: {
    cpu_seconds: 3.3,
    bytes_in: 4332,
    bytes_out: 7352,
  },
  response_log: '64.242.88.10 - - [07/Mar/2004:16:11:58 -0800] "GET /twiki/bin/view/TWiki/WikiSyntax HTTP/1.1" 200 7352',
  host_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1taKk"
}

// 1. The ServiceLog has been initiated, now it requires: PaymentPrefs and a dna_bundle_hash to be set
scenario.runTape('can do initial setup', async (t, { app }) => {
  const addr = app.call("service", "set_payment_prefs", {"entry" : payment_prefs})

  t.ok(addr.Ok, "Address is set")
})

// 2. The client starts a new request for hosting, based on a call from the HC Interceptor
scenario.runTape('can log a client request', async (t, { app }) => {
  app.call("service", "set_payment_prefs", {"entry" : payment_prefs})

  const addr = app.call("service", "log_request", {"entry" : sample_request})

  const result = app.call("service", "get_request", {"address": addr.Ok})

  t.deepEqual(result, { Ok: { App: [ 'client_request', JSON.stringify(sample_request) ] } })
})

// 3. The Conductor wants to record a HostResponse, indicating some hosting was done

scenario.runTape('can log a host response', async (t, { app }) => {
  app.call("service", "set_payment_prefs", {"entry" : payment_prefs})

  const addr = app.call("service", "log_request", {"entry" : sample_request})

  t.deepEqual(addr.Ok, "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1x4")

  const addr2 = app.call("service", "log_response", {"entry" : sample_response})

  const result = app.call("service", "get_response", {"address": addr2.Ok})

  t.deepEqual(result, { Ok: { App: [ 'host_response', JSON.stringify(sample_response) ] } })
})

// 4. With the client signature on that HostResponse, the Conductor creates a ServiceLog, that is a billable log
scenario.runTape('can create a servicelog', async (t, { app }) => {
  app.call("service", "set_payment_prefs", {"entry" : payment_prefs})

  app.call("service", "log_request", {"entry" : sample_request})

  const addr = app.call("service", "log_response", {"entry" : sample_response})

  const service_log = {
    response_hash: addr.Ok,
    client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1tZKk"
  }

  const addr2 = app.call("service", "log_service", {"entry": service_log})

  const result = app.call("service", "get_service", {"address": addr2.Ok})

  t.deepEqual(result, { Ok: { App: [ 'service_log', JSON.stringify(service_log) ] } })
})

// 5. On the UI, list all billable ServiceLogs, filter by start and end time (for pagination)

// 6. Generate an invoice based on the selected ServiceLogs

// 7. Checks if the unpaid value is greater than the PaymentPrefs, then call stop_hosting() on Hosting App
