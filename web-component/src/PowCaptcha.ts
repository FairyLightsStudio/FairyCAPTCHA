import { html, css, LitElement } from 'lit';
import { property, state } from 'lit/decorators.js';
import { createClient } from '@connectrpc/connect';
import { createConnectTransport } from '@connectrpc/connect-web';
import { PoWCAPTCHAFrontendService } from '@buf/fairy-lights-studio_pow-captcha.bufbuild_es/pow_captcha/v1/pow_captcha_frontend_pb';
import type { Challenge } from '@buf/fairy-lights-studio_pow-captcha.bufbuild_es/pow_captcha/v1/pow_captcha_frontend_pb';
import { type Timestamp ,timestampDate,timestampMs} from '@bufbuild/protobuf/wkt';

type Status = 'initializing' | 'ready' | 'solving' | 'error';

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
  private status: Status = 'initializing';

  @state()
  private statusText = '🛡️ 此操作由 PoWCAPTCHA 保护，以防滥用';

  private tokenPool: { token: string; expiresAt: number }[] = [];

  private rpcClient;

  constructor() {
    super();
    this.rpcClient = createClient(
      PoWCAPTCHAFrontendService,
      createConnectTransport({
        baseUrl: this.endpoint,
      }),
    );
  }
  async connectedCallback() {
    super.connectedCallback();

    this._solveAndCacheChallenge().catch(err => {
      console.error('Failed to solve initial challenge', err);
      this.status = 'error';
      this.statusText = '❌ 验证服务初始化失败';
    });
  }

  /**
   * Gets a valid token from the pool.
   * If no valid token is available, it will solve a new challenge and return the new token.
   * @returns A promise that resolves to a valid token.
   */
  public async getToken(): Promise<string> {
    this._cleanExpiredTokens();

    if (this.tokenPool.length > 0) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      return this.tokenPool.pop()!.token;
    }

    this.status = 'solving';
    this.statusText = '⏳ 正在使用设备算力通过验证';
    try {
      const token = await this._solveAndCacheChallenge();
      this.status = 'ready';
      this.statusText = '✅ 已通过验证';
      // Hide the success message after a short delay
      setTimeout(() => {
        this.statusText = '🛡️ 此操作由 PoWCAPTCHA 保护，以防滥用';
      }, 2000);
      return token;
    } catch (err) {
      this.status = 'error';
      this.statusText = '❌ 获取验证时出现错误';
      throw err;
    }
  }

  private async _solveAndCacheChallenge(): Promise<string> {
    const { examSession, challenge } = await this.rpcClient.getChallenge({
      powserviceAccessKeyId: this.accessKeyId,
    });

    if (!examSession || !challenge) {
      throw new Error('Invalid challenge response');
    }

    const nonce = await this._findNonce(challenge);

    const { token } = await this.rpcClient.submitSolution({
      examSession,
      solution: { nonce },
    });

    const expiresAt = challenge.expiresOn
      ? timestampMs(challenge.expiresOn)
      : Date.now() + 5 * 60 * 1000; // Default to 5 minutes

    this.tokenPool.push({ token, expiresAt });
    this.status = 'ready';

    // Pre-fetch another token if the pool is getting low
    if (this.tokenPool.length <= 1) {
      this._solveAndCacheChallenge().catch(err => {
        console.error('Failed to pre-fetch challenge', err);
      });
    }

    return token;
  }

  private _cleanExpiredTokens() {
    const now = Date.now();
    // Consider tokens expiring in the next 10 seconds as invalid
    const margin = 10 * 1000;
    this.tokenPool = this.tokenPool.filter(
      item => item.expiresAt > now + margin,
    );
  }

  private async _findNonce(challenge: Challenge): Promise<string> {
    const { baseData, difficulty, timestamp } = challenge;
    if (!timestamp) {
      throw new Error('Challenge timestamp is missing');
    }

    const targetPrefix = '0'.repeat(difficulty);
    const encoder = new TextEncoder();
    const challengeTimestamp = timestampDate(timestamp).toISOString();

    let nonce = 0;
    // eslint-disable-next-line no-constant-condition
    while (true) {
      const nonceHex = nonce.toString(16);
      const dataToHash = `${baseData}:${challengeTimestamp}:${nonceHex}`;
      const data = encoder.encode(dataToHash);
      // eslint-disable-next-line no-await-in-loop
      const hashBuffer = await crypto.subtle.digest('SHA-256', data);
      const hashArray = Array.from(new Uint8Array(hashBuffer));
      const hashHex = hashArray
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');

      if (hashHex.startsWith(targetPrefix)) {
        return nonceHex;
      }
      nonce += 1;
    }
  }

  render() {
    return html`<span>${this.statusText}</span>`;
  }
}
