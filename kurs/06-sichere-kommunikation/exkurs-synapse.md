# Exkurs: Eigenen Matrix-Server (Synapse) aufsetzen

Dieser Exkurs zeigt, wie du einen eigenen Matrix-Homeserver mit Synapse betreibst. Das ist nicht notwendig, um Matrix zu nutzen – du kannst einfach einen Account auf `matrix.org` oder einem anderen öffentlichen Server anlegen. Der Eigenbetireb gibt dir vollständige Kontrolle: Deine Nachrichten und Medien liegen auf deiner Infrastruktur, und du kannst Accounts selbst verwalten.

## Was dafür nötig ist

- Ein Server mit öffentlicher IP (VPS empfohlen)
- Eine Domain, deren DNS du kontrollierst
- Docker und Docker Compose (aus Session 2)
- Einen offenen Port 443 (HTTPS) und 8448 (Matrix-Federation)

**Warum brauche ich eine öffentliche Domain?**
Matrix-IDs sind an eine Domain gebunden (`@alice:example.com`). Eine lokale VM reicht für Tests, aber für echte Nutzung und Federation mit anderen Servern brauchst du eine öffentlich erreichbare Domain.

## Schritt 1: Verzeichnis anlegen

```bash
mkdir ~/synapse
cd ~/synapse
```

## Schritt 2: Konfiguration generieren

Synapse kann seine eigene Konfiguration generieren. Dabei wird auch der **Signing Key** erstellt – der kryptographische Identitätsausweis deines Servers im Matrix-Netzwerk.

```bash
docker run -it --rm \
  -v $(pwd)/data:/data \
  -e SYNAPSE_SERVER_NAME=matrix.example.com \
  -e SYNAPSE_REPORT_STATS=no \
  matrixdotorg/synapse:latest generate
```

Das erzeugt `data/homeserver.yaml` und `data/matrix.example.com.signing.key`.

> **Den Signing Key sichern.** Wenn der Key verloren geht, erkennen andere Matrix-Server deinen Homeserver nicht mehr. Er muss im Backup enthalten sein.

## Schritt 3: Docker Compose aufsetzen

```bash
nano docker-compose.yml
```

```yaml
services:
  synapse:
    image: matrixdotorg/synapse:latest
    container_name: synapse
    restart: unless-stopped
    volumes:
      - ./data:/data
    depends_on:
      db:
        condition: service_healthy
    ports:
      - "8008:8008"

  db:
    image: postgres:16-alpine
    container_name: synapse-db
    restart: unless-stopped
    environment:
      POSTGRES_USER: synapse
      POSTGRES_PASSWORD: <dein-sicheres-passwort>
      POSTGRES_DB: synapse
      POSTGRES_INITDB_ARGS: "--encoding=UTF-8 --lc-collate=C --lc-ctype=C"
    volumes:
      - ./postgres-data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U synapse"]
      interval: 5s
      timeout: 5s
      retries: 10
```

**Warum PostgreSQL statt SQLite?**
Synapse verwendet standardmäßig SQLite. Das ist für Tests ausreichend, aber bei echtem Betrieb (insbesondere in großen Räumen mit vielen Teilnehmern) wird SQLite schnell zum Flaschenhals. PostgreSQL ist für jeden größeren produktiven Betrieb notwendig. Wenn es nur ein server für die eigene Familie ist, dann kann SQLite natürlich auch ausreichen.

**Warum `POSTGRES_INITDB_ARGS` mit `lc-collate=C`?**
Synapse setzt diese Werte voraus. Eine falsch konfigurierte Datenbank führt zu Fehlern beim Start. Das ist keine Empfehlung, sondern eine Anforderung.

## Schritt 4: Synapse auf PostgreSQL umstellen

Die generierte `homeserver.yaml` nutzt SQLite. Das muss geändert werden, bevor der Server zum ersten Mal mit echten Daten läuft.

```bash
nano data/homeserver.yaml
```

Den SQLite-Block ersetzen:
```yaml
# Vorher (entfernen):
# database:
#   name: sqlite3
#   args:
#     database: /data/homeserver.db

# Nachher:
database:
  name: psycopg2
  args:
    user: synapse
    password: <dein-sicheres-passwort>
    database: synapse
    host: db
    cp_min: 5
    cp_max: 10
```

Der Hostname `db` entspricht dem Containernamen in Docker Compose. Docker löst das intern auf.

`psycopg2` ist der Datenbanktreiber den synapse für die Kommunikation mit postgresql verwendet. Nicht drüber wundern.

`<dein-sicheres-passwort>` durch ein sicheres Passwort austauschen.

## Schritt 5: Reverse Proxy konfigurieren

Synapse lauscht intern auf Port 8008 (HTTP). Nach außen soll der Server über HTTPS auf Port 443 erreichbar sein. Das übernimmt Nginx mit einem Let's Encrypt-Zertifikat (aus Session 3 bekannt).

```nginx
server {
    listen 443 ssl;
    server_name matrix.example.com;

    ssl_certificate /etc/letsencrypt/live/matrix.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/matrix.example.com/privkey.pem;

    location / {
        proxy_pass http://localhost:8008;
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-For $remote_addr;
        proxy_set_header X-Forwarded-Proto https;
        proxy_read_timeout 600;
    }
}

# Federation-Port
server {
    listen 8448 ssl;
    server_name matrix.example.com;

    ssl_certificate /etc/letsencrypt/live/matrix.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/matrix.example.com/privkey.pem;

    location / {
        proxy_pass http://localhost:8008;
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-For $remote_addr;
        proxy_set_header X-Forwarded-Proto https;
    }
}
```

**Warum Port 8448?**
Andere Matrix-Server kontaktieren deinen Server über Port 8448 für Federation. Ohne diesen Port kannst du nur mit Nutzern auf deinem eigenen Server kommunizieren.

## Schritt 6: Starten und ersten Admin-Account anlegen

```bash
docker compose up -d

# Logs beobachten
docker compose logs -f synapse
```

Wenn Synapse läuft, einen Admin-Account anlegen:

```bash
docker exec -it synapse register_new_matrix_user \
  -c /data/homeserver.yaml \
  -u admin \
  -p <dein-sicheres-passwort> \
  -a \
  http://localhost:8008
```

`-a` setzt den Account als Administrator.

**Offene Registrierung deaktivieren.** Standardmäßig kann sich jeder einen Account anlegen. Für einen privaten Server sollte das in `homeserver.yaml` abgeschaltet werden:

```yaml
enable_registration: false
```

## Was läuft jetzt

```bash
docker compose ps
```

Zwei Container:
- `synapse`: Der Matrix-Homeserver
- `synapse-db`: Die PostgreSQL-Datenbank

Erreichbar über `https://matrix.example.com`. In Element als Homeserver `matrix.example.com` eintragen.

## Backups

Ohne Backup ist ein Synapse-Betrieb verantwortungslos. Drei Dinge müssen gesichert werden:

| Was                  | Wo                                      | Warum                                                    |
|----------------------|-----------------------------------------|----------------------------------------------------------|
| PostgreSQL-Datenbank | `./postgres-data`                       | Alle Nachrichten und Nutzerkonten                        |
| Mediendateien        | `./data/media_store`                    | Hochgeladene Bilder, Dateien                             |
| Signing Key          | `./data/matrix.example.com.signing.key` | Ohne ihn verliert der Server seine Identität im Netzwerk |

## Weiterführende Dokumentation

- [Offizielle Synapse-Dokumentation](https://element-hq.github.io/synapse/latest/)
- [Docker Hub: matrixdotorg/synapse](https://hub.docker.com/r/matrixdotorg/synapse)
