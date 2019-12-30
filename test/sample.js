const host_metrics = {
  duration: "3.2s",
  /*
   * These are only meaningfully collectible on a per-invoice period.
   *
   cpu: { elapsed: "3.2s", system: "600ms", user: "1.2s", load: 1.8 },
   network: { i: 12309, o: 7352 },
   storage: 4603
   *
   */
};

module.exports = {
  host_metrics,
  request1: {
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
  },

  request2: {
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
  },
    
  response1: {
    request_commit: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1x4",
    response_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
    host_metrics,
    entries: []
  },

  response2: {
    request_commit: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aStuv",
    response_hash: "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aXyzv",
    host_metrics,
    entries: []
  },

  service1: {
    "agent_id": "HcSCIp5KE88N7OwefwsKhKgRfJyr465fgikyphqCIpudwrcivgfWuxSju9mecor",
    "response_commit": "Qmc8zvqELGCBCykoKnFuvLquCsSVNVBN3Lp2eEcJdHNakd",
    "confirmation": {
      "response_hash": "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv",
      "client_metrics": {
	"duration": "1.23s"
      }
    },
    "confirmation_signature": "IrXZ4MRuaIMeN6NtaobSPahlTfQqL+ykLdNUT91tie1qAqT4DC/WXEq1yskwSIKJbg9Qkd1UqVhfOXmXhihCAQ=="
  },

  service2: {
    "agent_id": "HcSCIp5KE88N7OwefwsKhKgRfJyr465fgikyphqCIpudwrcivgfWuxSju9mecor",
    "response_commit": "QmU84Rqgs2bzBDYsp2too1oR2HYnrG5KxAMYBkcrPzjJ5w",
    "confirmation": {
      "response_hash": "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aXyzv",
      "client_metrics": {
	"duration": "1.23s"
      }
    },
    "confirmation_signature": "RYpDOlbmNJKSiK/9c5OF2yEum7QCJqiOdP5XxorD/nGggEvCXRva4yZXgYoDXiPS0hAz+ak42HMeuicbQ2CeDw=="
  }

}
