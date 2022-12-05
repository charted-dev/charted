/**
 * Typings module for the `winston-logstash` NPM library created by Noel (https://floofy.dev).
 */

declare module 'winston-logstash/lib/winston-logstash-latest' {
  import Transport from 'winston-transport';

  interface Options extends LogstashTransportSSLOptions {
    node_name?: string;
    meta?: Record<string, unknown>;
    ssl_enable?: boolean;
    retries?: number;
    max_connect_retries?: number;
    timeout_connect_retries?: number;
  }

  interface LogstashTransportSSLOptions {
    ssl_key?: string;
    ssl_cert?: string;
    ca?: string;
    ssl_passphrase?: string;
    rejectUnauthorized?: boolean;
  }

  class LogstashTransport extends Transport {
    constructor(options: Options);
  }

  export default LogstashTransport;
}
