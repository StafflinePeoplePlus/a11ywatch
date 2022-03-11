pub fn generate_compose_backend() -> &'static str {
    &"
services:
  api:
    container_name: api
    image: a11ywatch/a11ywatch-core
    networks:
      - back-net
      - front-net
    ports:
      - 3280:8080
    depends_on:
      - mongodb
    environment:
      - DB_URL=${DB_URL:-mongodb://mongodb:27017/?compressors=zlib&gssapiServiceName=mongodb}
      - DB_NAME=${DB_NAME:-a11ywatch}
      - APOLLO_GRAPH_VARIANT=${APOLLO_GRAPH_VARIANT:-current}
      - APOLLO_SCHEMA_REPORTING=${APOLLO_SCHEMA_REPORTING:-false}
      - CLIENT_URL=${CLIENT_URL:-http://localhost:3000}
      - WATCHER_CLIENT_URL=${WATCHER_CLIENT_URL:-http://crawler:8000}
      - GRAPHQL_PORT=${GRAPHQL_PORT:-8080}
      - EMAIL_SERVICE_URL=${EMAIL_SERVICE_URL}
      - EMAIL_CLIENT_ID=${EMAIL_CLIENT_ID}
      - EMAIL_CLIENT_KEY=${EMAIL_CLIENT_KEY}
      - STRIPE_KEY=${STRIPE_KEY}
      - SCRIPTS_CDN_URL=${SCRIPTS_CDN_URL:-http://localhost:8090/api}
      - PUPPET_SERVICE=${PUPPET_SERVICE:-http://pagemind:8040}
      - ROOT_URL=${ROOT_URL:-http://localhost:8080}
      - STRIPE_BASIC_PLAN=${STRIPE_BASIC_PLAN}
      - STRIPE_PREMIUM_PLAN=${STRIPE_PREMIUM_PLAN}
      - MAV_CLIENT_URL=${MAV_CLIENT_URL:-http://127.0.0.1:8020}
      - PRIVATE_KEY=${PRIVATE_KEY}
      - PUBLIC_KEY=${PUBLIC_KEY}
      - ADMIN_ORIGIN=${ADMIN_ORIGIN}
      - BACKGROUND_CRAWL=${BACKGROUND_CRAWL:-true}
      - REDIS_CLIENT=${REDIS_CLIENT:-redis://redis:6379}
      - REDIS_HOST=redis
      - DOCKER_ENV=true

  pagemind:
    container_name: pagemind
    image: a11ywatch/pagemind
    networks:
      - back-net
    ports:
      - 8040:8040
    environment:
      - SCRIPTS_CDN_URL=${SCRIPTS_CDN_URL:-http://127.0.0.1:8090/api}
      - SCRIPTS_CDN_URL_HOST=${SCRIPTS_CDN_URL_HOST:-http://localhost:8090/cdn}
      - AI_SERVICE_URL=${SCRIPTS_CDN_URL:-http://127.0.0.1:8020}
      - PORT=${PAGEMIND_PORT:-8040}
      - BACKUP_IMAGES=${BACKUP_IMAGES:-true}
      - PUPPET_POOL_MAX=${PUPPET_POOL_MAX:-4}

  mav:
    container_name: mav
    image: a11ywatch/mav
    networks:
      - back-net
    ports:
      - 8020:8020
    environment:
      - MAIN_API_URL=${MAIN_API_URL:-http://127.0.0.1:8080}
      - PORT=${MAV_PORT:-8020}
      - DOCKER_ENV=true

  cdn-server:
    container_name: cdn-server
    image: a11ywatch/cdn-server
    networks:
      - back-net
    ports:
      - 8090:8090
    environment:
      - PORT=${ELASTIC_CDN_PORT:-8090}

  crawler:
    container_name: crawler
    image: a11ywatch/crawler
    networks:
      - back-net
    ports:
      - 8000:8000

  mongodb:
    container_name: mongodb
    networks:
      - back-net
    image: mongo
    ports:
      - 27017:27017
    volumes:
      - mongodb:/data/db
    environment:
      - MONGO_INITDB_DATABASE=${DB_NAME:-a11ywatch}

  redis:
    container_name: redis
    image: bitnami/redis:6.0
    environment:
      - ALLOW_EMPTY_PASSWORD=yes
    networks:
      - back-net
      - front-net

networks:
  back-net:
  front-net:
volumes:
  mongodb:

  "
}

pub fn generate_compose_frontend() -> &'static str {
  &"
services:
  web:
    container_name: web
    image: a11ywatch/web
    ports:
      - 3270:3000
    networks:
      - front-net
    environment:
      - PORT=3000
      - API=${API:-http://localhost:3280/graphql}
      - WEB_SOCKET_URL=${WEB_SOCKET_URL:-ws://localhost:3280/graphql}
networks:
  front-net:

"
}