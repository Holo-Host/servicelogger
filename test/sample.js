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
      "call_spec": {
	"args_hash": "QmNLei78zWmzUdbeRB3CiUfAizWUrbeeZh5K1rhAQKCh51",
	"dna_alias": "openbook",
	"function": "create_post",
	"hha_hash": "QmNLei78zWmzUdbeRB3CiUfAizWUrbeeZh5K1rhAQKCh51",
	"zome": "blog"
      },
      "host_id": "HcScJhCTAB58mkeen7oKZrgxga457b69h7fV8A9CTXdTvjdx74fTp33tpcjitgz",
      "timestamp": "2019-11-25T05:48:34.123+07:00"
    },
    "request_signature": "MnElftn6vD54G8Ft+AtxrK1WNRvWu2fUeD064ZIFZZdnVtslfdMXqd/sBA9J61rfHsBv7x/hgBhoEK2dTGciCg=="
  },

  request2: {
    "agent_id": "HcSCIp5KE88N7OwefwsKhKgRfJyr465fgikyphqCIpudwrcivgfWuxSju9mecor",
    "request": {
      "call_spec": {
	"args_hash": "QmfZy5bvk7a3DQAjCbGNtmrPXWkyVvPrdnZMyBZ5q5ieKG",
	"dna_alias": "openbook",
	"function": "create_post",
	"hha_hash": "QmNLei78zWmzUdbeRB3CiUfAizWUrbeeZh5K1rhAQKCh51",
	"zome": "blog"
      },
      "host_id": "HcScJhCTAB58mkeen7oKZrgxga457b69h7fV8A9CTXdTvjdx74fTp33tpcjitgz",
      "timestamp": "2019-11-25T05:48:34.123+07:00"
    },
    "request_signature": "LE9b3HhK1KHfebAfOuT7FX6lG+xw3i83damlPQCbSQyMMHYeiUstLet5ixSjskhI8mjOhIC2y50PhwSF2EtTCw=="
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
    "response_commit": "QmQTYWmk2rY5hQQECHMM8kcjxve3CdYMbXWs62fUP4HZMq",
    "confirmation": {
      "client_metrics": {
	"duration": "1.23s"
      },
      "response_hash": "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv"
    },
    "confirmation_signature": "AR3nbCTpPXCGzfpUlwsk0igmNCIiCuTHbZBMkrdyCZybLsvUtU6gPNhnKyPVX52IMXlmZQXlQAC20OeKp0xjDQ=="
  },

  service2: {
    "agent_id": "HcSCIp5KE88N7OwefwsKhKgRfJyr465fgikyphqCIpudwrcivgfWuxSju9mecor",
    "response_commit": "QmNgujiSXAkSoP2Cf5TfEZfvE1sTAfNnVKNfuXSd1nNj8G",
    "confirmation": {
      "client_metrics": {
	"duration": "1.23s"
      },
      "response_hash": "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aXyzv"
    },
    "confirmation_signature": "Gi8B1+5/txEulO6AimUhgvGpBe47GJ9jgJzsmj6MU+S21qCd+MvOZEVm17D7RWGpQXGP7M8OZu1DLNvD1v4WBw=="
  }
}
