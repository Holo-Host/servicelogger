const path = require('path')
const { Config } = require('@holochain/tryorama')

const dnaLoad = (dnaName, dnaId) => {
    const dnaPath = path.join(__dirname, `../dist/${dnaName}.dna.json`)
    const dna = Config.dna(dnaPath, dnaId)
    return [dnaName, dnaId, dnaPath, dna]
}

let [dnaServName, dnaServId, dnaServPath, dnaServ] = dnaLoad( "servicelogger", "service" )
let [dnaFuelName, dnaFuelId, dnaFuelPath, dnaFuel] = dnaLoad( "holofuel", "transactions" )
let [dnaHostName, dnaHostId, dnaHostPath, dnaHost] = dnaLoad( "holo-hosting-app", "hosting" )


const networkType = process.env.APP_SPEC_NETWORK_TYPE || "sim2h"
let network = {}
// override the transport_config if we are in the Final Exam context!
if (process.env.HC_TRANSPORT_CONFIG) {
    network=require(process.env.HC_TRANSPORT_CONFIG)
} else {
    network =
        ( networkType === 'websocket'
          ? Config.network('websocket')

          : networkType === 'memory'
          ? Config.network('memory')

          : networkType === 'sim1h'
          ? {
              type: 'sim1h',
              dynamo_url: 'http://localhost:8000'
          }

          : networkType === 'sim2h'
          ? {
              type: 'sim2h',
              sim2h_url: 'ws://localhost:9000'
          }

          : (() => {throw new Error(`Unsupported network type: ${networkType}`)})()
        )
}

const logger = {
  type: 'debug',
  rules: {
    rules: [
      {
        exclude: true,
        pattern: '.*parity.*'
      },
      {
        exclude: true,
        pattern: '.*mio.*'
      },
      {
        exclude: true,
        pattern: '.*tokio.*'
      },
      {
        exclude: true,
        pattern: '.*hyper.*'
      },
      {
        exclude: true,
        pattern: '.*rusoto_core.*'
      },
      {
        exclude: true,
        pattern: '.*want.*'
      },
      {
        exclude: true,
        pattern: '.*rpc.*'
      }
      // ,
      // {
      //   exclude: true,
      //   pattern: '.*holochain_net.*'
      // },
      // {
      //   exclude: true,
      //   pattern: '.*holochain_net.*'
      // },
      // {
      //   exclude: true,
      //   pattern: '.*holochain_metrics.*'
      // }
    ]
  },
  state_dump: true
    // dpki: {
    //   instance_id: 'dpki_happ',
    //   init_params: {"revocation_key": "HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi","signed_auth_key":"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA=="}
    // },
}

const commonConfig = { logger, network }

module.exports = {
    one: (agent) => {
	const conf = Config.gen([{
	    id: 'app',
	    agent: {
		id: agent,
		name: `${agent}`,
		test_agent: true,
		public_address: "",
		keystore_file: ""
	    },
	    dna: {
		id: dnaServId,
		file: dnaServPath,
	    }
	}], commonConfig );
	return async function() {
	    const genconf = await conf.apply(conf, arguments);
	    // console.log("DEBUG: Gen config\n", genconf);
	    return genconf;
	};
    },

  bri: (agent) => {
    console.log(`bri( ${agent} )`)
    return Config.gen([{
      id: 'serv',
      agent: {
	id: agent,
	name: `${agent}`,
	test_agent: true,
	public_address: "",
	keystore_file: ""
      },
      dna: {
	id: dnaServId,
	file: dnaServPath,
      }
    }, {
      id: 'fuel',
      agent: {
	id: agent,
	name: `${agent}`,
	test_agent: true,
	public_address: "HcScJhCTAB58mkeen7oKZrgxga457b69h7fV8A9CTXdTvjdx74fTp33tpcjitgz",
	keystore_file: ""
      },
      dna: {
	id: dnaFuelId,
	file: dnaFuelPath,
      }
    }, {
      id: 'host',
      agent: {
	id: agent,
	name: `${agent}`,
	test_agent: true,
	public_address: "",
	keystore_file: ""
      },
      dna: {
	id: dnaHostId,
	file: dnaHostPath,
      }
    }], {
      bridges: [
	Config.bridge('holofuel-bridge', 'serv', 'fuel'),
  // Removed this bridge to test that all the checks pass and are not dependednt on the HHA
	// Config.bridge('hosting-bridge',  'serv', 'host')
      ],
      ...commonConfig
    })
  },
  /*
    bri: Config.gen({
	serv: dnaServ,
	fuel: dnaFuel,
	host: dnaHost
    }, {
	bridges: [
	    Config.bridge('holofuel-bridge', 'serv', 'fuel'),
	    Config.bridge('hosting-bridge',  'serv', 'host')
	],
	...commonConfig
    }),
  */
}
