const _ = require('lodash')
const path = require('path')
const { Config } = require('@holochain/try-o-rama')

const dnaName = "servicelogger"
const dnaId = "service"

const dnaPath = path.join(__dirname, `../dist/${dnaName}.dna.json`)
const dna = Config.dna(dnaPath, dnaId)

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
	    id: dnaId,
	    file: dnaPath,
	}
    }],
})

module.exports = { one }
