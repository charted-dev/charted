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

package result

import (
	"net/http"
	"noelware.org/charted/server/util"
)

// Result represents a response of the action that was executed. This is used
// in the database controllers.
type Result struct {
	// Success determines if this Result was a success.
	Success bool `json:"success"`

	// Data returns the underlying data that was successful,
	// this can be empty if Result.Errors are nil.
	Data interface{} `json:"data,omitempty"`

	// StatusCode returns the status code to use for this Result object.
	StatusCode int `json:"-"` // this shouldn't be in the JSON object when sent to the end user.

	// Errors returns the underlying Error object of what happened.
	// This is usually used in the result.Err() or result.Errs()
	// function fields.
	Errors []Error `json:"errors,omitempty"`
}

// Error represents the error that occurred in the resulted action.
type Error struct {
	// Code represents the error code that is used.
	Code string `json:"code"`

	// Message is a brief message of what happened.
	Message string `json:"message"`
}

// Ok returns a Result object with the data attached.
func Ok(data interface{}) *Result {
	return OkWithStatus(200, data)
}

// OkWithStatus returns a Result object with a different status code
// rather than 200 OK.
func OkWithStatus(status int, data interface{}) *Result {
	return &Result{
		StatusCode: status,
		Success:    true,
		Data:       data,
	}
}

// NoContent a result object using the 201 status code.
func NoContent() *Result {
	return &Result{
		StatusCode: 204,
	}
}

// Success returns a Result object with no data attached.
func Success() *Result {
	return &Result{
		StatusCode: 200,
		Success:    true,
	}
}

// Err returns a Result object with any error that occurred.
func Err(status int, code string, message string) *Result {
	return &Result{
		StatusCode: status,
		Success:    false,
		Errors: []Error{
			NewError(code, message),
		},
	}
}

// Errs returns a Result object for multiple errors that might've occurred.
func Errs(status int, errors ...Error) *Result {
	return &Result{
		StatusCode: status,
		Errors:     errors,
		Success:    false,
	}
}

// NewError constructs a new Error object.
func NewError(code string, message string) Error {
	return Error{
		Message: message,
		Code:    code,
	}
}

// Write is a convenient method to write this Result object
// to the response writer.
func (r *Result) Write(w http.ResponseWriter) {
	util.WriteJson(w, r.StatusCode, r)
}
