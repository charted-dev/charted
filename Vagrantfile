# -*- mode: ruby -*-
# vi: set ft=ruby :

# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
# Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

PLUGINS = %w(vagrant-libvirt)
PLUGINS.all? do |plugin|
    Vagrant.has_plugin?(plugin) || (
        puts "The #{plugin} plugin is optional but you might want it if you wish to use libvirt instead of VirtualBox"
        puts "You can install the plugin via: $ vagrant plugin install #{plugin}"
        puts ""
        puts "===> If you wish to use libvirt as the default, run `export VAGRANT_DEFAULT_PROVIDER=libvirt`"
    )
end

Vagrant.configure("2") do |config|
    # Configure Debian
    "debian/bullseye64".tap do |box|
        config.vm.define "debian" do |config|
            config.vm.box = box

            # Install Docker on the host
            config.vm.provision "Install Docker via apt", type: 'shell', inline: <<-SHELL
            export old_frontend=${DEBIAN_FRONTEND}
            export DEBIAN_FRONTEND=noninteractive

            # Update packages
            apt update -y

            # Install required packages
            apt install -y apt-transport-https ca-certificates curl gnupg2 software-properties-common

            # Add Docker's official GPG key
            mkdir -m 0755 -p /etc/apt/keyrings
            curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg

            # setup Docker repository
            echo \
                "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \
                $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

            # Now install Docker
            apt update && apt install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
            SHELL
        end
    end

    # Configure Windows Server 2022
#     "jborean93/WindowsServer2022".tap do |box|
#         config.vm.define "windows" do |config|
#             config.vm.box = box
#
#             # Install Docker for Windows Server
#             config.vm.provision "Install Docker", type: 'shell', inline: <<-SHELL
#                 Invoke-WebRequest -UseBasicParsing "https://raw.githubusercontent.com/microsoft/Windows-Containers/Main/helpful_tools/Install-DockerCE/install-docker-ce.ps1" -o install-docker-ce.ps1
#                 .\install-docker-ce.ps1
#             SHELL
#         end
#     end

    # Sync up this project
    config.vm.synced_folder ".", "/charted",
        create: true,
        owner: "vagrant"

    # Virtualbox-specific settings. Since we do require a bit of consumption
    # when running tests.
    config.vm.provider "virtualbox" do |vbox|
        vbox.memory = Integer(ENV["VAGRANT_MEMORY"] || 4096)
        vbox.cpus = Integer(ENV["VAGRANT_CPU_CORES"] || 2)

        vbox.customize ["modifyvm", :id, "--audio", "none"]
    end

    config.vm.provider "libvirt" do |virt|

    end
end
