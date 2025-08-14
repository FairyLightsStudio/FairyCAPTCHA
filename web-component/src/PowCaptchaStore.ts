import {
  BehaviorSubject,
  EMPTY,
  Observable,
  catchError,
  defer,
  filter,
  firstValueFrom,
  map,
  shareReplay,
  startWith,
  switchMap,
  tap,
} from 'rxjs';
import { createClient } from '@connectrpc/connect';
import { createConnectTransport } from '@connectrpc/connect-web';
import { PoWCAPTCHAFrontendService } from '@buf/fairy-lights-studio_pow-captcha.bufbuild_es/pow_captcha/v1/pow_captcha_frontend_pb';
import type { Challenge } from '@buf/fairy-lights-studio_pow-captcha.bufbuild_es/pow_captcha/v1/pow_captcha_frontend_pb';
import { timestampDate, timestampMs } from '@bufbuild/protobuf/wkt';

type Status = 'initializing' | 'ready' | 'solving' | 'error';
export interface PowCaptchaStatus {
  status: Status;
  statusText: string;
}

export class PowCaptchaStore {
  private readonly rpcClient;
  private readonly tokenPool$ = new BehaviorSubject<{ token: string; expiresAt: number }[]>([]);
  private readonly solveChallenge$ = new Observable<string>(subscriber => {
    this.status.next({ status: 'solving', statusText: '⏳ 正在使用设备算力通过验证' });
    this._solveChallengeInternal().then(token => {
      this.status.next({ status: 'ready', statusText: '✅ 已通过验证' });
      setTimeout(() => {
        this.status.next({ status: 'ready', statusText: '🛡️ 此操作由 PoWCAPTCHA 保护，以防滥用' });
      }, 2000);
      subscriber.next(token);
      subscriber.complete();
    }).catch(err => {
      this.status.next({ status: 'error', statusText: '❌ 获取验证时出现错误' });
      subscriber.error(err);
    });
  }).pipe(
    shareReplay({ bufferSize: 1, refCount: false })
  );

  public readonly status = new BehaviorSubject<PowCaptchaStatus>({
    status: 'initializing',
    statusText: '🛡️ 此操作由 PoWCAPTCHA 保护，以防滥用',
  });

  constructor(endpoint: string, private readonly accessKeyId: string) {
    this.rpcClient = createClient(
      PoWCAPTCHAFrontendService,
      createConnectTransport({ baseUrl: endpoint })
    );
    // Initial solve
    this.getToken().catch(err => console.error('Initial token fetch failed', err));
  }

  public async getToken(): Promise<string> {
    this._cleanExpiredTokens();
    const currentPool = this.tokenPool$.getValue();
    if (currentPool.length > 0) {
      const [tokenData, ...rest] = currentPool;
      this.tokenPool$.next(rest);
      return tokenData.token;
    }
    return firstValueFrom(this.solveChallenge$);
  }

  private async _solveChallengeInternal(): Promise<string> {
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
      : Date.now() + 5 * 60 * 1000;

    const currentPool = this.tokenPool$.getValue();
    this.tokenPool$.next([...currentPool, { token, expiresAt }]);
    
    // Pre-fetch
    if (this.tokenPool$.getValue().length <= 1) {
        this._solveChallengeInternal().catch(err => console.error('Prefetch failed', err));
    }

    return token;
  }

  private _cleanExpiredTokens() {
    const now = Date.now();
    const margin = 10 * 1000;
    const currentPool = this.tokenPool$.getValue();
    const newPool = currentPool.filter(item => item.expiresAt > now + margin);
    this.tokenPool$.next(newPool);
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
}
