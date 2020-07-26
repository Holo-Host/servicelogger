const { one } = require('./config')
const util = require('./util')
const sample = require('./sample')

module.exports = scenario => {

scenario('Test out traffic received by the servicelogger', async (s, t) => {
    const { app } = await s.players({app: one('app')}, true)
    const setup_prefs = {
	     dna_bundle_hash: "QmfAzihC8RVNLCwtDeeUH8eSAACweFq77KBK4e1bJWmU8A",
    }

    let initial_traffic = await app.call('app', "service", "get_traffic", {filter: "SECOND"})
    t.equal(initial_traffic.Ok.value.length, 0)

    // performs initial setup
    await app.call('app', "service", "setup", {"entry": setup_prefs})

    let req = await app.call('app', "service", "log_request", sample.request1)
    let addr = await app.call('app', "service", "log_response", {
	     ...sample.response1,
	     request_commit: req.Ok
    })
    let addr2 = await app.call('app', "service", "log_service", sample.service1)

    req = await app.call('app', "service", "log_request", sample.request2)
    addr = await app.call('app', "service", "log_response", {
	     ...sample.response2,
	     request_commit: req.Ok
    })
    addr2 = await app.call('app', "service", "log_service", sample.service2)


    let traffic = await app.call('app', "service", "get_traffic", {filter: "SECOND"})
    t.equal(traffic.Ok.value[0], 1)
    t.equal(traffic.Ok.value[1], 1)
    traffic = await app.call('app', "service", "get_traffic", {filter: "HOUR"})
    t.equal(traffic.Ok.value[0], 2)

})

}
