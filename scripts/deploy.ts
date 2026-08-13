/** Deploy a WASM contract:  INAZ_KEY=inazkey1... bun run scripts/deploy.ts contract.wasm */
import { InazumaClient, keypairFromSecret } from "@inazuma/sdk";

const [file] = process.argv.slice(2);
const secret = process.env.INAZ_KEY;
if (!secret || !file) throw new Error("usage: INAZ_KEY=... bun run scripts/deploy.ts <contract.wasm>");

const code = Buffer.from(await Bun.file(file).arrayBuffer()).toString("hex");
const inaz = new InazumaClient({ url: process.env.INAZ_RPC_URL });
const me = keypairFromSecret(secret);
const account = (await inaz.getAccount(me.address)) as { nonce: number };

const hash = await inaz.call<string>("inaz_sendTransaction", [
  { kind: "deploy", from_pubkey: me.pubkeyHex, code, nonce: account.nonce, chain_id: 7777 },
]);
console.log("deploy tx", hash);
console.log("receipt", await inaz.getReceipt(hash));
