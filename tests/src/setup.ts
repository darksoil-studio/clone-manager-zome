import { EntryRecord } from '@darksoil-studio/holochain-utils';
import {
	ActionHash,
	AgentPubKey,
	AppBundleSource,
	AppWebsocket,
	EntryHash,
	NewEntryAction,
	Record,
	encodeHashToBase64,
	fakeActionHash,
	fakeAgentPubKey,
	fakeDnaHash,
	fakeEntryHash,
} from '@holochain/client';
import { Player, Scenario, dhtSync, pause } from '@holochain/tryorama';
import { encode } from '@msgpack/msgpack';
import { dirname } from 'path';
import { fileURLToPath } from 'url';

import { CloneManagerClient } from '../../ui/src/clone-manager-client.js';
import { CloneManagerStore } from '../../ui/src/clone-manager-store.js';

const testHappUrl =
	dirname(fileURLToPath(import.meta.url)) +
	'/../../workdir/clone_manager_test.happ';

export async function setup(scenario: Scenario, numPlayers = 2) {
	const players = await promiseAllSequential(
		Array.from(new Array(numPlayers)).map(() => () => addPlayer(scenario)),
	);

	// Shortcut peer discovery through gossip and register all agents in every
	// conductor of the scenario.
	await scenario.shareAllAgents();

	await dhtSync(
		players.map(p => p.player),
		players[0].player.cells[0].cell_id[0],
	);

	console.log('Setup completed!');

	return players;
}

async function addPlayer(scenario: Scenario) {
	const player = await scenario.addPlayerWithApp({
		appBundleSource: {
			type: 'path',
			value: testHappUrl,
		},
	});

	await player.conductor
		.adminWs()
		.authorizeSigningCredentials(player.cells[0].cell_id);
	const store = new CloneManagerStore(
		new CloneManagerClient(player.appWs as any, 'clone_manager_test'),
	);
	return {
		store,
		player,
		startUp: async () => {
			await player.conductor.startUp();
			const port = await player.conductor.attachAppInterface();
			const issued = await player.conductor
				.adminWs()
				.issueAppAuthenticationToken({
					installed_app_id: player.appId,
				});
			const appWs = await player.conductor.connectAppWs(issued.token, port);
			store.client.client = appWs;
		},
	};
}

async function promiseAllSequential<T>(
	promises: Array<() => Promise<T>>,
): Promise<Array<T>> {
	const results: Array<T> = [];
	for (const promise of promises) {
		results.push(await promise());
	}
	return results;
}

export async function waitUntil(
	condition: () => Promise<boolean>,
	timeout: number,
) {
	const start = Date.now();
	const isDone = await condition();
	if (isDone) return;
	if (timeout <= 0) throw new Error('timeout');
	await pause(1000);
	return waitUntil(condition, timeout - (Date.now() - start));
}
