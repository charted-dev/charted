// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Go.
// Copyright 2022 Noelware <team@noelware.org>
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

package storage

type BaseStorageTrailer interface {
	// HandleUpload is the function to handle file uploads in this BaseStorageTrailer.
	//
	// @param [storage.UploadRequest] files :: Represents the files to upload that
	// was calculated on the request.
	//
	// @returns [error] :: Returns an error if anything occurred or `nil`.
	HandleUpload(files []UploadRequest) error

	GetIndexYaml(ownerID string, repoID string) (string, error)
	Init()
	Name() string
}
