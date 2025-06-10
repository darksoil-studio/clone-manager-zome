import { dhtSync, pause, runScenario } from '@holochain/tryorama';
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

		await dhtSync([alice.player, bob.player], alice.player.cells[0].cell_id[0]);

		const requests = await bob.store.client.getAllCloneRequests();

		assert.equal(Object.keys(requests).length, 1);
	});
});
