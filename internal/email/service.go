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

package email

// REFERENCE: https://medium.com/wesionary-team/sending-emails-with-go-golang-using-smtp-gmail-and-oauth2-185ee12ab306

import "net/smtp"

// Config represents the configuration to configure the email service.
type Config struct {
	Password string `toml:"password"`
	Address  string `toml:"address"`
	Host     string `toml:"host"`
	Port     int    `toml:"port"`
}

type Service struct {
	auth smtp.Auth
}

func NewEmailService(config *Config) *Service {
	auth := smtp.PlainAuth("", config.Address, config.Password, config.Host)
	return &Service{auth}
}

func (s *Service) Send(template string) error {
	return nil
}
