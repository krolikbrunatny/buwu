#!/usr/bin/env bash

{ # this ensures the entire script is downloaded #

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
  read -p "Library card number: " username
  read -s -p "Password: " password
  echo

  # Save credentials
  cat > $CONFIG << EOL
  username = "${username}"
  password = "${password}"
  days_threshold = 4
EOL

  echo "Validating credentials..."
  $BINARY --test
  if [ $? -eq 0 ]; then
    break
  else
    echo "Incorrect card number or password, try again."
  fi
done

# Install crontab
CMD="0 */4 * * * ${BINARY} >> ${LOG_FILE} 2>&1 && echo >> ${LOG_FILE}"
(crontab -l 2>/dev/null | grep -v ".buwu/buwu") | crontab - # clean
(crontab -l 2>/dev/null; echo "$CMD") | crontab -
if [ $? -eq 0 ]; then
  echo "Installation successful!"
fi

} # this ensures the entire script is downloaded #