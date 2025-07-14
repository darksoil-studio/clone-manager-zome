import { ZomeClient } from '@darksoil-studio/holochain-utils';
import {
	AgentPubKey,
	AppClient,
	EntryHash,
	EntryHashB64,
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

	createCloneRequest(cloneRequest: CloneRequest): Promise<EntryHash> {
		return this.callZome('create_clone_request', cloneRequest);
	}

	getAllCloneRequests(): Promise<Record<EntryHashB64, CloneRequest>> {
		return this.callZome('get_all_clone_requests', undefined);
	}

	announceAsCloneProviderForRequest(
		cloneRequestHash: EntryHash,
	): Promise<EntryHash> {
		return this.callZome(
			'announce_as_clone_provider_for_request',
			cloneRequestHash,
		);
	}

	retractAsCloneProviderForRequest(cloneRequestHash: EntryHash): Promise<void> {
		return this.callZome(
			'retract_as_clone_provider_for_request',
			cloneRequestHash,
		);
	}

	getCloneProvidersForRequest(
		cloneRequestHash: EntryHash,
	): Promise<Array<AgentPubKey>> {
		return this.callZome('get_clone_providers_for_request', cloneRequestHash);
	}
}
