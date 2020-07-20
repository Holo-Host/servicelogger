const { one } = require('./config')
const util = require('./util')
const sample = require('./sample')
const json_stable_stringify = require('json-stable-stringify')
const crypto = require('crypto')

global.window				= {}; // Bypass bug in Chaperone@0.1.5
const { Codec, KeyManager } = require("@holo-host/chaperone");

const utf8 = new TextEncoder();
const log = function(...args) {
    return console.log("MYLOGS:", ...args);
}

module.exports = scenario => {

    // Basic agentId check
    scenario('agentId', async (s, t) => {
	const { app } = await s.players({app: one('app')}, true)

	t.ok(app.info('app').agentAddress)
    })

    const setup_prefs = {
	dna_bundle_hash: "QmfAzihC8RVNLCwtDeeUH8eSAACweFq77KBK4e1bJWmU8A",
    }

    scenario('log request using real Chaperone input', async (s, t) => {
	let resp;
	const { app } = await s.players({app: one('app')}, true);

	var whoami = await app.call('app', "service", "whoami", {});
	t.deepEqual(util.get( ['Ok', 'dna_name'], whoami ), "ServiceLogger");

	const keys			= new KeyManager( crypto.randomBytes( 32 ) );
	const client_agent_id		= Codec.AgentId.encode( keys.publicKey() );
	const request = {
	    "timestamp": "2020-02-12T21:05:32.021+00:00",
	    "host_id": "HcScJhCTAB58mkeen7oKZrgxga457b69h7fV8A9CTXdTvjdx74fTp33tpcjitgz",
	    "call_spec": {
		"hha_hash": "QmUgZ8e6xE1h9fH89CNqAXFQkkKyRh2Ag6jgTNC8wcoNYS",
		"dna_alias": "holofuel",
		"zome": "transactions",
		"function": "ledger_state",
		"args_hash": "QmSvPd3sHK7iWgZuW47fyLy4CaZQe2DwxvRhrJ39VpBVMK"
	    }
	};

	const sig_bytes			= keys.sign( utf8.encode(json_stable_stringify(request)) );
	const signature			= Codec.Signature.encode( sig_bytes );

	resp = await app.call('app', "service", "log_request", {
	    "agent_id": client_agent_id,
	    "request": request,
	    "request_signature": signature,
	});

	log("log_request response:", JSON.stringify(resp) );
	t.ok( resp.Ok );
    });


// TODO: these are constructed w/ a host_id that *matches* the default one generated for Sim2h Scenario tests,
// because hdk::sign doesn't allow signing of arbitrary JSON-serialized data (ie. w/ escapes).

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
	payload: json_stable_stringify( sample.request1.request )
    })
    console.log(`***DEBUG***: signature1 == ` + JSON.stringify( signature1 ));
  //t.ok(signature1.Ok) // TODO: should succeed
    var request1 = {
	...sample.request1,
	request_signature: signature1.Ok
    }
    // Use the request with the computed signature
  //const addr = await app.call('app', "service", "log_request", request1)

    const addr = await app.call('app', "service", "log_request", sample.request1)

    t.deepEqual(addr, { Ok: 'QmaYjjLKWfr8QTbwx79q55NXxrsfk1HV2CKhPuhjrYyVJa' })

    const result = await app.call('app', "service", "get_request", {"address": addr.Ok})
    console.log("***DEBUG***: get_request == " + JSON.stringify( result ));

    t.deepEqual( util.get( ['Ok', 'meta', 'address'], result ), addr.Ok )
    t.deepEqual( util.get( ['Ok', 'client_request', 'request', 'call_spec', 'args_hash'], result ),
		 sample.request1.request.call_spec.args_hash )


    // Ensure that host_id and signature validation occurs.
    let request1_bad_host = {
	agent_id: sample.request1.agent_id,
	request: {
	    timestamp: sample.request1.request.timestamp,
	    host_id: sample.request1.agent_id, // valid, but not correct
	    call_spec: sample.request1.request.call_spec
	},
	request_signature: sample.request1.request_signature
    }
    const host_fail = await app.call('app', "service", "log_request", request1_bad_host )
    console.log("***DEBUG***: sig_fail == " + JSON.stringify( host_fail ))
    let host_fail_err = util.get( ['Err', 'Internal'], host_fail )
    console.log("***DEBUG***: host_fail_err == " + JSON.stringify( host_fail_err ))
    t.ok(host_fail_err && host_fail_err.includes("invalid for request payload"),
	 "should generate an 'ValidationFailed: " + JSON.stringify( host_fail ))

    let request1_bad_sig = {
	...sample.request1,
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

    const request_addr = await app.call('app', "service", "log_request", sample.request1)

    // try to log a response with a bad request_commit
    const bad_response = {
	request_commit: "xxxxxxx-fake-address-xxxxxxx",
	response_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
        host_metrics: sample.host_metrics,
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
        host_metrics: sample.host_metrics,
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
    const req = await app.call('app', "service", "log_request", sample.request1)

    const addr = await app.call('app', "service", "log_response", {
	...sample.response1,
	request_commit: req.Ok
    })
    console.log("***DEBUG***: log_response: "+JSON.stringify( addr ))
    t.deepEqual( addr, { Ok: 'QmQTYWmk2rY5hQQECHMM8kcjxve3CdYMbXWs62fUP4HZMq' })

    // try to log a bad service_log
    const bad_service_log = {
	...sample.service1,
	response_commit: 'QmfaKeADDresStVHEWr4fcQvwCHsfzANYAgPiorBwYKYAq',
    }
    const failure = await app.call('app', "service", "log_service", bad_service_log)
    console.log("***DEBUG***: log_service: "+JSON.stringify( failure ))
    t.ok(failure.Err.Internal.includes("HostResponse entry not found!"), "should generate an error")

    // then log an actual service_log
    const addr2 = await app.call('app', "service", "log_service", sample.service1)
    console.log("***DEBUG***: log_service: "+JSON.stringify( addr2 ))

    const result = await app.call('app', "service", "get_service", {"address": addr2.Ok})
    console.log("***DEBUG***: get_service: " + JSON.stringify( result ))

    t.deepEqual( util.get( ['Ok', 'meta', 'address'], result ), addr2.Ok )
    t.deepEqual( util.get( ['Ok', 'service_log', 'response_commit' ], result ), addr.Ok )
})

// 4. List all billable ServiceLogs. TODO: filter by start and end time (for pagination)
scenario('log then list all servicelog', async (s, t) => {
    const { app } = await s.players({app: one('app')}, true)

    // performs initial setup
    await app.call('app', "service", "setup", {"entry": setup_prefs})

    // Logs a sample request
    const req1 = await app.call('app', "service", "log_request", sample.request1)
    console.log("***DEBUG***: log_request 1: "+JSON.stringify( req1 ))
    t.ok( req1.Ok, "should have succeeded" )

    // Log a first response & service_log
    const addr1 = await app.call('app', "service", "log_response", {
	...sample.response1,
	request_commit: req1.Ok
    })
    console.log("***DEBUG***: log_response 1: "+JSON.stringify( addr1 ))
    t.ok( addr1.Ok, "should have succeeded" )

    const sl_addr1 = await app.call('app', "service", "log_service", sample.service1)
    t.ok( sl_addr1.Ok, "should have succeeded" )
    console.log("***DEBUG***: log_service 1: "+JSON.stringify( sl_addr1 ))

    // Log a second response & service_log
    const req2 = await app.call('app', "service", "log_request", sample.request2)
    t.ok( req2.Ok, "should have succeeded" )
    const addr2 = await app.call('app', "service", "log_response", {
	...sample.response2,
	request_commit: req2.Ok
    })
    console.log("***DEBUG***: log_response 2: "+JSON.stringify( addr2 ))
    t.ok( addr2.Ok, "should have succeeded" )
    const service_log2 = {
	response_commit: addr2.Ok,
	client_signature: "XxHr36lu3RgdvjZZ0cBRxDHwVqWtapemDVzKEEYEOHg1RkYeMShfxZ+RxwcmQnRQYeJFHV/zO8zYw8dNq8r2Cg=="
    }
    const sl_addr2 = await app.call('app', "service", "log_service", sample.service2 )
    console.log("***DEBUG***: log_service 2: "+JSON.stringify( sl_addr2 ))
    t.ok( sl_addr2.Ok, "should have succeeded" )

    const results = await app.call('app', "service", "list_uninvoiced_servicelogs", {})

    t.deepEqual(results.Ok.sort(), [sl_addr1.Ok, sl_addr2.Ok].sort())
})

}
