import { EntryRecord, ZomeClient } from '@darksoil-studio/holochain-utils';
import {
	ActionHash,
	AgentPubKey,
	AppClient,
	CreateLink,
	Delete,
	DeleteLink,
	EntryHash,
	EntryHashB64,
	Link,
	SignedActionHashed,
} from '@holochain/client';

import { CloneManagerSignal, CloneRequest } from './types.js';

export class CloneManagerClient extends ZomeClient<CloneManagerSignal> {
	constructor(
		public client: AppClient,
		public roleName: string,
		public zomeName = 'clone_manager',
	) {
		super(client, roleName, zomeName);
	}

	createCloneRequest(cloneRequest: CloneRequest) {
		return this.callZome('create_clone_request', cloneRequest);
	}

	getAllCloneRequests(): Promise<Record<EntryHashB64, CloneRequest>> {
		return this.callZome('get_all_clone_requests', undefined);
	}
}
