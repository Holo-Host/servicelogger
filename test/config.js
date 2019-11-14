const _ = require('lodash')
const path = require('path')
const { Config } = require('@holochain/try-o-rama')

const dnaPath = path.join(__dirname, "../dist/servicelogger.dna.json")
const dna = Config.dna(dnaPath, 'service')

const one = (agent) => ({
    instances: [{
	id: 'app',
	agent: {
	    id: agent,
	    name: `${agent}`,
	    test_agent: true,
	    public_address: "",
	    keystore_file: ""
	},
	dna: {
	    id: 'service',
	    file: dnaPath,
	}
    }],
    // dpki: {
    //   instance_id: 'dpki_happ',
    //   init_params: {"revocation_key": "HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi","signed_auth_key":"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA=="}
    // }
})

module.exports = { one }
