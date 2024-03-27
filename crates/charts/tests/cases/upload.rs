// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// this is temporarily blocked since there is no *easy* way of generating
// a multipart stream (without diving into how `multer` works), but this
// can be raised as its own separate issue.

// #[tokio::test]
// async fn test_upload() {
//     // ~ we will keep track of the temporary directory
//     let tempdir = TempDir::new().unwrap();
//     let path = tempdir.into_path();
//     let storage = StorageService::Filesystem(remi_fs::StorageService::with_config(remi_fs::Config::new(&path)));

//     // run our tests in a separate block
//     {
//         let storage = storage.clone();
//         let charts = HelmCharts::new(storage);

//         // should succeed
//         let stream = once(async move {
//             let contents = fs::read(fixture("youtrack.tgz")).unwrap();

//             let mut bytes = BytesMut::new();
//             bytes.extend(b"--charted-boundary\r\n;Content-Disposition: form-data; name=\"youtrack.tgz\"\r\n");
//             bytes.extend(b"Content-Type: application/tar+gzip\r\n\r\n");
//             bytes.extend(contents);
//             bytes.extend(b"\r\n--charted-boundary--\r\n");

//             Result::<Bytes, Infallible>::Ok(bytes.into())
//         });

//         let multipart = Multipart::new(stream, "--charted-boundary");
//         charts
//             .upload(
//                 UploadReleaseTarballRequest {
//                     owner: 1,
//                     repo: 3,
//                     version: String::from("2023.3.23390"),
//                 },
//                 multipart,
//             )
//             .await
//             .unwrap();
//     }

//     // clean up the storage service so we don't dangle the `path` from being destroyed since it
//     // is a reference to the tempdir
//     mem::drop(storage);
//     fs::remove_dir_all(path).expect("tempdir to be removed by now");
// }
