import { encodeHashToBase64 } from '@holochain/client';
import { dhtSync, pause, runScenario } from '@holochain/tryorama';
import { encode } from '@msgpack/msgpack';
import { assert, test } from 'vitest';

import { setup, waitUntil } from './setup.js';

test('make service request', async () => {
	await runScenario(async scenario => {
		const [alice, bob] = await setup(scenario);

		const cloneRequestHash = await alice.store.client.createCloneRequest({
			dna_modifiers: {
				network_seed: 'hi',
				properties: encode({}),
			},
		});

		await dhtSync([alice.player, bob.player], alice.player.cells[0].cell_id[0]);

		const requests = await bob.store.client.getAllCloneRequests();

		assert.equal(Object.keys(requests).length, 1);

		await bob.store.client.announceAsCloneProviderForRequest(cloneRequestHash);

		await dhtSync([alice.player, bob.player], alice.player.cells[0].cell_id[0]);

		let providers =
			await alice.store.client.getCloneProvidersForRequest(cloneRequestHash);

		assert.equal(providers.length, 1);
		assert.equal(
			encodeHashToBase64(providers[0]),
			encodeHashToBase64(bob.player.agentPubKey),
		);

		await bob.store.client.retractAsCloneProviderForRequest(cloneRequestHash);

		await dhtSync([alice.player, bob.player], alice.player.cells[0].cell_id[0]);

		providers =
			await alice.store.client.getCloneProvidersForRequest(cloneRequestHash);

		assert.equal(providers.length, 0);
	});
});
