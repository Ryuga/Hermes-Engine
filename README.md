# Hermes Code Execution Engine v2

Hermes is a high-performance, sandboxed code execution engine written in Rust. Fully upgraded with modern `cgroups v2` support, it runs untrusted and potentially hostile code inside strictly isolated environments ensuring safety, predictability, and high throughput.

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/Ryuga/Hermes)
[![Status](https://img.shields.io/website?url=https%3A%2F%2Fapi.tortoisecommunity.org&up_message=UP&down_message=DOWN&label=API)](https://execute.tortoisecommunity.org)
[![Status](https://img.shields.io/website?url=https%3A%2F%2Fexecute.tortoisecommunity.org&up_message=UP&down_message=DOWN&label=WEBSITE)](https://execute.tortoisecommunity.org)

---

## About 
Designed for judge platforms, coding sandboxes, and auto-eval services, Hermes provides on-demand execution of arbitrary code through a simple REST API. Version 2 introduces native Docker compatibility, eliminating host kernel configuration headaches by utilizing a custom split-cgroup architecture to strictly enforce resource limits.

---

## 🐳 Docker Deployment (Recommended)

The easiest and safest way to run Hermes Engine is via Docker. 

The provided `Dockerfile` handles all system dependencies, compilers, and the Isolate sandbox engine.

### Requirements
* Docker installed on the host machine.

### 1. Clone & Configure
```bash
git clone https://github.com/Ryuga/Hermes.git
cd Hermes
```

Create a `.env` file in the root directory:
```shell
DEBUG=true  # turns on log output through std_log
HOST=0.0.0.0
PORT=8000
ALLOWED_ORIGIN=https://your_frontend_domain_to_allow_cors.com
API_TOKEN=your_api_token_for_allowing_auth
```

### 2. Build the Image
```bash
docker build -t hermes-engine .
```

### 3. Run the Container
**Note:** Because Hermes manages a strict `cgroups v2` hierarchy internally to track memory and CPU limits, you **must** use the following flags to grant the container a private, unhindered cgroup namespace.

```bash
docker run -d \
  --name hermes \
  --privileged \
  --cgroupns=private \
  --tmpfs /sys/fs/cgroup \
  -p 8000:8000 \
  hermes-engine
```

The application will be live at `http://127.0.0.1:8000`. 

Validate by running `curl http://127.0.0.1:8000`, which should return `UP!`.

---

## 💻 Bare-Metal Deployment

If you prefer not to use Docker, you can run Hermes directly on a Linux server.

> ### ⚠️ **Security Warning**
>
> While `isolate` strictly enforces resource limits and prevents file system modifications, it requires read-only bind mounts to system directories (`/lib`, `/usr`, `/etc`) to provide the necessary runtimes for languages like Python and Java.
> Running Hermes directly on a bare-metal host means untrusted code may be able to read sensitive host system details. 
> 
>**For production, deploying via Docker is strongly recommended**
>
> 
### Requirements
* Linux server with `cgroups v2` enabled
* Rust toolchain installed
* Isolate installed (`apt-get install isolate`)
* Compilers/Runtimes installed (Python, Node, GCC, OpenJDK, etc.)
* Nginx installed (for production proxying)

### 1. Build Application
```bash
git clone [https://github.com/Ryuga/Hermes.git](https://github.com/Ryuga/Hermes.git)
cd Hermes
cargo build --release
```
Binary will be output to: `target/release/Hermes`

### 2. Configure `.env`
```shell
DEBUG=false
HOST=127.0.0.1
PORT=8000
ALLOWED_ORIGIN=https://your_frontend_domain_for_cors.com
```

Ensure your Axum bind address in the Rust code matches the config:
```rust
TcpListener::bind("127.0.0.1:8000")
```

### 3. Run App as a Systemd Service
Create the service file:
```bash
sudo nano /etc/systemd/system/hermes.service
```

```ini
[Unit]
Description=Hermes Engine
After=network.target

[Service]
User=root
WorkingDirectory=/path/to/your/project/root
ExecStart=/path/to/your/project/root/target/release/Hermes
Restart=always
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```
*(Note: Isolate generally requires root privileges to initialize sandboxes. If running as a non-root user, ensure Isolate is properly configured with SUID).*

Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable hermes
sudo systemctl start hermes
```

### 4. Nginx HTTPS Reverse Proxy
```bash
sudo nano /etc/nginx/sites-available/hermes
```

```nginx
server {
    listen 443 ssl; 
    server_name api.hermes.domain;

    ssl_certificate /etc/nginx/certs/origin.crt;
    ssl_certificate_key /etc/nginx/certs/origin.key;

    location / {
        proxy_pass [http://127.0.0.1:8000](http://127.0.0.1:8000);

        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";

        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto https;
    }
}
```

Enable site and restart Nginx:
```bash
sudo ln -s /etc/nginx/sites-available/hermes /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

---

## Code Execution API

Execute user code in a sandboxed environment. Currently supports:
* Python
* C++
* JavaScript
* Java

### Endpoint
```http
POST /execute
```

### Request
**Headers**
```http
Content-Type: application/json
```

**Body**
```json
{
  "language": "python",
  "code": "print(1+1)"
}
```

### Response
```json
{
  "code": 0,
  "output": "2\n",
  "std_log": ""
}
```

**Fields**
* `code` → exit code (`0` = success, non-zero = runtime error / timeout)
* `output` → program stdout
* `std_log` → error output / logs (if `DEBUG=true` or compilation fails)

### Example
```bash
curl -X POST http://127.0.0.1:8000/execute \
  -H "Content-Type: application/json" \
  -d '{"language":"python","code":"print(1+1)"}'
```

---

## Used By
* [Runtime](https://runtime-bot.tortoisecommunity.org) - Discord bot for code execution.
* [Tortoise Community](https://execute.tortoisecommunity.org) - Online Code Execution Tool.
* [Tortoise-BOT](https://tortoise-bot.tortoisecommunity.org) - Discord code execution functionality.
