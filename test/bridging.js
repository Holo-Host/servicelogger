const path = require('path')

const { bri } = require('./config')
const util = require('./util')

sample = require('./sample')

module.exports = scenario => {

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

async function perform_hosting_setup(conductor, host, fuel) {
  const test_name = "***DEBUG***: Holo Hosting Setup: "

  // Perform Holohost setup
  const reg_pro = await conductor.callSync(host, "provider", "register_as_provider", Provider_Doc)
  console.log(test_name+"Register as Provider: " + JSON.stringify( reg_pro, null, 4 ))

  // Add the holofuel account to the Provider
  const fuel_account = {
    holofuel_account_details: {
      account_number : conductor.info(fuel).agentAddress
    }
  }
  const add_fuel = await conductor.callSync(host, "provider", "add_holofuel_account", fuel_account);
  console.log(test_name+"Provider Holofuel Account w/: " + JSON.stringify( fuel_account, null, 4 ) + " ==> " + JSON.stringify( add_fuel, null, 4 ))


  const app_address = await conductor.callSync(host, "provider", "register_app", App_Config);
  console.log(test_name+"APP ADDRESS:: " +JSON.stringify( app_address ));

  const payment_prefs = {
    app_hash: app_address.Ok,
    max_fuel_per_invoice: 2.0,
    max_unpaid_value: 4.0,
    price_per_unit: 1.0
  }

  const service_log_details = await conductor.callSync(host, "host", "add_service_log_details", payment_prefs);
  console.log(test_name+"Service Log Details: " +JSON.stringify( service_log_details ));

  return app_address.Ok;
}

// 5. Generate an invoice based on the selected ServiceLogs
scenario('testing invoice generation', async (s, t) => {
  const { conductor } = await s.players( { conductor: bri('app') }, true )
  const test_name = "***DEBUG***: Multi-DNA Invoice Generation: "
  
  let serv = 'serv'
  let fuel = 'fuel'
  let host = 'host'

  for ( let agent_name of [ serv, fuel, host ]) {
    let agent_id = conductor.info( agent_name ).agentAddress
    console.log(test_name+`Agent ID of ${agent_name}: ${agent_id}`)
  }
  const happ_address = await perform_hosting_setup(conductor, host, fuel);
  t.ok( happ_address )

  s.consistency()

  const setup_prefs = {
    dna_bundle_hash: happ_address,
  }

  // performs initial setup
  const serv_setup = await conductor.callSync(serv, "service", "setup", {"entry": setup_prefs})
  console.log(test_name+"Service Log setup: " +JSON.stringify( serv_setup ));

  // Log a first request, response & service
  const requ1 = await conductor.callSync(serv, "service", "log_request", sample.request1)
  t.ok( requ1.Ok )
  const addr1 = await conductor.callSync(serv, "service", "log_response", {
    ...sample.response1,
    request_commit: requ1.Ok,
  })
  t.ok( addr1.Ok )

  var serv_log_svc = await conductor.callSync(serv, "service", "log_service", {
    ...sample.service1,
    response_commit: addr1.Ok,
  })
  console.log(test_name+"Service Log service: " +JSON.stringify( serv_log_svc ));
  t.ok( serv_log_svc.Ok )

  // Check if an invoice should be generated (passed threshold)
  const serv_gen_inv = await conductor.callSync(serv, "service", "generate_invoice", {})
  console.log(test_name+"Service Generate Invoice: " +JSON.stringify( serv_gen_inv ));

  // No invoice should have been generated yet
  var invoices = await conductor.callSync(serv, "service", "list_unpaid_invoices", {})
  t.deepEqual(invoices, { Ok: [] })

  // Log a second request, response & service
  const requ2 = await conductor.callSync(serv, "service", "log_request", sample.request2)
  t.ok( requ2.Ok )
  const addr2 = await conductor.callSync(serv, "service", "log_response", {
    ...sample.response2,
    request_commit: requ2.Ok,
  })
  t.ok( addr2.Ok )
  serv_log_svc = await conductor.callSync(serv, "service", "log_service", {
    ...sample.service2,
    response_commit: addr2.Ok,
  })
  console.log(test_name+"Service Log service: " +JSON.stringify( serv_log_svc ));
  t.ok( serv_log_svc.Ok )

  // Check if an invoice should be generated (passed threshold)
  var invoice = await conductor.callSync(serv, "service", "generate_invoice", {})
  t.deepEqual(invoice, { Ok: 'QmSuEKvY4ccGFAdkBnEdXo2mGy1fEc7DGi9D1gXh4JJy3q' })

  // Now we should have an invoice
  invoices = await conductor.callSync(serv, "service", "list_unpaid_invoices", {})
  t.deepEqual(invoices, { Ok: [ 'QmSuEKvY4ccGFAdkBnEdXo2mGy1fEc7DGi9D1gXh4JJy3q' ] })

  // Check that the Invoice notes indicate the source and details of the invoice,
  const earnings = await conductor.callSync(fuel, "transactions", "list_transactions", {
    filters: {
      earnings: {
	Notes: "\"Holo_earnings\":.*"
      }
    }
  })
  console.log( test_name + `Holo Earning tx list: ` + JSON.stringify( earnings, null, 2 ))
  const notes_json = JSON.parse( util.get( [ 'Ok', 'transactions', 0, 'event', 'Request', 'notes' ], earnings ))
  t.isEqual( util.get( [ 'Holo_earnings', 'happ_domain' ], notes_json ), "app2.holo.host" )
  t.isEqual( util.get( [ 'Holo_earnings', 'records' ], notes_json ), 2 )

})

/*
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
  const app_address = await perform_hosting_setup(host, fuel);

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
  app.call("service", "generate_invoice", {})
  var invoices = app.call("service", "list_unpaid_invoices", {})
  t.deepEqual(invoices, { Ok: [] })

  // Log a second response & service_log **triggering** an invoice generation (passed threshold)
  const addr2 = app.call("service", "log_response", {"entry" : sample_response2})
  const service_log2 = {
    response_hash: addr2.Ok,
    client_signature: "QmXsSgDu7NNdAq7F9rmmHSaRz79a8njtkaYgRqxzz1tZKk"
  }
  app.call("service", "log_service", {"entry": service_log1})

  // Now we should have an invoice
  app.call("service", "generate_invoice", {})
  invoices = app.call("service", "list_unpaid_invoices", {})
  t.deepEqual(invoices, { Ok: ['QmP8GBjQi1Ehm8wsXfkv66oarDAd1xFVBF7TVgzyKjfACB'] })

  // Check if payment status is still HOSTING
  var payment_status = app.call("service", "get_payment_status", {}).Ok;
  t.deepEqual(payment_status.unpaid_value, 2.0);
  t.deepEqual(payment_status.max_unpaid_value, 4.0);
  t.deepEqual(payment_status.situation, "Hosting");

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

  app.call("service", "generate_invoice", {})
  payment_status = app.call("service", "get_payment_status", {}).Ok;
  console.log("Unpaid invoice threshold exceeded: get_payment_status:" + JSON.stringify( payment_status ))
  t.deepEqual(payment_status.unpaid_value, 6.0);
  t.deepEqual(payment_status.situation, "Stopped");

})

*/

}
