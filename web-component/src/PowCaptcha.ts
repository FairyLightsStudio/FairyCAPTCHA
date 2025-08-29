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
  endpoint = 'https://localhost:8080';

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

    const nonce = await this._findNonceWithWorkers(challenge);

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

  private _findNonceWithWorkers(challenge: Challenge): Promise<string> {
    return new Promise((resolve, reject) => {
      const workerCount = navigator.hardwareConcurrency / 2  || 4; // Use 4 as a fallback
      const workers: Worker[] = [];

      for (let i = 0; i < workerCount; i += 1) {
        // Using `new URL` is a modern way to ensure bundlers can handle workers.
        const worker = new Worker(new URL('./pow.worker.ts', import.meta.url), {
          type: 'module',
        });
        workers.push(worker);

        worker.onmessage = e => {
          if (e.data.nonce) {
            // Once one worker finds the solution, terminate all others and resolve.
            workers.forEach(w => w.terminate());
            resolve(e.data.nonce);
          } else if (e.data.error) {
            workers.forEach(w => w.terminate());
            reject(new Error(e.data.error));
          }
        };

        worker.onerror = e => {
          // Terminate all workers on any error and reject the promise.
          workers.forEach(w => w.terminate());
          reject(new Error(`Worker error: ${e.message}`));
        };

        // The challenge object from protobuf cannot be cloned directly into a worker.
        // We need to send a plain object representation.
        const plainChallenge = {
          baseData: challenge.baseData,
          difficulty: challenge.difficulty,
          timestamp: challenge.timestamp
            ? {
                seconds: challenge.timestamp.seconds,
                nanos: challenge.timestamp.nanos,
              }
            : undefined,
        };

        worker.postMessage({
          challenge: plainChallenge,
          workerId: i,
          workerCount,
        });
      }
    });
  }

  render() {
    return html`<span>${this.statusText}</span>`;
  }
}
