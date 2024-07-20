FROM python:3.9-slim-bullseye

WORKDIR /app

RUN apt-get update && \
    apt-get install -y curl gnupg

RUN echo "deb https://packages.cloud.google.com/apt coral-edgetpu-stable main" | tee /etc/apt/sources.list.d/coral-edgetpu.list && \
    curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | apt-key add -

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        libedgetpu1-std \
        && rm -rf /var/lib/apt/lists/*

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
RUN pip install --extra-index-url https://google-coral.github.io/py-repo/ pycoral~=2.0

COPY . .

CMD ["bash"]
