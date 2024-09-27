#!/bin/bash

sudo -v

echo "Installing rust"
if command -v rustc &> /dev/null
then
    echo "Rust is already installed."
    echo ""

else
    # Install Rust using rustup (Rust installation manager)
    echo "Installing Rust..."
    sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    # Add Rust to the current session's PATH
    source $HOME/.cargo/env

    echo "Rust installed successfully."
    echo ""

fi

# Add wasm32-unknown-unknown target for WebAssembly
echo "Adding WebAssembly target (wasm32-unknown-unknown)..."
rustup target add wasm32-unknown-unknown

echo "WebAssembly target added successfully."
echo ""

echo "Installing protoc-gen-* tools"
cargo install protoc-gen-prost protoc-gen-prost-crate
echo "protoc-gen-* tools installed succesfully."
echo ""

echo "Installing other require tools for development."
sudo apt-get -y install --no-install-recommends ca-certificates gnupg2 tar gcc make pkg-config netcat-openbsd
echo "Utility tools installed successfully."
echo ""


echo "Installing buf"
if command -v buf &> /dev/null
then
    echo "Buff is already installed."
    echo ""

else
    LINK=$(curl -s https://api.github.com/repos/bufbuild/buf/releases/latest | awk "/download.url.*buf-Linux-$(uname -m)\"/ {print \$2}" | sed 's/"//g')
    sudo curl -L $LINK -o /usr/bin/buf && sudo chmod +x /usr/bin/buf
    echo "Buf installed successfully."
    echo ""
fi

echo "Installed toml-cli"
cargo install toml-cli
echo "Toml-cli installed successfully."
echo ""

echo "Installing substreams..."
if command -v substreams &> /dev/null
then
    echo "Substreams already installed."
    echo ""

else
    echo "Substreams not found. Installing..."
    sudo curl -o /usr/bin/substreams https://storage.googleapis.com/substreams-registry/bin/substreams
    sudo chmod +rx /usr/bin/substreams
    echo "Insalled substreams successfully."
    echo ""

fi

echo "Installing nvm for installing Node.js."
if command -v nvm &> /dev/null
then
    echo "Nvm is already installed."
    echo ""
else
    # installs nvm (Node Version Manager)
    sudo curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
    echo ""
    echo "Running nvm bash completion script for using it now..."
    export NVM_DIR="$HOME/.nvm"
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
    [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
    echo "Nvm installed successfully."
    echo ""
fi

# Check if Node.js is installed
echo "Installing Node.js version 20..."
if command -v node &> /dev/null
then
    NODE_VERSION=$(node -v | grep -oP "\d+\.\d+\.\d+" | cut -d. -f1)
    echo "Node.js version $NODE_VERSION is installed."
    # Check if Node.js was installed via a package manager (apt, yum, snap)
    
    echo "Uninstalling node if installed from other package manager.."
    if dpkg -s nodejs &> /dev/null; then
        echo "Node.js was installed via apt. Uninstalling..."
        sudo apt-get remove --purge -y nodejs
        sudo apt-get autoremove -y
        echo ""
    elif rpm -q nodejs &> /dev/null; then
        echo "Node.js was installed via yum. Uninstalling...\n"
        sudo yum remove -y nodejs
        echo ""
    elif snap list node &> /dev/null; then
        echo "Node.js was installed via snap. Uninstalling...\n"
        sudo snap remove node
        echo ""
    else
        NODE_PATH=$(which node)
        NODE_DIR=$(dirname $(dirname "$NODE_PATH"))    
        
        if [[ "$NODE_PATH" == *"$HOME/.nvm"* ]]; then
            if [ "$NODE_VERSION" -lt 20 ]; then
                echo "Node.js version is less than 20."
                echo "Installing Node.js version 20."
                nvm install 20
                nvm use 20
                source $HOME/.bashrc
                NODE_VERSION=$(node -v)
                echo "Node.js version $NODE_VERSION installed successfully."
                echo ""
            fi
        else
            echo "Node.js seems to installed manually. at $NODE_DIR. Removing manually."
            sudo rm -rf "$NODE_DIR"
        fi
else
    echo "No Node.js installation detected."
    echo "Installing Node.js version 20."
    nvm install 20
    nvm use 20
    source $HOME/.bashrc
    NODE_VERSION=$(node -v)
    echo "Node.js version $NODE_VERSION installed successfully."
    echo ""
fi

echo "Installing graph cli"
if command -v graph &> /dev/null
then 
    echo "Graph cli is already installed"
else
    npm install -g @graphprotocol/graph-cli
    echo "Graph cli installed successfully"
fi