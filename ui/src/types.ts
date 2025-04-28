import { 
  Record, 
  ActionHash, 
  DnaHash,
  SignedActionHashed,
  EntryHash, 
  AgentPubKey,
  Create,
  Update,
  Delete,
  CreateLink,
  DeleteLink
} from '@holochain/client';
import { ActionCommittedSignal } from '@darksoil-studio/holochain-utils';

export type CloneManagerSignal = ActionCommittedSignal<EntryTypes, LinkTypes>;

export type EntryTypes = never;

export type LinkTypes = string;
