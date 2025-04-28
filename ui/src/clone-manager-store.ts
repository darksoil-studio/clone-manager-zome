import { 
  collectionSignal, 
  liveLinksSignal, 
  deletedLinksSignal, 
  allRevisionsOfEntrySignal,
  latestVersionOfEntrySignal, 
  immutableEntrySignal, 
  deletesForEntrySignal, 
  AsyncComputed,
  pipe,
} from "@darksoil-studio/holochain-signals";
import { slice, HashType, retype, EntryRecord, MemoHoloHashMap } from "@darksoil-studio/holochain-utils";
import { NewEntryAction, Record, ActionHash, EntryHash, AgentPubKey } from '@holochain/client';

import { CloneManagerClient } from './clone-manager-client.js';

export class CloneManagerStore {

  constructor(public client: CloneManagerClient) {}
  
}
