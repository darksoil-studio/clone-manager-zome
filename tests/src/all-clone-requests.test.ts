import { pause, runScenario } from '@holochain/tryorama';
import { encode } from '@msgpack/msgpack';
import { assert, test } from 'vitest';

import { setup, waitUntil } from './setup.js';

test('make service request', async () => {
	await runScenario(async scenario => {
		const [alice, bob] = await setup(scenario);

		await alice.store.client.createCloneRequest({
			dna_modifiers: {
				network_seed: 'hi',
				properties: encode({}),
			},
		});

		await pause(5000);

		const requests = await bob.store.client.getAllCloneRequests();

		assert.equal(Object.keys(requests).length, 1);
	});
});
