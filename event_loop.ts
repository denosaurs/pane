import { sync, unwrap } from "./plugin.ts";
import { Event } from "./types.ts";

export class EventLoop {
  public static Step(): Event[] {
    return unwrap(sync("event_loop_step"));
  }
}
