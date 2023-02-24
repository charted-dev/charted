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
        puts "===> If you wish to use libvirt as the default, run `export VAGRANT_DEFAULT_PROVIDER=libvirt`, otherwise, pass the `--provider=libvirt` flag."
    )
end

Vagrant.configure("2") do |config|
    # Configure VirtualBox
    config.vm.provider "virtualbox" do |vbox|
       vbox.memory = Integer(ENV['BOX_MEMORY'] || 4096)
       vbox.cpus   = Integer(ENV['BOX_CPUS']   || 2)
       vbox.customize ["modifyvm", :id, "--audio", "none"]
    end

    # Configure libvirt
    if Vagrant.has_plugin?("vagrant-libvirt")
       config.vm.provider "libvirt" do |virt|
          virt.memory = Integer(ENV['BOX_MEMORY'] || 4096)
          virt.cpus   = Integer(ENV['BOX_CPUS']   || 2)
       end
    end

    # Configure the main volume for this
    config.vm.synced_folder ".", "/charted",
        create: true,
        owner: "vagrant"

    # Configure Debian
    "debian".tap do |box|
       config.vm.define box do |config|
           config.vm.box = "debian/bullseye64"

           # Since charted-server does optionally require Docker for some tests (like testing external
           # databases like Postgres), we will need to install it
           deb_docker config
       end
    end

    # Configure Windows if we have a `WIN_2022_BOX` environment
    # variable.
    windows_box = ENV["WIN_2022_BOX"] || ""
    if windows_box.empty? == false
        "windows".tap do |box|
            config.vm.define box do |config|
                config.vm.box = windows_box
                windows_docker config
            end
        end
    end
end

def deb_docker(config)
    config.vm.provision "Install Docker via apt", type: 'shell', inline: <<-SHELL
        # Update all packages
        export DEBIAN_FRONTEND=noninteractive
        function install!() {
            apt update
            apt install -y $@
        }

        echo "===> Updating repositories..."
        apt upgrade -y
        install! apt-transport-https ca-certificates curl gnupg2 software-properties-common

        echo "===> Installing common packages..."
        install! git curl neofetch unzip zip bash vim nano libarchive-tools lsb-release pkg-config libssl-dev postgresql-client redis-tools

        # Add Docker's official GPG key
        echo "===> Generating Docker APT repository..."
        mkdir -p /etc/apt/keyrings
        curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg

        # Setup Docker Debian repository
        echo \
            "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \
            $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

        # Now install!
        echo "===> Installing Docker..."
        install! docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

        echo "===> Finished~!"
    SHELL
end

def windows_docker(config)
    config.vm.provision "Install Docker via PowerShell", type: 'shell', inline: <<-SHELL
        Invoke-WebRequest -UseBasicParsing "https://raw.githubusercontent.com/microsoft/Windows-Containers/Main/helpful_tools/Install-DockerCE/install-docker-ce.ps1" -o install-docker-ce.ps1
        .\install-docker-ce.ps1
        Remove-File install-docker-ce.ps1
    SHELL
end
