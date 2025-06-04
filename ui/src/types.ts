import { ActionCommittedSignal } from '@darksoil-studio/holochain-utils';
import {
	ActionHash,
	AgentPubKey,
	Create,
	CreateLink,
	Delete,
	DeleteLink,
	DnaHash,
	DnaModifiers,
	EntryHash,
	Record,
	SignedActionHashed,
	Update,
} from '@holochain/client';

export type CloneManagerSignal = ActionCommittedSignal<EntryTypes, LinkTypes>;

export type EntryTypes = never;

export type LinkTypes = string;

export interface CloneRequest {
	dna_modifiers: DnaModifiers;
}
