# Strauss

## Stream Automation Micro-Services Solution

-- W I P --

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


### Build
#### Debian
```
./scripts/gen-dev-env.sh
cargo build
cargo test
docker compose build
```

#### Other
*Note* this will produce a target dir owned by root
```
./scripts/gen-dev-env.sh
docker compose --env-file .env -f strauss-build.prod.yml run build
docker compose build
```

#### Create Deployment Package
``` ./scripts/package.sh ```