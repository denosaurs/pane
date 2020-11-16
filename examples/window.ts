import { load, SurfaceNew, SurfaceStep, unload } from "../plugin.ts";

await load();

let id = SurfaceNew();
setInterval(() => SurfaceStep(id), 1000 / 30);
