import { createContext } from '@lit/context';

import { CloneManagerStore } from './clone-manager-store.js';

export const cloneManagerStoreContext = createContext<CloneManagerStore>(
	'clone_manager/store',
);
