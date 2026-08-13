/** Call a contract (write):  INAZ_KEY=inazkey1... bun run scripts/call.ts <address> "add:5" */
import { InazumaClient, keypairFromSecret } from "@inazuma/sdk";

const [address, args = ""] = process.argv.slice(2);
const secret = process.env.INAZ_KEY;
if (!secret || !address) throw new Error("usage: INAZ_KEY=... bun run scripts/call.ts <address> [args]");

const inaz = new InazumaClient({ url: process.env.INAZ_RPC_URL });
const me = keypairFromSecret(secret);
const account = (await inaz.getAccount(me.address)) as { nonce: number };

const hash = await inaz.call<string>("inaz_sendTransaction", [
  { kind: "invoke", from_pubkey: me.pubkeyHex, to: address, args, nonce: account.nonce, chain_id: 7777 },
]);
console.log("tx", hash);
console.log("receipt", await inaz.getReceipt(hash));
