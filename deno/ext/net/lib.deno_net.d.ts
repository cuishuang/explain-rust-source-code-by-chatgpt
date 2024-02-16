// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

/// <reference no-default-lib="true" />
/// <reference lib="esnext" />
/// <reference lib="esnext.disposable" />

declare namespace Deno {
  /** @category Network */
  export interface NetAddr {
    transport: "tcp" | "udp";
    hostname: string;
    port: number;
  }

  /** @category Network */
  export interface UnixAddr {
    transport: "unix" | "unixpacket";
    path: string;
  }

  /** @category Network */
  export type Addr = NetAddr | UnixAddr;

  /** A generic network listener for stream-oriented protocols.
   *
   * @category Network
   */
  export interface Listener<T extends Conn = Conn>
    extends AsyncIterable<T>, Disposable {
    /** Waits for and resolves to the next connection to the `Listener`. */
    accept(): Promise<T>;
    /** Close closes the listener. Any pending accept promises will be rejected
     * with errors. */
    close(): void;
    /** Return the address of the `Listener`. */
    readonly addr: Addr;

    /**
     * Return the rid of the `Listener`.
     *
     * @deprecated Use {@linkcode Deno.Listener} instance methods instead.
     * {@linkcode Deno.Listener.rid} will be removed in Deno 2.0.
     */
    readonly rid: number;

    [Symbol.asyncIterator](): AsyncIterableIterator<T>;

    /**
     * Make the listener block the event loop from finishing.
     *
     * Note: the listener blocks the event loop from finishing by default.
     * This method is only meaningful after `.unref()` is called.
     */
    ref(): void;

    /** Make the listener not block the event loop from finishing. */
    unref(): void;
  }

  /** Specialized listener that accepts TLS connections.
   *
   * @category Network
   */
  export type TlsListener = Listener<TlsConn>;

  /** @category Network */
  export interface Conn extends Reader, Writer, Closer, Disposable {
    /** The local address of the connection. */
    readonly localAddr: Addr;
    /** The remote address of the connection. */
    readonly remoteAddr: Addr;
    /**
     * The resource ID of the connection.
     *
     * @deprecated Use {@linkcode Deno.Conn} instance methods instead.
     * {@linkcode Deno.Conn.rid} will be removed in Deno 2.0.
     */
    readonly rid: number;
    /** Shuts down (`shutdown(2)`) the write side of the connection. Most
     * callers should just use `close()`. */
    closeWrite(): Promise<void>;

    /** Make the connection block the event loop from finishing.
     *
     * Note: the connection blocks the event loop from finishing by default.
     * This method is only meaningful after `.unref()` is called.
     */
    ref(): void;
    /** Make the connection not block the event loop from finishing. */
    unref(): void;

    readonly readable: ReadableStream<Uint8Array>;
    readonly writable: WritableStream<Uint8Array>;
  }

  /** @category Network */
  export interface TlsHandshakeInfo {
    /**
     * Contains the ALPN protocol selected during negotiation with the server.
     * If no ALPN protocol selected, returns `null`.
     */
    alpnProtocol: string | null;
  }

  /** @category Network */
  export interface TlsConn extends Conn {
    /** Runs the client or server handshake protocol to completion if that has
     * not happened yet. Calling this method is optional; the TLS handshake
     * will be completed automatically as soon as data is sent or received. */
    handshake(): Promise<TlsHandshakeInfo>;
    /**
     * The resource ID of the connection.
     *
     * @deprecated Use {@linkcode Deno.TlsConn} instance methods instead.
     * {@linkcode Deno.TlsConn.rid} will be removed in Deno 2.0.
     */
    readonly rid: number;
  }

  /** @category Network */
  export interface ListenOptions {
    /** The port to listen on. */
    port: number;
    /** A literal IP address or host name that can be resolved to an IP address.
     *
     * __Note about `0.0.0.0`__ While listening `0.0.0.0` works on all platforms,
     * the browsers on Windows don't work with the address `0.0.0.0`.
     * You should show the message like `server running on localhost:8080` instead of
     * `server running on 0.0.0.0:8080` if your program supports Windows.
     *
     * @default {"0.0.0.0"} */
    hostname?: string;
  }

  /** @category Network */
  // deno-lint-ignore no-empty-interface
  export interface TcpListenOptions extends ListenOptions {
  }

  /** Listen announces on the local transport address.
   *
   * ```ts
   * const listener1 = Deno.listen({ port: 80 })
   * const listener2 = Deno.listen({ hostname: "192.0.2.1", port: 80 })
   * const listener3 = Deno.listen({ hostname: "[2001:db8::1]", port: 80 });
   * const listener4 = Deno.listen({ hostname: "golang.org", port: 80, transport: "tcp" });
   * ```
   *
   * Requires `allow-net` permission.
   *
   * @tags allow-net
   * @category Network
   */
  export function listen(
    options: TcpListenOptions & { transport?: "tcp" },
  ): Listener;

  /** Options which can be set when opening a Unix listener via
   * {@linkcode Deno.listen} or {@linkcode Deno.listenDatagram}.
   *
   * @category Network
   */
  export interface UnixListenOptions {
    /** A path to the Unix Socket. */
    path: string;
  }

  /** Listen announces on the local transport address.
   *
   * ```ts
   * const listener = Deno.listen({ path: "/foo/bar.sock", transport: "unix" })
   * ```
   *
   * Requires `allow-read` and `allow-write` permission.
   *
   * @tags allow-read, allow-write
   * @category Network
   */
  // deno-lint-ignore adjacent-overload-signatures
  export function listen(
    options: UnixListenOptions & { transport: "unix" },
  ): Listener;

  /** @category Network */
  export interface ListenTlsOptions extends TcpListenOptions {
    /** Server private key in PEM format */
    key?: string;
    /** Cert chain in PEM format */
    cert?: string;
    /** Path to a file containing a PEM formatted CA certificate. Requires
     * `--allow-read`.
     *
     * @tags allow-read
     * @deprecated Pass the certificate file contents directly to the
     * {@linkcode Deno.ListenTlsOptions.cert} option instead. This option will
     * be removed in Deno 2.0.
     */
    certFile?: string;
    /** Server private key file. Requires `--allow-read`.
     *
     * @tags allow-read
     * @deprecated Pass the key file contents directly to the
     * {@linkcode Deno.ListenTlsOptions.key} option instead. This option will
     * be removed in Deno 2.0.
     */
    keyFile?: string;

    transport?: "tcp";

    /** Application-Layer Protocol Negotiation (ALPN) protocols to announce to
     * the client. If not specified, no ALPN extension will be included in the
     * TLS handshake.
     */
    alpnProtocols?: string[];
  }

  /** Listen announces on the local transport address over TLS (transport layer
   * security).
   *
   * ```ts
   * using listener = Deno.listenTls({
   *   port: 443,
   *   cert: Deno.readTextFileSync("./server.crt"),
   *   key: Deno.readTextFileSync("./server.key"),
   * });
   * ```
   *
   * Requires `allow-net` permission.
   *
   * @tags allow-net
   * @category Network
   */
  export function listenTls(options: ListenTlsOptions): TlsListener;

  /** @category Network */
  export interface ConnectOptions {
    /** The port to connect to. */
    port: number;
    /** A literal IP address or host name that can be resolved to an IP address.
     * If not specified,
     *
     * @default {"127.0.0.1"} */
    hostname?: string;
    transport?: "tcp";
  }

  /**
   * Connects to the hostname (default is "127.0.0.1") and port on the named
   * transport (default is "tcp"), and resolves to the connection (`Conn`).
   *
   * ```ts
   * const conn1 = await Deno.connect({ port: 80 });
   * const conn2 = await Deno.connect({ hostname: "192.0.2.1", port: 80 });
   * const conn3 = await Deno.connect({ hostname: "[2001:db8::1]", port: 80 });
   * const conn4 = await Deno.connect({ hostname: "golang.org", port: 80, transport: "tcp" });
   * ```
   *
   * Requires `allow-net` permission for "tcp".
   *
   * @tags allow-net
   * @category Network
   */
  export function connect(options: ConnectOptions): Promise<TcpConn>;

  /** @category Network */
  export interface TcpConn extends Conn {
    /**
     * Enable/disable the use of Nagle's algorithm.
     *
     * @param [noDelay=true]
     */
    setNoDelay(noDelay?: boolean): void;
    /** Enable/disable keep-alive functionality. */
    setKeepAlive(keepAlive?: boolean): void;
    /**
     * The resource ID of the connection.
     *
     * @deprecated Use {@linkcode Deno.Conn} instance methods instead.
     * {@linkcode Deno.Conn.rid} will be removed in Deno 2.0.
     */
    readonly rid: number;
  }

  /** @category Network */
  export interface UnixConnectOptions {
    transport: "unix";
    path: string;
  }

  /** @category Network */
  export interface UnixConn extends Conn {
    /**
     * The resource ID of the connection.
     *
     * @deprecated Use {@linkcode Deno.UnixConn} instance methods instead.
     * {@linkcode Deno.UnixConn.rid} will be removed in Deno 2.0.
     */
    readonly rid: number;
  }

  /** Connects to the hostname (default is "127.0.0.1") and port on the named
   * transport (default is "tcp"), and resolves to the connection (`Conn`).
   *
   * ```ts
   * const conn1 = await Deno.connect({ port: 80 });
   * const conn2 = await Deno.connect({ hostname: "192.0.2.1", port: 80 });
   * const conn3 = await Deno.connect({ hostname: "[2001:db8::1]", port: 80 });
   * const conn4 = await Deno.connect({ hostname: "golang.org", port: 80, transport: "tcp" });
   * const conn5 = await Deno.connect({ path: "/foo/bar.sock", transport: "unix" });
   * ```
   *
   * Requires `allow-net` permission for "tcp" and `allow-read` for "unix".
   *
   * @tags allow-net, allow-read
   * @category Network
   */
  // deno-lint-ignore adjacent-overload-signatures
  export function connect(options: UnixConnectOptions): Promise<UnixConn>;

  /** @category Network */
  export interface ConnectTlsOptions {
    /** The port to connect to. */
    port: number;
    /** A literal IP address or host name that can be resolved to an IP address.
     *
     * @default {"127.0.0.1"} */
    hostname?: string;
    /**
     * Server certificate file.
     *
     * @deprecated Pass the cert file contents directly to the
     * {@linkcode Deno.ConnectTlsOptions.caCerts} option instead. This option
     * will be removed in Deno 2.0.
     */
    certFile?: string;
    /** A list of root certificates that will be used in addition to the
     * default root certificates to verify the peer's certificate.
     *
     * Must be in PEM format. */
    caCerts?: string[];
    /** Application-Layer Protocol Negotiation (ALPN) protocols supported by
     * the client. If not specified, no ALPN extension will be included in the
     * TLS handshake.
     */
    alpnProtocols?: string[];
    /** PEM formatted client certificate chain. */
    certChain?: string;
    /** PEM formatted (RSA or PKCS8) private key of client certificate.  */
    privateKey?: string;
  }

  /** Establishes a secure connection over TLS (transport layer security) using
   * an optional cert file, hostname (default is "127.0.0.1") and port.  The
   * cert file is optional and if not included Mozilla's root certificates will
   * be used (see also https://github.com/ctz/webpki-roots for specifics)
   *
   * ```ts
   * const caCert = await Deno.readTextFile("./certs/my_custom_root_CA.pem");
   * const conn1 = await Deno.connectTls({ port: 80 });
   * const conn2 = await Deno.connectTls({ caCerts: [caCert], hostname: "192.0.2.1", port: 80 });
   * const conn3 = await Deno.connectTls({ hostname: "[2001:db8::1]", port: 80 });
   * const conn4 = await Deno.connectTls({ caCerts: [caCert], hostname: "golang.org", port: 80});
   * ```
   *
   * Requires `allow-net` permission.
   *
   * @tags allow-net
   * @category Network
   */
  export function connectTls(options: ConnectTlsOptions): Promise<TlsConn>;

  /** @category Network */
  export interface StartTlsOptions {
    /** A literal IP address or host name that can be resolved to an IP address.
     *
     * @default {"127.0.0.1"} */
    hostname?: string;
    /** A list of root certificates that will be used in addition to the
     * default root certificates to verify the peer's certificate.
     *
     * Must be in PEM format. */
    caCerts?: string[];
    /** Application-Layer Protocol Negotiation (ALPN) protocols to announce to
     * the client. If not specified, no ALPN extension will be included in the
     * TLS handshake.
     */
    alpnProtocols?: string[];
  }

  /** Start TLS handshake from an existing connection using an optional list of
   * CA certificates, and hostname (default is "127.0.0.1"). Specifying CA certs
   * is optional. By default the configured root certificates are used. Using
   * this function requires that the other end of the connection is prepared for
   * a TLS handshake.
   *
   * Note that this function *consumes* the TCP connection passed to it, thus the
   * original TCP connection will be unusable after calling this. Additionally,
   * you need to ensure that the TCP connection is not being used elsewhere when
   * calling this function in order for the TCP connection to be consumed properly.
   * For instance, if there is a `Promise` that is waiting for read operation on
   * the TCP connection to complete, it is considered that the TCP connection is
   * being used elsewhere. In such a case, this function will fail.
   *
   * ```ts
   * const conn = await Deno.connect({ port: 80, hostname: "127.0.0.1" });
   * const caCert = await Deno.readTextFile("./certs/my_custom_root_CA.pem");
   * // `conn` becomes unusable after calling `Deno.startTls`
   * const tlsConn = await Deno.startTls(conn, { caCerts: [caCert], hostname: "localhost" });
   * ```
   *
   * Requires `allow-net` permission.
   *
   * @tags allow-net
   * @category Network
   */
  export function startTls(
    conn: Conn,
    options?: StartTlsOptions,
  ): Promise<TlsConn>;

  /** Shutdown socket send operations.
   *
   * Matches behavior of POSIX shutdown(3).
   *
   * ```ts
   * const listener = Deno.listen({ port: 80 });
   * const conn = await listener.accept();
   * Deno.shutdown(conn.rid);
   * ```
   *
   * @deprecated Use {@linkcode Deno.Conn.closeWrite} instead.
   * {@linkcode Deno.shutdown} will be removed in Deno 2.0.
   *
   * @category Network
   */
  export function shutdown(rid: number): Promise<void>;
}
