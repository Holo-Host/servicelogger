//"use strict"; // locks up tests for some reason
// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape

/*
 * Try-o-rama
 */
const { Orchestrator, tapeExecutor, singleConductor, combine, callSync } = require('@holochain/try-o-rama')

const MIN_EXPECTED_SCENARIOS = 1

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dumbWaiter = interval => (run, f) => run(s =>
  f(Object.assign({}, s, {
    consistency: () => new Promise(resolve => {
      console.log(`dumbWaiter is waiting ${interval}ms...`)
      setTimeout(resolve, interval)
    })
  }))
                                              )


let transport_config = 'memory';
let middleware = combine(
  // by default, combine conductors into a single conductor for in-memory networking
  // NB: this middleware makes a really huge difference! and it's not very well tested,
  // as of Oct 1 2019. So, keep an eye out.
  singleConductor,
  callSync,
  tapeExecutor(require('tape')),
);


/*
 * End-to-end testing; default assumes sim2h_server on localhost:9000
 */
const APP_SPEC_NETWORK_TYPE = process.env.APP_SPEC_NETWORK_TYPE || "sim2h"

if (APP_SPEC_NETWORK_TYPE === "websocket")
{
  transport_config = "websocket"

  // omit singleConductor
  middleware = combine(
    callSync,
    tapeExecutor(require('tape')),
  );
}
else if (APP_SPEC_NETWORK_TYPE === "sim1h") // default
{
    transport_config = {
	type: 'sim1h',
	dynamo_url: "http://localhost:8000",
    }

    // omit singleConductor
    middleware = combine(
	// dumbWaiter(1000),
	callSync,
	tapeExecutor(require('tape')),
    );
}
else if (APP_SPEC_NETWORK_TYPE === "sim2h")
{
    transport_config = {
        type: 'sim2h',
        sim2h_url: "wss://localhost:9000",
    }

    // omit singleConductor
    middleware = combine(
        // dumbWaiter(1000),
        callSync,
        tapeExecutor(require('tape')),
    );
}

// override the transport_config if we are in the Final Exam context!
if (process.env.HC_TRANSPORT_CONFIG) {
    transport_config=require(process.env.HC_TRANSPORT_CONFIG)
}


const orchestrator = new Orchestrator({
    middleware,
    waiter: {
	softTimeout: 5000,
	hardTimeout: 10000,
    },
    globalConfig: {
	logger: {
            type: "debug",
            rules: {
		rules: [
                    {
			exclude: true,
			pattern: ".*parity.*"
                    },
                    {
			exclude: true,
			pattern: ".*mio.*"
                    },
                    {
			exclude: true,
			pattern: ".*tokio.*"
                    },
                    {
			exclude: true,
			pattern: ".*hyper.*"
                    },
                    {
			exclude: true,
			pattern: ".*rusoto_core.*"
                    },
                    {
			exclude: true,
			pattern: ".*want.*"
                    },
                    {
			exclude: true,
			pattern: ".*rpc.*"
                    }
		]
            },
            state_dump: true,
	},
	network: transport_config
    }
})

// Run the various scenerio test packages

require('./basic')(orchestrator.registerScenario)


// Check to see that we haven't accidentally disabled a bunch of scenarios
const num = orchestrator.numRegistered()
if (num < MIN_EXPECTED_SCENARIOS) {
    console.error(`Expected at least ${MIN_EXPECTED_SCENARIOS} scenarios, but only ${num} were registered!`)
    process.exit(1)
}
else {
    console.log(`Registered ${num} scenarios (at least ${MIN_EXPECTED_SCENARIOS} were expected)`)
}

orchestrator.run().then(stats => {
    console.log("All done.")
})
