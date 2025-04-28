import { 
  SignedActionHashed,
  CreateLink,
  Link,
  DeleteLink,
  Delete,
  AppClient, 
  Record, 
  ActionHash, 
  EntryHash, 
  AgentPubKey,
} from '@holochain/client';
import { EntryRecord, ZomeClient } from '@darksoil-studio/holochain-utils';

import { CloneManagerSignal } from './types.js';

export class CloneManagerClient extends ZomeClient<CloneManagerSignal> {

  constructor(public client: AppClient, public roleName: string, public zomeName = 'clone_manager') {
    super(client, roleName, zomeName);
  }
}
