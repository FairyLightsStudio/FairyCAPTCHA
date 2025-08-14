// A simple way to convert protobuf timestamp to ISO string without the full library
function protoTimestampToISO(ts: { seconds: bigint; nanos: number }): string {
  const ms = Number(ts.seconds * 1000n + BigInt(ts.nanos / 1_000_000));
  return new Date(ms).toISOString();
}

async function findNonce(
  challenge: {
    baseData: string;
    difficulty: number;
    timestamp: { seconds: bigint; nanos: number };
  },
  workerId: number,
  workerCount: number
): Promise<string> {
  const { baseData, difficulty, timestamp } = challenge;
  if (!timestamp) {
    throw new Error('Challenge timestamp is missing');
  }

  const targetPrefix = '0'.repeat(difficulty);
  const encoder = new TextEncoder();
  const challengeTimestamp = protoTimestampToISO(timestamp);

  let nonce = workerId;
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
    // Jump to the next nonce for this worker
    nonce += workerCount;
  }
}

self.onmessage = async (
  e: MessageEvent<{
    challenge: {
      baseData: string;
      difficulty: number;
      timestamp: { seconds: bigint; nanos: number };
    };
    workerId: number;
    workerCount: number;
  }>
) => {
  const { challenge, workerId, workerCount } = e.data;
  try {
    const nonce = await findNonce(challenge, workerId, workerCount);
    self.postMessage({ nonce });
  } catch (error) {
    self.postMessage({ error: (error as Error).message });
  }
};
