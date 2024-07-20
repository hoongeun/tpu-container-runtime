#!/bin/sh

is_installed() {
  dpkg-query -W -f='${Status}' "$1" 2>/dev/null | grep -c "ok installed"
}

# Package you want to check
libedgetpu="libedgetpu1-std"

# Check if the package is installed
if [ $(is_installed $libedgetpu) -eq 0 ]; then
  echo "Package '$libedgetpu' is not installed. Installing..."
  echo "deb https://packages.cloud.google.com/apt coral-edgetpu-stable main" | sudo tee /etc/apt/sources.list.d/coral-edgetpu.list && \
    curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | sudo apt-key add -
  sudo apt-get -q update
  sudo apt-get install -qy $libedgetpu
else
  echo "Package '$libedgetpu' is already installed. Skipping installation."
fi

if ! docker images -q classify-image > /dev/null 2>&1; then
  echo "Docker image 'classify-image' not found. Building the image..."
  docker build -t classify-image . -f lib-packaged.Dockerfile
else
  echo "Docker image 'classify-image' already exists. Skipping the build step."
fi

docker run -it --privileged \
  --device /dev/bus/usb:/dev/bus/usb \
  classify-image \
  python app.py --input validation/parrot.jpg
