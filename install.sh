#!/usr/bin/env bash

# Pull binary
INSTALL_DIR="$(pwd)/buwu"
mkdir -p $INSTALL_DIR
PLATFORM=$(uname -s)
ARCH=$(uname -m)
# aarch64 -> arm64
if [ "$ARCH" = "aarch64" ]; then
  ARCH="arm64"
fi

ARCHIVE="buwu-$PLATFORM-$ARCH.tar.gz"
SUPPORTED_PLATFORMS=("buwu-Darwin-arm64.tar.gz", "buwu-Linux-x86_64.tar.gz")

if ! [[ ${SUPPORTED_PLATFORMS[@]} =~ $ARCHIVE ]]; then
  echo "Binary not available for this system. Please build project manually."
  exit 1
fi

echo "Downloading binary..."
curl -s -L https://github.com/krolikbrunatny/buwu/releases/download/v0.0.1/$ARCHIVE | tar xz -C ./buwu

# Enable legacy unsafe renegotiation
OPENSSL_CFG="openssl_conf = openssl_init

[openssl_init]
ssl_conf = ssl_sect

[ssl_sect]
system_default = system_default_sect

[system_default_sect]
Options = UnsafeLegacyRenegotiation
"
printf "$OPENSSL_CFG" > $INSTALL_DIR/openssl.cfg
OPENSSL_ENV="OPENSSL_CONF=${INSTALL_DIR}/openssl.cfg"

# Create config file
CONFIG="${INSTALL_DIR}/config.toml"
BINARY="${INSTALL_DIR}/buwu"
LOG_FILE="${INSTALL_DIR}/buwu.log"

if [ -f "$CONFIG" ]; then
    rm "$CONFIG"
fi
touch $CONFIG
chmod 600 $CONFIG

# Get credentials
while true
do
  read -p "Library card number: " username < /dev/tty
  read -s -p "Password: " password < /dev/tty
  echo

  # Save credentials
  cat > $CONFIG << EOL
username = "${username}"
password = "${password}"
days_threshold = 4
EOL

  echo "Validating credentials..."
  (export ${OPENSSL_ENV}; ${BINARY} --test)
  if [ $? -eq 0 ]; then
    break
  else
    echo "Incorrect card number or password, try again."
  fi
done

# Install crontab
# Every 6 hours at random minute
RANDOM_MINUTE=$(($RANDOM % 60))
CMD="${RANDOM_MINUTE} */6 * * * (export ${OPENSSL_ENV}; ${BINARY} >> ${LOG_FILE} 2>&1 && echo >> ${LOG_FILE})"
(crontab -l 2>/dev/null | grep -v ".buwu/buwu") | crontab - # clean
(crontab -l 2>/dev/null; echo "$CMD") | crontab -
if [ $? -eq 0 ]; then
  echo "Installation successful!"
fi
