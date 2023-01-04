# bcrypt-wasi

This wraps the Rust crate [`bcrypt`](https://docs.rs/bcrypt/latest/bcrypt/) in a very simple WASI (or commandline) compatible program.

Compatible with [`workerd`](https://github.com/cloudflare/workerd) / CloudFlare Workers' [WASI support](https://blog.cloudflare.com/announcing-wasi-on-workers/). You need to run the worker in unbounded mode for it to not time out.

[`argon2-wasi`](https://github.com/lufsorg/argon2-wasi) provides an identical API for Argon2.

## Usage

Simple example:

```ts
import { WASI } from "@cloudflare/workers-wasi";
import bcrypt from './bcrypt-wasi.wasm';

export async function invoke(args: string[]) {
  const stdout = new TransformStream();
  const stderr = new TransformStream();
  const wasi = new WASI({
    args: ["bcrypt-wasi.wasm", ...args],
    stdout: stdout.writable,
    stderr: stderr.writable,
  });
  const instance = new WebAssembly.Instance(bcrypt, {
    wasi_snapshot_preview1: wasi.wasiImport,
  });
  await wasi.start(instance);
  const errors = await stderr.readable.getReader().read();
  const errorsValue = new TextDecoder().decode(errors.value);
  if (errorsValue) {
    console.error('[invoke] stderr: ', errorsValue);
    throw new Error(errorsValue);
  }
  const ret = await stdout.readable.getReader().read();
  const retValue = new TextDecoder().decode(ret.value);
  return retValue.trim();
}

export async function bcryptHash(password: string, rounds: number = 9): Promise<string> {
  return await invoke(["hash", password, rounds.toString()]);
}

export async function bcryptVerify(password: string, hash: string): Promise<boolean> {
  return await invoke(["verify", password, hash]) === "true";
}
```

Then just use `await bcryptHash('somepwd');` or `await bcryptVerify('somepwd', '$2b$09$.J2z..')`

## License

[MIT license](http://opensource.org/licenses/MIT)
