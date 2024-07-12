FROM ubuntu:22.04

RUN apt-get update && apt-get install -y python3 python3-pip \
    && echo "deb https://packages.cloud.google.com/apt coral-edgetpu-stable main" | tee /etc/apt/sources.list.d/coral-edgetpu.list \
    && curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | apt-key add - \
    && apt-get update \
    && apt-get install -y libedgetpu1-std

RUN pip3 install tflite-runtime

# Copy application code
COPY app.py /app.py

# Set up entrypoint
ENTRYPOINT ["python3", "/app.py"]
