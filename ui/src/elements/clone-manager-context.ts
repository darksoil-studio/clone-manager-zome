import { appClientContext } from '@darksoil-studio/holochain-elements';
import { AppClient } from '@holochain/client';
import { consume, provide } from '@lit/context';
import { LitElement, css, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';

import { CloneManagerClient } from '../clone-manager-client.js';
import { CloneManagerStore } from '../clone-manager-store.js';
import { cloneManagerStoreContext } from '../context.js';

/**
 * @element clone-manager-context
 */
@customElement('clone-manager-context')
export class CloneManagerContext extends LitElement {
	@consume({ context: appClientContext })
	private client!: AppClient;

	@provide({ context: cloneManagerStoreContext })
	@property({ type: Object })
	store!: CloneManagerStore;

	@property()
	role!: string;

	@property()
	zome = 'clone_manager';

	connectedCallback() {
		super.connectedCallback();
		if (this.store) return;
		if (!this.role) {
			throw new Error(
				`<clone-manager-context> must have a role="YOUR_DNA_ROLE" property, eg: <clone-manager-context role="role1">`,
			);
		}
		if (!this.client) {
			throw new Error(`<clone-manager-context> must either:
        a) be placed inside <app-client-context>
          or 
        b) receive an AppClient property (eg. <clone-manager-context .client=\${client}>) 
          or 
        c) receive a store property (eg. <clone-manager-context .store=\${store}>)
      `);
		}

		this.store = new CloneManagerStore(
			new CloneManagerClient(this.client, this.role, this.zome),
		);
	}

	render() {
		return html`<slot></slot>`;
	}

	static styles = css`
		:host {
			display: contents;
		}
	`;
}
