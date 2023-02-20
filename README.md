# Strauss

## Stream Automation Micro-Services Solution

-- W I P --

### Current Services
#### Redis
Redis Instance supporting PubSub Network

##### Connection Info
Connect clients running in docker @ redis://redis:6379
<p>Use val run targets to expose port 6379 @ redis://localhost:6379 for local testing

#### Chat
Transports Twitch Chat to Redis Network.

##### Channels
|       Channel        |     Description         |
|----------------------|-------------------------|
| #strauss-chat-msg-tx | Transmit to Twitch Chat |
| #strauss-chat-msg-rx | Receive Twitch Chat     |

### Build
#### Debian:Bullseye
Install System Dependencies
```
sudo apt install pkg-config libssl-dev
```

From Repo Root:
```
./scripts/gen-dev-env.sh
cargo build
cargo test
docker compose build
```

#### Other
From Repo Root:
```
./scripts/gen-dev-env.sh
docker compose -f strauss-build.prod.yml run build
docker compose build
```

#### Create Deployment Package
*Note* This will run scripts/gen-prod-env.sh and rewrite your .env file
``` ./scripts/package.sh ```

### Install

##### Host System Requirements:
Docker + Docker Compose Plugin<br />
[install docker enginer](https://docs.docker.com/engine/install/) || [install compose plugin](https://docs.docker.com/compose/install/linux/)
<br />Deployment user in docker group

#### Prepare Host

```
mkdir ~/.strauss && cd ~/.strauss
touch .secrets && chmod 600 .secrets
touch strauss.yml
```

###### .secrets

```
TWITCH_USER=${Bot Username}
TWITCH_TOKEN=${Bot token}
```

Login to twitch as `${Bot Username}` and generate a 'Bot Chat
Token' -> [Token Generator](https://twitchtokengenerator.com/)

###### strauss.yml

```
services:
    chat:
        channel: ${Your Channel}
```

### Deploy

###### Extract

```
cd ~/.strauss
# Download Package
tar zxf strauss-${Version Tag}.tar.gz --strip-components=1
```

###### Start
```./deploy.sh```

##### Manual Deployment
###### Setup Env

```
cat .env > .env.runtime
cat .secrets >> .env.runtime
```

###### Run

```
docker compose --env-file=.env.runtime -f docker-compose.yml -f docker-compose.prod.yml pull
docker compose --env-file=.env.runtime -f docker-compose.yml -f docker-compose.prod.yml up -d --remove-orphans
docker image prune -f
```
