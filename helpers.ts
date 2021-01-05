/** JSON.parse with BigInt support */
export function deserialize(text: string): unknown {
  return JSON.parse(
    text.replace(/([^\"]+\"\:\s*)(\d{16,})/g, '$1"$2n"'),
    (_, v) => {
      if (typeof v === "string" && /^\d{16,}n$/.test(v)) {
        v = BigInt(v.slice(0, -1));
      }

      return v;
    },
  );
}
/** JSON.stringify with BigInt support */
export function serialize(value: unknown, space?: number): string {
  return JSON.stringify(value, (_, v) => {
    if (typeof v === "bigint") {
      v = v.toString() + "n";
    }
    return v;
  }, space).replace(/(?:\")(\d{16,})(?:n\")/g, "$1");
}
