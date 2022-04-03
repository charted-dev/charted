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

	// GetMetadata is the function to return the repository's metadata
	// on the trailer itself.
	//
	// @param [string] ownerId :: The owner's ID that is used to retrieve.
	//
	// @param [string] repoId  :: The repository ID that is used to retrieve it.
	//
	// @returns [(*storage.RepositoryMetadata, error)] :: Returns the repository metadata
	// as a pointer or an `error` as a tuple if anything goes wrong.
	GetMetadata(ownerId string, repoId string) (*RepositoryMetadata, error) //nolint
	Init()
	Name() string
}
