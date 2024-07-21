# Stage 1: Use the official TensorFlow image
# Use the TensorFlow devel Docker image as the base image
FROM tensorflow/tensorflow:devel as tensorflow

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    git \
    build-essential \
    curl \
    unzip \
    llvm \
    clang \
    && apt-get clean

RUN rm -rf /tensorflow_src
RUN git clone https://github.com/tensorflow/tensorflow.git /tensorflow_src
WORKDIR /tensorflow_src

# https://github.com/google-coral/libcoral/blob/release-frogfish/WORKSPACE#L18
RUN git checkout v2.16.1

RUN mkdir /tflite_build
WORKDIR /tflite_build

RUN cmake -DTFLITE_C_BUILD_SHARED_LIBS=ON /tensorflow_src/tensorflow/lite/c

RUN make -j2 # limit the resource because of system breaking issue.

CMD ["bash"]

# Stage 2: Base image
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y wget unzip git flatbuffers-compiler

RUN mkdir -p /tensorflow/lib
RUN mkdir -p /tensorflow/include

COPY --from=tensorflow /tensorflow_src/tensorflow /tensorflow/include/tensorflow
COPY --from=tensorflow /tflite_build/libtensorflowlite_c.so /tensorflow/lib/libtensorflowlite_c.so

RUN mkdir -p /libedgetpu/lib
RUN mkdir -p /libedgetpu/include

RUN wget https://github.com/google-coral/libedgetpu/releases/download/release-grouper/edgetpu_runtime_20221024.zip -O /tmp/edgetpu_runtime.zip \
    && unzip /tmp/edgetpu_runtime.zip -d /tmp/edgetpu_runtime \
    # && cp /tmp/edgetpu_runtime/edgetpu_runtime/libedgetpu/edgetpu.h /libedgetpu/include/ \
    && cp /tmp/edgetpu_runtime/edgetpu_runtime/libedgetpu/throttled/k8/libedgetpu.so.1.0 /libedgetpu/lib/libedgetpu.so

RUN git clone https://github.com/google-coral/libedgetpu.git /libedgetpu/include
RUN find /libedgetpu/include -name "*.fbs" -exec sh -c 'flatc --gen-object-api --force-empty --gen-mutable -o $(dirname {}) -c {}' \;

VOLUME ["/tensorflow", "/libedgetpu"]

CMD ["bash"]
