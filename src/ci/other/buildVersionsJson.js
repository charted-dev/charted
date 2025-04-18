#!/usr/bin/env bun
// @ts-check

/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
 * Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import { s3, which, $, S3Client } from 'bun';

/** @type {S3Client} */
let client = s3;

// for testing, we use MinIO and we want to simulate from the actual
// artifacts registry
if (process.env.NODE_ENV === 'development') {
    const mc = which('mc');
    if (mc !== null) {
        console.log(
            '[dev] found `mc` binary, using credentials from MinIO to test'
        );

        const alias = process.env.MC_ALIAS;
        if (alias === undefined) {
            throw new Error(
                `environment variable $MC_ALIAS is required if \`mc\` is used to grab credentials`
            );
        }

        const info = await $`${mc} alias export ${alias}`.json();
        client = new S3Client({
            endpoint: info.url,
            accessKeyId: info.accessKey,
            secretAccessKey: info.secretKey,
            region: process.env.S3_REGION || 'us-east-1',
            bucket: process.env.S3_BUCKET || 'noelware'
        });
    }
}

const REGISTRY = 'https://artifacts.noelware.org';
const GITHUB = 'https://github.com/charted-dev/charted';
const PREFIX = 'artifacts/charted/server';

// the `versions.json` file that both installers for charted are formatted
// the same as all Noelware products and services, which is:
//
// $lastModifiedAt => iso8601 date of when this was last ran
// versions        => object of "version" -> information
const versions = {
    $lastModifiedAt: new Date().toISOString(),
    versions: {}
};

const objects = await client
    .list({ prefix: PREFIX, maxKeys: 43 })
    .then((m) => m.contents || []);

for (const { key } of objects) {
    const [version, binary] = key.replace(PREFIX, '').split('/').slice(1);
    if (!Object.hasOwn(versions.versions, version)) {
        versions.versions[version] = {};
    }

    // all binaries are formatted as 'ume-linux-x86_64'
    //                                ^^^ ^^^^^ ^^^^^^
    //                               name   os   arch

    const [_, os, arch] = binary.split('-');
    const osInfoKey = `${os}/${arch
        .replace('.exe', '')
        .replace('.sha256', '')}`;

    if (
        !Object.hasOwn(versions.versions, osInfoKey) &&
        !binary.endsWith('.sha256')
    ) {
        versions.versions[version][osInfoKey] = {};
    }

    if (binary.endsWith('.sha256')) {
        versions.versions[version][
            osInfoKey
        ].checksum_url = `${REGISTRY}/charted/server/${version}/${binary}`;

        continue;
    }

    versions.versions[version][
        osInfoKey
    ].download_url = `${REGISTRY}/charted/server/${version}/${binary}`;
    versions.versions[version][
        osInfoKey
    ].changelog_url = `${GITHUB}/releases/${version}`;
}

const util = await import('node:util');
console.log(
    'new versions.json file:\n',
    util.inspect(versions, { depth: Infinity, colors: true })
);

await client
    .write(`${PREFIX}/versions.json`, JSON.stringify(versions), {
        type: 'application/json',
        acl: 'public-read'
    })
    .then((data) => {
        console.log(
            `wrote ${data} bytes into new versions.json file, which can be accessed at: ${REGISTRY}/charted/server/versions.json`
        );
    });
