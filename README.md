# EventPlanner

![SurrealDB](https://img.shields.io/badge/build_with-SurrealDB-%23ff00a0)
![License](https://img.shields.io/badge/License-AGPL%20v3.0-blue)

## Overview

The application allows you to dynamically manage votings / adjustments
for your future events, without a giant data collection. As the application only
relies on the surrealdb backend and a small init-executable for migrations and table adjustments
it's very lightweight without unnecessary functions.

Here is a quick summup of the functionality:

- **Reusable**: each event is directly linked to a particular account, which can be created by everyone
- **Overview**: the application shows you all the needed data concerning the possible dates and the client votes
- **No data-collection**: only a username is saved for the identification by the event organizer for every planned
  visitor. We don't require something like an E-Mail or something comparable
- **Quick**: a simple url sent to the visitor and everything goes on

## Deployment

Currently, are only deployments with docker supported:

| Image                                 | description                                                        | use-case                                            |
|---------------------------------------|--------------------------------------------------------------------|-----------------------------------------------------|
| ghcr.io/randoooom/eventplanner:latest | the full application with a frontend powered by nitrojs and ssr    | everything which doesn't require static             |
| ghcr.io/randoooom/eventplanner:static | only static frontend without any ssr powered by nginx and nuxt-ssg | for static enforced context or low-resource servers |

### Possible configuration

```yaml
# docker-compose.yml

version: "3"
services:
  database:
    image: surrealdb/surrealdb:latest
    environment:
      - SURREAL_USER=<USERNAME>
      - SURREAL_PASS=<PASSWORD>
    volumes:
      - ./data:/var/lib/surreal
    restart: always
    command: start file:///var/lib/surreal
    ports:
      - "8080:8000"
  frontend:
    image: ghcr.io/randoooom/eventplanner:static
    depends_on:
      - database
    environment:
      - RUST_LOG=info
      - SURREALDB_ENDPOINT=database:8000
      - SURREALDB_USERNAME=<USERNAME>
      - SURREALDB_PASSWORD=<PASSWORD>
      - NUXT_PUBLIC_SURREALDB_ENDPOINT=<PUBLIC_SURREALDB_ENDPOINT>
    ports:
      - "8000:80"
```

**nginx:**

```
# site.conf

server {
  listen 80;
  listen [::]:80;
    
  location / {
    proxy_pass http://localhost:8000;
  }

  location /surrealdb/ {
    proxy_pass http://localhost:8080/;
  }
}
```

## Roadmap

- more tests
- ui/ux adjustments
- index page

## License
The Source-Code is licensed under the [AGPL v3.0 license](https://github.com/Randoooom/eventplanner/blob/master/LICENSE.md).
