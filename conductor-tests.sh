#!/bin/bash

# Perform setup
curl -d '{"jsonrpc": "2.0", "id": "0", "method": "call", "params": { "instance_id": "logger-instance", "zome": "service", "function": "setup", "args": { "entry": {"dna_bundle_hash": "Hchaushduahsiduahisduhaisduhaisu" } } } }' -H "Content-Type: application/json" -X POST http://localhost:4000/

# List service logs
curl -d '{"jsonrpc": "2.0", "id": "0", "method": "call", "params": { "instance_id": "logger-instance", "zome": "service", "function": "list_uninvoiced_servicelogs", "args": { } } }' -H "Content-Type: application/json" -X POST http://localhost:4000/
