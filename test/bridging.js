const path = require('path')
const sleep = require('sleep')
const { Config, Conductor, Scenario } = require('../../holochain-rust/nodejs_conductor')
Scenario.setTape(require('tape'))

const dnaPath = path.join(__dirname, "../dist/servicelogger.dna.json")
const dna = Config.dna(dnaPath, 'servicelogger')
const agentApp = Config.agent("app")
const appInstance = Config.instance(agentApp, dna)

// ATTENTION! to test the Holofuel bridge you need to place a 'holofuel.dna.json' file into the /dist folder (created by packaging the holofuel app)
// https://github.com/Holo-Host/holofuel/
const fuelPath = path.join(__dirname, "../dist/holofuel.dna.json")
const fuelDna = Config.dna(fuelPath, 'holofuel')
const fuelApp = Config.agent("fuel")
const fuelInstance = Config.instance(fuelApp, fuelDna)

// ATTENTION! to test the Holohosting bridge you need to place a 'holohosting.dna.json' file into the /dist folder
// https://github.com/Holo-Host/Holo-Hosting-App
const hostPath = path.join(__dirname, "../dist/holohosting.dna.json")
const hostDna = Config.dna(hostPath, 'holohost')
const hostApp = Config.agent("host")
const hostInstance = Config.instance(hostApp, hostDna)

const hfBridge = Config.bridge('holofuel-bridge', appInstance, fuelInstance)
const hhBridge = Config.bridge('hosting-bridge', appInstance, hostInstance)

const scenario = new Scenario([appInstance, hostInstance, fuelInstance], { bridges: [hfBridge, hhBridge], debugLog: true })

const sample_request = {
  agent_id: "QmUMwQthHNKSjoHpvxtxPPMA8qiMNytwBQEgVXHXjZvZRb",
  zome_call_spec: "blog/create_post",
  dna_hash: "QmfAzihC8RVNLCwtDeeUH8eSAACweFq77KBK4e1bJWmU8A",
  client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1taKk",
}

const sample_response1 = {
  request_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1x4",
  hosting_stats: {
    cpu_seconds: 3.2,
    bytes_in: 12309,
    bytes_out: 7352,
  },
  response_data_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
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
  response_data_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
  response_log: '64.242.88.10 - - [07/Mar/2004:16:11:58 -0800] "GET /twiki/bin/view/TWiki/WikiSyntax HTTP/1.1" 200 7352',
  host_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1taKk"
}

const Provider_Doc = {
  provider_doc: {
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
  }
}

const App_Config = {
  app_bundle: {
    happ_hash: "QuarnnnnvltuenblergjasnvAfs",
    dna_list: ["QweAFioina","QtavsFdvva"]
  },
  app_details: {
    name:"App Name",
    details:"Details for this app",
  },
  domain_name: {
    dns_name: "app2.holo.host"
  }
}

function perform_hosting_setup(host, fuel) {
  // Perform Holohost setup
  host.call("provider", "register_as_provider", Provider_Doc);

  // sleep to wait for link propagation
  sleep.sleep(5);
  // Add the holofuel account to the Provider
  host.call("provider", "add_holofuel_account", {"holofuel_account_details":{"account_number" : fuel.agentId}});

  const app_address = host.call("provider", "register_app", App_Config).Ok;
  // console.log("APP ADDRESS:: ", app_address);

  const payment_prefs = {
    app_hash: app_address,
    max_fuel_per_invoice: 2.0,
    max_unpaid_value: 4.0,
    price_per_unit: 1.0
  }

  // sleep to wait for link propagation
  sleep.sleep(5);

  host.call("host", "add_service_log_details", payment_prefs);
  sleep.sleep(5);
  return app_address;
}

// 5. Generate an invoice based on the selected ServiceLogs
scenario.runTape('testing invoice generation', async (t, { app, host, fuel }) => {

  const app_address = perform_hosting_setup(host, fuel);

  const setup_prefs = {
    dna_bundle_hash: app_address,
  }

  // performs initial setup
  app.call("service", "setup", {"entry": setup_prefs})

  // Logs a sample request
  app.call("service", "log_request", {"entry": sample_request})

  // Log a first response & service_log
  const addr1 = app.call("service", "log_response", {"entry": sample_response1})
  const service_log1 = {
    response_hash: addr1.Ok,
    client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1tZKk"
  }
  app.call("service", "log_service", {"entry": service_log1})

  // Check we have no invoice yet
  var invoices = app.call("service", "get_unpaid_invoices", {})
  t.deepEqual(invoices, { Ok: [] })

  // Log a second response & service_log **triggering** an invoice generation (passed threshold)
  const addr2 = app.call("service", "log_response", {"entry" : sample_response2})
  const service_log2 = {
    response_hash: addr2.Ok,
    client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1tZKk"
  }
  app.call("service", "log_service", {"entry": service_log1})

  // Now we should have an invoice
  invoices = app.call("service", "get_unpaid_invoices", {})
  t.deepEqual(invoices, { Ok: ['QmeiBEvq43yd77PS1jyEPZAnckCvrbETGNAp4wcHbEMv7b'] })
})


const sample_response3 = {
  request_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1x4",
  hosting_stats: {
    cpu_seconds: 3.2,
    bytes_in: 12309,
    bytes_out: 7352,
  },
  response_data_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
  response_log: '64.242.88.10 - - [07/Mar/2004:16:11:58 -0800] "GET /twiki/bin/view/TWiki/WikiSyntax HTTP/1.1" 200 7352',
  host_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzV1taKk"
}

const sample_response4 = {
  request_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1x4",
  hosting_stats: {
    cpu_seconds: 3.1,
    bytes_in: 3332,
    bytes_out: 552,
  },
  response_data_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy8xV6aV1xv",
  response_log: '64.242.88.10 - - [08/Mar/2004:16:11:58 -0800] "GET /twiki/bin/view/TWiki/WikiSyntax HTTP/1.1" 200 552',
  host_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxkz1taKv"
}

// 6. Checks if the unpaid value is greater than the PaymentPrefs, then will alert envoy over `get_payment_status` API
scenario.runTape('testing payment status', async (t, { app, host, fuel }) => {
  const app_address = perform_hosting_setup(host, fuel);

  const setup_prefs = {
    dna_bundle_hash: app_address,
  }

  // performs initial setup
  app.call("service", "setup", {"entry": setup_prefs})

  // Logs a sample request
  app.call("service", "log_request", {"entry": sample_request})

  // Log a first response & service_log
  const addr1 = app.call("service", "log_response", {"entry": sample_response1})
  const service_log1 = {
    response_hash: addr1.Ok,
    client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1tZKk"
  }
  app.call("service", "log_service", {"entry": service_log1})

  // Check we have no invoice yet
  var invoices = app.call("service", "get_unpaid_invoices", {})
  t.deepEqual(invoices, { Ok: [] })

  // Log a second response & service_log **triggering** an invoice generation (passed threshold)
  const addr2 = app.call("service", "log_response", {"entry" : sample_response2})
  const service_log2 = {
    response_hash: addr2.Ok,
    client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1tZKk"
  }
  app.call("service", "log_service", {"entry": service_log1})

  // Now we should have an invoice
  invoices = app.call("service", "get_unpaid_invoices", {})
  t.deepEqual(invoices, { Ok: ['QmeiBEvq43yd77PS1jyEPZAnckCvrbETGNAp4wcHbEMv7b'] })

  // Check if payment status is still HOSTING
  var payment_status = app.call("service", "get_payment_status", {}).Ok;
  t.deepEqual(payment_status.unpaid_value, 2.0);
  t.deepEqual(payment_status.max_unpaid_value, 4.0);
  t.deepEqual(payment_status.situation, "HOSTING");

  // Log two more resposes **triggering** another invoice generation
  const addr3 = app.call("service", "log_response", {"entry" : sample_response3});
  app.call("service", "log_service", {"entry": {
    response_hash: addr3.Ok,
    client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1tZKk",
    }
  });
  const addr4 = app.call("service", "log_response", {"entry" : sample_response4});
  app.call("service", "log_service", {"entry": {
    response_hash: addr4.Ok,
    client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1tZKk",
    }
  });

  payment_status = app.call("service", "get_payment_status", {}).Ok;
  t.deepEqual(payment_status.unpaid_value, 4.0);
  t.deepEqual(payment_status.situation, "STOPPED");

})


