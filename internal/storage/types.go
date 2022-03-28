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

// File represents the file that is being uploaded to the storage trailer.
type File struct {
	// ContentType returns the content type of this File.
	ContentType string `json:"content_type"`

	// Path returns the storage path. If this is the filesystem storage,
	// this will return the full path; if this is in S3 or GCS, it will be
	// prefixed with `s3:` or `gcs:` with the object path.
	Path string `json:"path"`

	// Size returns in bytes of how large this File is.
	Size int64
}

// UploadRequest represents the request_upload object that is sent when you
// hit the /storage/:user/:project/upload endpoint. This must be embedded
// under the `request_upload` parameter when uploading.
type UploadRequest struct {
	// ContentType refers to the content type this file is. By default,
	// it will be "application/octet-stream"
	ContentType string `json:"content_type"`

	// Name refers to the file name that is used to upload. If this
	// is `nil`, this will refer to the default file name that is used
	// from the request itself.
	Name *string `json:"name,omitempty"`

	// Size refers to the bytes on how long this file is, mainly for
	// metadata.
	Size int64 `json:"size"`
}

// RepositoryMetadata represents the metadata stored under the following:
//    - Filesystem: <DIRECTORY>/projects/:uid/:name/metadata.lock
//    - S3: <BUCKET>/projects/:uid/:name/metadata.lock
//
// This holds an JSON file of the calculated metadata available.
type RepositoryMetadata struct {
	// Description refers to the project description, if any.
	Description *string `json:"description"`

	// OwnerId is the ID of the owner who owns this repository.
	OwnerId string `json:"owner_id"`

	// Files is a list of files that are available.
	Files []File `json:"files"`

	// Name is the repository name.
	Name string `json:"name"`

	// Id is the repository ID.
	Id string `json:"id"`
}
