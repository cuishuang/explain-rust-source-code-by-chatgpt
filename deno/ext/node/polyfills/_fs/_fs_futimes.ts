// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

// TODO(petamoriken): enable prefer-primordials for node polyfills
// deno-lint-ignore-file prefer-primordials

import type { CallbackWithError } from "ext:deno_node/_fs/_fs_common.ts";
import { FsFile } from "ext:deno_fs/30_fs.js";

function getValidTime(
  time: number | string | Date,
  name: string,
): number | Date {
  if (typeof time === "string") {
    time = Number(time);
  }

  if (
    typeof time === "number" &&
    (Number.isNaN(time) || !Number.isFinite(time))
  ) {
    throw new Deno.errors.InvalidData(
      `invalid ${name}, must not be infinity or NaN`,
    );
  }

  return time;
}

export function futimes(
  fd: number,
  atime: number | string | Date,
  mtime: number | string | Date,
  callback: CallbackWithError,
) {
  if (!callback) {
    throw new Deno.errors.InvalidData("No callback function supplied");
  }

  atime = getValidTime(atime, "atime");
  mtime = getValidTime(mtime, "mtime");

  // TODO(@littledivy): Treat `fd` as real file descriptor.
  new FsFile(fd, Symbol.for("Deno.internal.FsFile")).utime(atime, mtime).then(
    () => callback(null),
    callback,
  );
}

export function futimesSync(
  fd: number,
  atime: number | string | Date,
  mtime: number | string | Date,
) {
  atime = getValidTime(atime, "atime");
  mtime = getValidTime(mtime, "mtime");

  // TODO(@littledivy): Treat `fd` as real file descriptor.
  new FsFile(fd, Symbol.for("Deno.internal.FsFile")).utimeSync(atime, mtime);
}
