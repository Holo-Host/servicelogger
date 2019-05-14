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



// 5. Generate an invoice based on the selected ServiceLogs
scenario.runTape('generating an invoice', async (t, { app, host, fuel }) => {

  // Perform Holohost setup
  const register_pro = host.call("provider", "register_as_provider", Provider_Doc);
  console.log(JSON.stringify(register_pro));

  // sleep to wait for link propagation
  sleep.sleep(5);
  // Add the holofuel account to the Provider
  host.call("provider", "add_holofuel_account", {"account_number" : fuel.agentId});

  const register_app = host.call("provider", "register_app", App_Config);
  console.log(JSON.stringify(register_app));

  const app_address = register_app.Ok;
  console.log("APP ADDRESS:: ", app_address);

  payment_prefs = {
    app_hash: app_address,
    max_fuel_per_invoice: 2.0,
    max_unpaid_value: 10.0,
//    price_per_unit: 1.0
  }
  
  // sleep to wait for link propagation
  sleep.sleep(5);

  host.call("host","add_service_log_details", payment_prefs);

  const setup_prefs = {
    dna_bundle_hash: app_address,
  }

  sleep.sleep(5);
  // performs initial setup
  app.call("service", "setup", {"entry": setup_prefs})

  // Logs a sample request
  app.call("service", "log_request", {"entry" : sample_request})

  // Log a first response & service_log
  const addr1 = app.call("service", "log_response", {"entry" : sample_response1})
  const service_log1 = {
    response_hash: addr1.Ok,
    client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1tZKk"
  }
  app.call("service", "log_service", {"entry": service_log1})

  // Log a second response & service_log
  const addr2 = app.call("service", "log_response", {"entry" : sample_response1})
  const service_log2 = {
    response_hash: addr2.Ok,
    client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1tZKk"
  }
  app.call("service", "log_service", {"entry": service_log1})

  const result = app.call("service", "generate_invoice", {})

  t.deepEqual(result, { Ok: 'Qmby4AKM773kXEtSue49GA2LHFEcWMiVYT4mp1CNBUE6Ex' })
})


// 6. Checks if the unpaid value is greater than the PaymentPrefs, then call stop_hosting() on Hosting App
