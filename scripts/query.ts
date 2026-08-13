/** Read a contract for free:  bun run scripts/query.ts <address> get */
import { InazumaClient } from "@inazuma/sdk";

const [address, method = "get", ...args] = process.argv.slice(2);
if (!address) throw new Error("usage: bun run scripts/query.ts <address> [method] [args...]");

const inaz = new InazumaClient({ url: process.env.INAZ_RPC_URL });
console.log(await inaz.query(address, method, args));
