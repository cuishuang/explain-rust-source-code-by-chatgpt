// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

import { primordials } from "ext:core/mod.js";
const {
  Error,
} = primordials;
import { op_is_terminal } from "ext:core/ops";

import { ERR_INVALID_FD } from "ext:deno_node/internal/errors.ts";
import { LibuvStreamWrap } from "ext:deno_node/internal_binding/stream_wrap.ts";
import { providerType } from "ext:deno_node/internal_binding/async_wrap.ts";
import { Socket } from "node:net";
import { setReadStream } from "ext:deno_node/_process/streams.mjs";

// Returns true when the given numeric fd is associated with a TTY and false otherwise.
function isatty(fd) {
  if (typeof fd !== "number") {
    return false;
  }
  try {
    /**
     * TODO: Treat `fd` as real file descriptors. Currently, `rid` 0, 1, 2
     * correspond to `fd` 0, 1, 2 (stdin, stdout, stderr). This may change in
     * the future.
     */
    return op_is_terminal(fd);
  } catch (_) {
    return false;
  }
}

class TTY extends LibuvStreamWrap {
  constructor(handle) {
    super(providerType.TTYWRAP, handle);
  }
}

export class ReadStream extends Socket {
  constructor(fd, options) {
    if (fd >> 0 !== fd || fd < 0) {
      throw new ERR_INVALID_FD(fd);
    }

    // We only support `stdin`.
    if (fd != 0) throw new Error("Only fd 0 is supported.");

    const tty = new TTY(Deno.stdin);
    super({
      readableHighWaterMark: 0,
      handle: tty,
      manualStart: true,
      ...options,
    });

    this.isRaw = false;
    this.isTTY = true;
  }

  setRawMode(flag) {
    flag = !!flag;
    this._handle.setRaw(flag);

    this.isRaw = flag;
    return this;
  }
}

setReadStream(ReadStream);

export class WriteStream extends Socket {
  constructor(fd) {
    if (fd >> 0 !== fd || fd < 0) {
      throw new ERR_INVALID_FD(fd);
    }

    // We only support `stdin`, `stdout` and `stderr`.
    if (fd > 2) throw new Error("Only fd 0, 1 and 2 are supported.");

    const tty = new TTY(
      fd === 0 ? Deno.stdin : fd === 1 ? Deno.stdout : Deno.stderr,
    );

    super({
      readableHighWaterMark: 0,
      handle: tty,
      manualStart: true,
    });

    const { columns, rows } = Deno.consoleSize();
    this.columns = columns;
    this.rows = rows;
    this.isTTY = true;
  }
}

export { isatty };
export default { isatty, WriteStream, ReadStream };
