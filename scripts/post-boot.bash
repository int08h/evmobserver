# Post Ubutnu 18.04 LTS install
sudo apt remove lvm2 -y --purge
sudo apt remove at -y --purge
sudo apt remove snapd -y --purge
sudo systemctl stop cron
sudo systemctl disable cron
sudo apt remove lxcfs -y --purge
sudo apt remove policykit-1 -y --purge
sudo apt remove open-iscsi -y --purge

# Create environment variable for correct distribution
export CLOUD_SDK_REPO="cloud-sdk-$(lsb_release -c -s)"

# Add the Cloud SDK distribution URI as a package source
echo "deb http://packages.cloud.google.com/apt $CLOUD_SDK_REPO main" | sudo tee -a /etc/apt/sources.list.d/google-cloud-sdk.list

# Import the Google Cloud Platform public key
curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | sudo apt-key add -

sudo apt-get update
sudo apt-get upgrade
sudo apt-get install build-essential binutils-dev libunwind-dev gdb lldb linux-tools-common linux-tools-gcp 
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

# If nightly is needed
#rustup toolchain install nightly
#rustup default nightly

# Google Cloud SDK
sudo apt-get install -y google-cloud-sdk python-boto
sudo rm -f /etc/boto.cfg


