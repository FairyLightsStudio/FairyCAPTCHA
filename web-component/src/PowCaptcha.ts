import { html, css, LitElement, type PropertyValues } from 'lit';
import { property, state } from 'lit/decorators.js';
import { Subscription } from 'rxjs';
import { PowCaptchaStore, type PowCaptchaStatus } from './PowCaptchaStore.js';

export class PowCaptcha extends LitElement {
  static styles = css`
    :host {
      display: inline-block;
      font-family: sans-serif;
      font-size: 14px;
      color: #6b7280;
    }
  `;

  /**
   * Your PoWService Access Key ID.
   */
  @property({ type: String, attribute: 'access-key-id' })
  accessKeyId = '';

  /**
   * The backend endpoint for the PoWCAPTCHA service.
   */
  @property({ type: String })
  endpoint = 'https://pow-captcha.fly.dev';

  @state()
  private statusText = '🛡️ 此操作由 PoWCAPTCHA 保护，以防滥用';

  private store?: PowCaptchaStore;
  private statusSubscription?: Subscription;

  connectedCallback() {
    super.connectedCallback();
    this._initializeStore();
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    this.statusSubscription?.unsubscribe();
  }

  protected updated(changedProperties: PropertyValues<this>): void {
    if (changedProperties.has('accessKeyId') || changedProperties.has('endpoint')) {
      this._initializeStore();
    }
  }

  /**
   * Gets a valid token.
   * If no valid token is available, it will solve a new challenge and return the new token.
   * @returns A promise that resolves to a valid token.
   */
  public async getToken(): Promise<string> {
    if (!this.store) {
      throw new Error('PoWCaptcha component is not initialized.');
    }
    return this.store.getToken();
  }

  private _initializeStore() {
    this.statusSubscription?.unsubscribe();
    
    if (this.accessKeyId) {
      this.store = new PowCaptchaStore(this.endpoint, this.accessKeyId);
      this.statusSubscription = this.store.status.subscribe(
        (status: PowCaptchaStatus) => {
          this.statusText = status.statusText;
        }
      );
    } else {
        this.statusText = '❌ 未提供 Access Key ID';
    }
  }

  render() {
    return html`<span>${this.statusText}</span>`;
  }
}
