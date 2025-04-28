import {
	AsyncComputed,
	allRevisionsOfEntrySignal,
	collectionSignal,
	deletedLinksSignal,
	deletesForEntrySignal,
	immutableEntrySignal,
	latestVersionOfEntrySignal,
	liveLinksSignal,
	pipe,
} from '@darksoil-studio/holochain-signals';
import {
	EntryRecord,
	HashType,
	MemoHoloHashMap,
	retype,
	slice,
} from '@darksoil-studio/holochain-utils';
import {
	ActionHash,
	AgentPubKey,
	EntryHash,
	NewEntryAction,
	Record,
} from '@holochain/client';

import { CloneManagerClient } from './clone-manager-client.js';

export class CloneManagerStore {
	constructor(public client: CloneManagerClient) {}
}
