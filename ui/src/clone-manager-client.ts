import { EntryRecord, ZomeClient } from '@darksoil-studio/holochain-utils';
import {
	ActionHash,
	AgentPubKey,
	AppClient,
	CreateLink,
	Delete,
	DeleteLink,
	EntryHash,
	Link,
	Record,
	SignedActionHashed,
} from '@holochain/client';

import { CloneManagerSignal } from './types.js';

export class CloneManagerClient extends ZomeClient<CloneManagerSignal> {
	constructor(
		public client: AppClient,
		public roleName: string,
		public zomeName = 'clone_manager',
	) {
		super(client, roleName, zomeName);
	}
}
