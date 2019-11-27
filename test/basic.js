const { one } = require('./config')
const util = require('./util')
const json_stable_stringify = require('json-stable-stringify')

module.exports = scenario => {


// Basic agentId check
scenario('agentId', async (s, t) => {
    const { app } = await s.players({app: one('app')}, true)

    t.ok(app.info('app').agentAddress)
})

const setup_prefs = {
  dna_bundle_hash: "QmfAzihC8RVNLCwtDeeUH8eSAACweFq77KBK4e1bJWmU8A",
}

// TODO: these are constructed w/ a host_id that *matches* the default one generated for Sim2h Scenario tests,
// because hdk::sign doesn't allow signing of arbitrary JSON-serialized data (ie. w/ escapes).
const sample_request1 = {
  "agent_id": "HcSCIp5KE88N7OwefwsKhKgRfJyr465fgikyphqCIpudwrcivgfWuxSju9mecor",
  "request": {
    "timestamp": "2019-11-25T05:48:34.123+07:00",
    "host_id": "HcScJhCTAB58mkeen7oKZrgxga457b69h7fV8A9CTXdTvjdx74fTp33tpcjitgz",
    "call_spec": {
      "hha_hash": "QmNLei78zWmzUdbeRB3CiUfAizWUrbeeZh5K1rhAQKCh51",
      "dna_alias": "openbook",
      "zome": "blog",
      "function": "create_post",
      "args_hash": "QmNLei78zWmzUdbeRB3CiUfAizWUrbeeZh5K1rhAQKCh51"
    }
  },
  "request_signature": "eILE1NJsxw2ANRpKzKV1r9J6pJDJlTnDtWKoAWOuR6h5FydqgGIexqucNi/yZLHmRT7OFFdR4dangGQjk86OAA=="
}

const sample_request2 = {
  "agent_id": "HcSCIp5KE88N7OwefwsKhKgRfJyr465fgikyphqCIpudwrcivgfWuxSju9mecor",
  "request": {
    "timestamp": "2019-11-25T05:48:34.123+07:00",
    "host_id": "HcScJhCTAB58mkeen7oKZrgxga457b69h7fV8A9CTXdTvjdx74fTp33tpcjitgz",
    "call_spec": {
      "hha_hash": "QmNLei78zWmzUdbeRB3CiUfAizWUrbeeZh5K1rhAQKCh51",
      "dna_alias": "openbook",
      "zome": "blog",
      "function": "create_post",
      "args_hash": "QmfZy5bvk7a3DQAjCbGNtmrPXWkyVvPrdnZMyBZ5q5ieKG"
    }
  },
  "request_signature": "aaHZg6qaeRhbiYoJCN9oN3vxJIsuVigQxH2OTDWvfVHRk7QbRBHT+Ay0k50q94VKGRe1J+lq1YRhK1l5BgarAg=="
}

const host_metrics = {
    cpu: { elapsed: "3.2s", system: "600ms", user: "1.2s", load: 1.8 },
    network: { i: 12309, o: 7352 },
    storage: 4603
};

const sample_response1 = {
    request_commit: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1x4",
    response_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
    host_metrics,
    entries: []
}

const sample_response2 = {
    request_commit: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aStuv",
    response_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aXyzv",
    host_metrics,
    entries: []
}


// 1. The client starts a new request for hosting, based on a call from the HC Interceptor
scenario('can log a client request', async (s, t) => {
    const { app } = await s.players({app: one('app')}, true)

    var whoami = await app.call('app', "service", "whoami", {})
    console.log("***DEBUG***: whoami == " + JSON.stringify( whoami ));
    t.deepEqual(util.get( ['Ok', 'dna_name'], whoami ), "ServiceLogger")
    
    var setup = await app.call('app', "service", "setup", {"entry": setup_prefs})
    console.log("***DEBUG***: setup == " + JSON.stringify( setup ));

    // We need to compute the correct signature for a request w/ a private key we have -- our own.

    var test1 = await app.call('app', "service", "sign", { payload: "'hello'" })
    console.log(`***DEBUG***: test1 == ` + JSON.stringify( test1 ));
    t.ok(test1.Ok)

    var test2 = await app.call('app', "service", "sign", { payload: "\"hello\"" })
    console.log(`***DEBUG***: test2 == ` + JSON.stringify( test2 ));
  //t.ok(test2.Ok) // TODO: should succeed

    var signature1 = await app.call('app', "service", "sign", {
	payload: json_stable_stringify( sample_request1.request )
    })
    console.log(`***DEBUG***: signature1 == ` + JSON.stringify( signature1 ));
  //t.ok(signature1.Ok) // TODO: should succeed
    var request1 = {
	...sample_request1,
	request_signature: signature1.Ok
    }
    // Use the request with the computed signature
  //const addr = await app.call('app', "service", "log_request", request1)

    const addr = await app.call('app', "service", "log_request", sample_request1)

    t.deepEqual(addr, { Ok: 'QmeQbPutRefwE7SRwZrgCZguj5Zn9zYNZiNEZb5Sdb671a' })

    const result = await app.call('app', "service", "get_request", {"address": addr.Ok})
    console.log("***DEBUG***: get_request == " + JSON.stringify( result ));

    t.deepEqual( util.get( ['Ok', 'meta', 'address'], result ), addr.Ok )
    t.deepEqual( util.get( ['Ok', 'client_request', 'request', 'call_spec', 'args_hash'], result ),
		 sample_request1.request.call_spec.args_hash )


    // Ensure that host_id and signature validation occurs.
    let request1_bad_host = {
	agent_id: sample_request1.agent_id,
	request: {
	    timestamp: sample_request1.request.timestamp,
	    host_id: sample_request1.agent_id, // valid, but not correct
	    call_spec: sample_request1.request.call_spec
	},
	request_signature: sample_request1.request_signature
    }
    const host_fail = await app.call('app', "service", "log_request", request1_bad_host )
    console.log("***DEBUG***: sig_fail == " + JSON.stringify( host_fail ))
    let host_fail_err = util.get( ['Err', 'Internal'], host_fail )
    console.log("***DEBUG***: host_fail_err == " + JSON.stringify( host_fail_err ))
    t.ok(host_fail_err && host_fail_err.includes("doesn't match request"),
	 "should generate an 'Host Agent ... doesn't match: " + JSON.stringify( host_fail ))

    let request1_bad_sig = {
	...sample_request1,
	request_signature: "XxHr36xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxCg=="
    }
    const sig_fail = await app.call('app', "service", "log_request", request1_bad_sig )
    console.log("***DEBUG***: sig_fail == " + JSON.stringify( sig_fail ))
    let sig_fail_err = util.get( ['Err', 'Internal'], sig_fail )
    console.log("***DEBUG***: sig_fail_err == " + JSON.stringify( sig_fail_err ))
    t.ok(sig_fail_err && sig_fail_err.includes("invalid for request"),
	 "should generate an 'Signature invalid ...': " + JSON.stringify( sig_fail ))
})


// 2. The Conductor wants to record a HostResponse, indicating some hosting was done
scenario('can log a host response', async (s, t) => {
    const { app } = await s.players({app: one('app')}, true)

    // performs initial setup
    await app.call('app', "service", "setup", {"entry": setup_prefs})

    const request_addr = await app.call('app', "service", "log_request", sample_request1)

    // try to log a response with a bad request_commit
    const bad_response = {
	request_commit: "xxxxxxx-fake-address-xxxxxxx",
	response_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
	host_metrics,
	entries: []
    }
    const rsp_fail = await app.call('app', "service", "log_response", bad_response)
    console.log("***DEBUG***: rsp_fail == " + JSON.stringify( rsp_fail ));
    let rsp_fail_err = util.get( ['Err', 'Internal'], rsp_fail )
    console.log("***DEBUG***: rsp_fail_err == " + JSON.stringify( rsp_fail_err ));
    t.ok(rsp_fail_err && rsp_fail_err.includes("ClientRequest entry not found!"),
	 "should generate a not found:" + JSON.stringify( rsp_fail ))

    // Log a valid response
    const response = {
	request_commit: request_addr.Ok,
	response_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
	host_metrics,
	entries: []
    }
    const addr = await app.call('app', "service", "log_response", response)

    const result = await app.call('app', "service", "get_response", {"address": addr.Ok})
    console.log("***DEBUG***: get_response == " + JSON.stringify( result ));

    t.deepEqual( util.get( ['Ok', 'meta', 'address'], result ), addr.Ok )
    t.deepEqual( util.get( ['Ok', 'host_response', 'response_hash'], result ), response.response_hash )
})

// 3. With the client signature on that HostResponse, the Conductor creates a ServiceLog, that is a billable log
scenario('can create a servicelog', async (s, t) => {
    const { app } = await s.players({app: one('app')}, true)

    // performs initial setup
    await app.call('app', "service", "setup", {"entry": setup_prefs})  

    // Logs a sample request
    const req = await app.call('app', "service", "log_request", sample_request1)

    const addr = await app.call('app', "service", "log_response", {
	...sample_response1,
	request_commit: req.Ok
    })
    console.log("***DEBUG***: log_response: "+JSON.stringify( addr ))
    t.deepEqual( addr, { Ok: 'QmaaReBEEeuxNtVHEWr4fcQvwCHsfzANYAgPiorBwYKYAq' })

    // try to log a bad service_log 
    const bad_service_log = {
	response_commit: "xxx-fakeaddr-xxx",
	client_signature: "XxHr36lu3RgdvjZZ0cBRxDHwVqWtapemDVzKEEYEOHg1RkYeMShfxZ+RxwcmQnRQYeJFHV/zO8zYw8dNq8r2Cg=="
    }
    const failure = await app.call('app', "service", "log_service", {"entry": bad_service_log})
    t.ok(failure.Err.Internal.includes("HostResponse entry not found!"), "should generate an error")

    // then log an actual service_log
    const service_log = {
	response_commit: addr.Ok,
	client_signature: "XxHr36lu3RgdvjZZ0cBRxDHwVqWtapemDVzKEEYEOHg1RkYeMShfxZ+RxwcmQnRQYeJFHV/zO8zYw8dNq8r2Cg=="
    }

    const addr2 = await app.call('app', "service", "log_service", {"entry": service_log})
    console.log("***DEBUG***: log_service: "+JSON.stringify( addr2 ))

    const result = await app.call('app', "service", "get_service", {"address": addr2.Ok})
    console.log("***DEBUG***: get_service: " + JSON.stringify( result ))

    t.deepEqual(result, { Ok: { App: [ 'service_log', JSON.stringify(service_log) ] } })
})

// 4. List all billable ServiceLogs. TODO: filter by start and end time (for pagination)
scenario('log then list all servicelog', async (s, t) => {
    const { app } = await s.players({app: one('app')}, true)

    // performs initial setup
    await app.call('app', "service", "setup", {"entry": setup_prefs})  

    // Logs a sample request
    const req1 = await app.call('app', "service", "log_request", sample_request1)
    console.log("***DEBUG***: log_request 1: "+JSON.stringify( req1 ))

    // Log a first response & service_log
    const addr1 = await app.call('app', "service", "log_response", {
	...sample_response1,
	request_commit: req1.Ok
    })
    console.log("***DEBUG***: log_response 1: "+JSON.stringify( addr1 ))

    const service_log1 = {
	response_commit: addr1.Ok,
	client_signature: "XxHr36lu3RgdvjZZ0cBRxDHwVqWtapemDVzKEEYEOHg1RkYeMShfxZ+RxwcmQnRQYeJFHV/zO8zYw8dNq8r2Cg=="
    }
    const sl_addr1 = await app.call('app', "service", "log_service", {
	entry: service_log1
    })
    console.log("***DEBUG***: log_service 1: "+JSON.stringify( sl_addr1 ))

    // Log a second response & service_log
    const req2 = await app.call('app', "service", "log_request", sample_request2)
    const addr2 = await app.call('app', "service", "log_response", {
	...sample_response2,
	request_commit: req2.Ok
    })
    console.log("***DEBUG***: log_response 2: "+JSON.stringify( addr2 ))
    const service_log2 = {
	response_commit: addr2.Ok,
	client_signature: "XxHr36lu3RgdvjZZ0cBRxDHwVqWtapemDVzKEEYEOHg1RkYeMShfxZ+RxwcmQnRQYeJFHV/zO8zYw8dNq8r2Cg=="
    }
    const sl_addr2 = await app.call('app', "service", "log_service", {
	entry: service_log2
    })
    console.log("***DEBUG***: log_service 2: "+JSON.stringify( sl_addr2 ))

    const results = await app.call('app', "service", "list_uninvoiced_servicelogs", {})

    t.deepEqual(results.Ok.sort(), [sl_addr1.Ok, sl_addr2.Ok].sort())
})

}
