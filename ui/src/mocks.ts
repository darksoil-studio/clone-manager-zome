import {
	AgentPubKeyMap,
	HashType,
	HoloHashMap,
	ZomeMock,
	decodeEntry,
	fakeCreateAction,
	fakeDeleteEntry,
	fakeEntry,
	fakeRecord,
	fakeUpdateEntry,
	hash,
	pickBy,
} from '@darksoil-studio/holochain-utils';
import {
	ActionHash,
	AgentPubKey,
	AppClient,
	Delete,
	EntryHash,
	Link,
	NewEntryAction,
	Record,
	SignedActionHashed,
	decodeHashFromBase64,
	fakeActionHash,
	fakeAgentPubKey,
	fakeDnaHash,
	fakeEntryHash,
} from '@holochain/client';

import { CloneManagerClient } from './clone-manager-client.js';

export class CloneManagerZomeMock extends ZomeMock implements AppClient {
	constructor(myPubKey?: AgentPubKey) {
		super(
			'clone_manager_test',
			'clone_manager',
			'clone_manager_test_app',
			myPubKey,
		);
	}
}
