import { Plug } from "./deps.ts";

const VERSION = "0.2.0";
const POLICY = Deno.env.get("PLUGIN_URL") === undefined
  ? Plug.CachePolicy.STORE
  : Plug.CachePolicy.NONE;
const PLUGIN_URL = Deno.env.get("PLUGIN_URL") ??
  `https://github.com/denosaurs/pane/releases/download/${VERSION}/`;

await Plug.prepare({
  name: "pane",
  url: PLUGIN_URL,
  policy: POLICY,
});
