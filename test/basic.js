const { one } = require('./config')
const util = require('./util')

module.exports = scenario => {


// Basic agentId check
scenario('agentId', async (s, t) => {
    const { app } = await s.players({app: one('app')}, true)

    t.ok(app.info('app').agentAddress)
})

const setup_prefs = {
  dna_bundle_hash: "QmfAzihC8RVNLCwtDeeUH8eSAACweFq77KBK4e1bJWmU8A",
}

const sample_request1 = {
  "agent_id": "HcSCIp5KE88N7OwefwsKhKgRfJyr465fgikyphqCIpudwrcivgfWuxSju9mecor",
  "call_spec": {
    "zome": "blog",
    "function": "create_post"
  },
  "payload": {
    "hash": "QmNLei78zWmzUdbeRB3CiUfAizWUrbeeZh5K1rhAQKCh51",
    "signature": "PaHr36lu3RgdvjZZ0cBRxDHwVqWtapemDVzKEEYEOHg1RkYeMShfxZ+RxwcmQnRQYeJFHV/zO8zYw8dNq8r2Cg=="
  }
}

const sample_request2 = {
  "agent_id": "HcSCIp5KE88N7OwefwsKhKgRfJyr465fgikyphqCIpudwrcivgfWuxSju9mecor",
  "call_spec": {
    "zome": "blog",
    "function": "create_post"
  },
  "payload": {
    "hash": "QmNQa1FSTXNHmrjjfgUW3Px3Vkke4oKiFWdigWkYSux2Pi",
    "signature": "sMZaWZu090wdnOcCxAeGG8SeTDyi+T7SWd+9WovxFcvTmvg3jRrX/6oPMm+40VMIBced9LpqR4Oo22qBH30RCw=="
  }
}

const sample_response1 = {
  request_commit: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1x4",
  hosting_stats: {
    cpu_seconds: 3.2,
    bytes_in: 12309,
    bytes_out: 7352,
  },
  response_digest: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
  response_log: '64.242.88.10 - - [07/Mar/2004:16:11:58 -0800] "GET /twiki/bin/view/TWiki/WikiSyntax HTTP/1.1" 200 7352',
  host_signature: "PaHr36lu3RgdvjZZ0cBRxDHwVqWtapemDVzKEEYEOHg1RkYeMShfxZ+RxwcmQnRQYeJFHV/zO8zYw8dNq8r2Cg=="
}

const sample_response2 = {
  request_commit: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1x4",
  hosting_stats: {
    cpu_seconds: 3.3,
    bytes_in: 4332,
    bytes_out: 7352,
  },
  response_digest: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
  response_log: '64.242.88.10 - - [07/Mar/2004:16:11:59 -0800] "GET /twiki/bin/view/TWiki/WikiSyntax HTTP/1.1" 200 5678',
  host_signature: "PaHr36lu3RgdvjZZ0cBRxDHwVqWtapemDVzKEEYEOHg1RkYeMShfxZ+RxwcmQnRQYeJFHV/zO8zYw8dNq8r2Cg=="
}


// 1. The client starts a new request for hosting, based on a call from the HC Interceptor
scenario('can log a client request', async (s, t) => {
    const { app } = await s.players({app: one('app')}, true)

    var whoami = await app.call('app', "service", "whoami", {})
    console.log("***DEBUG***: whoami == " + JSON.stringify( whoami ));
    t.deepEqual(util.get( ['Ok', 'dna_name'], whoami ), "ServiceLogger")
    
    var setup = await app.call('app', "service", "setup", {"entry": setup_prefs})
    console.log("***DEBUG***: setup == " + JSON.stringify( setup ));

    const addr = await app.call('app', "service", "log_request", sample_request1)

    t.deepEqual(addr, { Ok: 'QmV4ec4r7bVKxeH6H96nmNozDwFLSZTa8TBCmc3KRAfJs5' })

    const result = await app.call('app', "service", "get_request", {"address": addr.Ok})
    console.log("***DEBUG***: get_request == " + JSON.stringify( result ));

    t.deepEqual( util.get( ['Ok', 'meta', 'address'], result ), addr.Ok )
    t.deepEqual( util.get( ['Ok', 'request', 'payload', 'hash'], result ), sample_request1.payload.hash )

    // Ensure that signature validation occurs
    let sample_request1_badsig = {
	...sample_request1,
	payload: {
	    hash: sample_request1.payload.hash,
	    signature: "XxHr36lu3RgdvjZZ0cBRxDHwVqWtapemDVzKEEYEOHg1RkYeMShfxZ+RxwcmQnRQYeJFHV/zO8zYw8dNq8r2Cg=="
	}
    }
    const sig_fail = await app.call('app', "service", "log_request", sample_request1_badsig )
    console.log("***DEBUG***: sig_fail == " + JSON.stringify( sig_fail ))
    let sig_fail_err = util.get( ['Err', 'Internal'], sig_fail )
    console.log("***DEBUG***: sig_fail_err == " + JSON.stringify( sig_fail_err ))
    t.ok(sig_fail_err && sig_fail_err.includes("Signature invalid"),
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
	hosting_stats: {
	    cpu_seconds: 3.2,
	    bytes_in: 12309,
	    bytes_out: 7352,
	},
	response_digest: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
	response_log: '64.242.88.10 - - [07/Mar/2004:16:11:58 -0800] "GET /twiki/bin/view/TWiki/WikiSyntax HTTP/1.1" 200 7352',
	host_signature: "XxHr36lu3RgdvjZZ0cBRxDHwVqWtapemDVzKEEYEOHg1RkYeMShfxZ+RxwcmQnRQYeJFHV/zO8zYw8dNq8r2Cg=="
    }
    const rsp_fail = await app.call('app', "service", "log_response", {"entry" : bad_response})
    console.log("***DEBUG***: rsp_fail == " + JSON.stringify( rsp_fail ));
    let rsp_fail_err = util.get( ['Err', 'Internal'], rsp_fail )
    console.log("***DEBUG***: rsp_fail_err == " + JSON.stringify( rsp_fail_err ));
    t.ok(rsp_fail_err && rsp_fail_err.includes("ClientRequest entry not found!"),
	 "should generate a not found:" + JSON.stringify( rsp_fail ))

    // Log a valid response
    const response = {
	request_commit: request_addr.Ok,
	hosting_stats: {
	    cpu_seconds: 3.2,
	    bytes_in: 12309,
	    bytes_out: 7352,
	},
	response_digest: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
	response_log: '64.242.88.10 - - [07/Mar/2004:16:11:58 -0800] "GET /twiki/bin/view/TWiki/WikiSyntax HTTP/1.1" 200 7352',
	host_signature: "XxHr36lu3RgdvjZZ0cBRxDHwVqWtapemDVzKEEYEOHg1RkYeMShfxZ+RxwcmQnRQYeJFHV/zO8zYw8dNq8r2Cg=="
    }
    const addr = await app.call('app', "service", "log_response", {"entry" : response})

    const result = await app.call('app', "service", "get_response", {"address": addr.Ok})

    let host_response_entry = util.get( ['Ok', 'App'], result );
    t.ok( host_response_entry )
    if ( host_response_entry ) {
	// The tuple "type","Entry JSON"' is returned
	let host_response = JSON.parse( host_response_entry[1] )
	console.log("***DEBUG***: host_response == " + JSON.stringify( host_response ))
	t.deepEqual(host_response, response)
    }
})

// 3. With the client signature on that HostResponse, the Conductor creates a ServiceLog, that is a billable log
scenario('can create a servicelog', async (s, t) => {
    const { app } = await s.players({app: one('app')}, true)

    // performs initial setup
    await app.call('app', "service", "setup", {"entry": setup_prefs})  

    // Logs a sample request
    const req = await app.call('app', "service", "log_request", sample_request1)

    const addr = await app.call('app', "service", "log_response", {
	entry: {
	    ...sample_response1,
	    request_commit: req.Ok
	}
    })
    console.log("***DEBUG***: log_response: "+JSON.stringify( addr ))
    t.deepEqual( addr, { Ok: 'QmetEa4CgG6KQrWqR7ekBo4qc5LMMdczd24iqafUvaBE9t' })

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
	entry: {
	    ...sample_response1,
	    request_commit: req1.Ok
	}
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
	entry: {
	    ...sample_response2,
	    request_commit: req2.Ok
	}
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
