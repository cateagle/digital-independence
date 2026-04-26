# PostgreSQL Grundlagen - Spickzettel

## 1. Was ist PostgreSQL?

PostgreSQL (kurz: "Postgres") ist eine robuste, quelloffene relationale Datenbank. Sie ist die Standardwahl für viele [Self-Hosting](../glossar.md#self-hosting)-Projekte und unterstützt komplexe Abfragen, [JSON](../glossar.md#json-javascript-object-notation), Volltextsuche und vieles mehr.

Postgres läuft als Dienst im Hintergrund und nimmt Verbindungen entgegen - ähnlich wie ein Webserver, nur für Datenbankabfragen.

In diesem Guide betreiben wir Postgres als Docker-Container. Das hat mehrere Vorteile: keine direkte Installation auf dem Host-System, einfaches Versions-Management und saubere Trennung vom Rest des Systems.

## 2. Postgres als Docker-Container starten

Der einfachste Weg, Postgres zu starten:

```bash
docker run -d \
  --name postgres \
  -e POSTGRES_USER=meinapp \
  -e POSTGRES_PASSWORD=sicheres_passwort \
  -e POSTGRES_DB=meinapp_db \
  -p 5432:5432 \
  -v postgres_data:/var/lib/postgresql/data \
  postgres:16
```

- `-d` - Container läuft im Hintergrund
- `--name postgres` - Name des Containers
- `-e POSTGRES_USER` / `-e POSTGRES_PASSWORD` / `-e POSTGRES_DB` - Erstellt beim Start automatisch Benutzer und Datenbank
- `-p 5432:5432` - Port nach außen freigeben (nur wenn nötig)
- `-v postgres_data:/var/lib/postgresql/data` - Daten in einem named Volume persistieren

**Status prüfen:**
```bash
docker ps
docker logs postgres
```

PostgreSQL hört jetzt auf Port **5432**.

## 3. Mit psql verbinden

`psql` ist das Kommandozeilenwerkzeug für PostgreSQL. Im Container ist es bereits enthalten.

**psql im laufenden Container ausführen:**
```bash
docker exec -it postgres psql -U meinapp -d meinapp_db
```

Du siehst jetzt den psql-Prompt:
```
psql (16.x)
Type "help" for help.

meinapp_db=#
```

**Als postgres-Superuser einloggen:**
```bash
docker exec -it postgres psql -U postgres
```

**psql beenden:**
```
\q
```

## 4. psql-Grundbefehle

Alle psql-Metabefehle beginnen mit `\` (Backslash) und werden ohne Semikolon ausgeführt.

| Befehl | Bedeutung |
|---|---|
| `\l` | Alle Datenbanken auflisten |
| `\c datenbankname` | Zu einer Datenbank wechseln |
| `\dt` | Alle Tabellen in der aktuellen Datenbank anzeigen |
| `\du` | Alle Benutzer (Rollen) anzeigen |
| `\d tabellenname` | Struktur einer Tabelle anzeigen |
| `\q` | psql beenden |

**Beispiel - Datenbanken anzeigen:**
```
meinapp_db=# \l
```
Ausgabe:
```
   Name      |  Owner   | Encoding
-------------+----------+---------
 meinapp_db  | meinapp  | UTF8
 postgres    | postgres | UTF8
 template0   | postgres | UTF8
 template1   | postgres | UTF8
```

## 5. Benutzer und Datenbanken anlegen

Wenn der Container mit `POSTGRES_USER`, `POSTGRES_PASSWORD` und `POSTGRES_DB` gestartet wurde, sind Benutzer und Datenbank bereits vorhanden. Für weitere Datenbanken oder Benutzer:

**Neuen Datenbankbenutzer anlegen:**
```sql
CREATE USER andereapp WITH PASSWORD 'anderes_passwort';
```

**Neue Datenbank anlegen:**
```sql
CREATE DATABASE andereapp_db OWNER andereapp;
```

**Dem Benutzer alle Rechte auf die Datenbank geben:**
```sql
GRANT ALL PRIVILEGES ON DATABASE andereapp_db TO andereapp;
```

**Alles in einem Schritt (per docker exec):**
```bash
docker exec -i postgres psql -U postgres <<EOF
CREATE USER andereapp WITH PASSWORD 'anderes_passwort';
CREATE DATABASE andereapp_db OWNER andereapp;
GRANT ALL PRIVILEGES ON DATABASE andereapp_db TO andereapp;
EOF
```

**Verbindung mit dem neuen Benutzer testen:**
```bash
docker exec -it postgres psql -h localhost -U andereapp -d andereapp_db
```

## 6. Konfiguration

Im Docker-Container wird die Konfiguration über Umgebungsvariablen und Mount-Punkte gesteuert - nicht über Dateien auf dem Host.

### Umgebungsvariablen

| Variable | Bedeutung |
|---|---|
| `POSTGRES_USER` | Superuser-Name (Standard: `postgres`) |
| `POSTGRES_PASSWORD` | Passwort für den Superuser |
| `POSTGRES_DB` | Name der beim Start angelegten Datenbank |
| `POSTGRES_INITDB_ARGS` | Zusätzliche Argumente für `initdb` |

### Eigene postgresql.conf einbinden

Für eigene Konfigurationseinstellungen kann eine Datei in den Container gemountet werden:

```bash
# Konfigurationsdatei auf dem Host erstellen
cat > /srv/postgres/postgresql.conf <<EOF
max_connections = 200
shared_buffers = 256MB
log_statement = 'all'
EOF
```

```bash
docker run -d \
  --name postgres \
  -e POSTGRES_USER=meinapp \
  -e POSTGRES_PASSWORD=sicheres_passwort \
  -e POSTGRES_DB=meinapp_db \
  -v postgres_data:/var/lib/postgresql/data \
  -v /srv/postgres/postgresql.conf:/etc/postgresql/postgresql.conf \
  postgres:16 -c config_file=/etc/postgresql/postgresql.conf
```

### Konfiguration im laufenden Container prüfen

```bash
docker exec -it postgres psql -U postgres -c "SHOW max_connections;"
docker exec -it postgres psql -U postgres -c "SHOW config_file;"
```

## 7. Mit docker-compose betreiben (empfohlen)

Für echte Projekte ist [Docker Compose](../glossar.md#docker-compose) die bessere Wahl. Damit laufen App und Datenbank im selben Netzwerk und können sich gegenseitig per Servicenamen ansprechen.

**docker-compose.yml:**
```yaml
services:
  app:
    image: meine-app
    environment:
      DATABASE_URL: postgresql://meinapp:sicheres_passwort@db:5432/meinapp_db
    depends_on:
      db:
        condition: service_healthy

  db:
    image: postgres:16
    environment:
      POSTGRES_USER: meinapp
      POSTGRES_PASSWORD: sicheres_passwort
      POSTGRES_DB: meinapp_db
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U meinapp -d meinapp_db"]
      interval: 5s
      timeout: 5s
      retries: 5

volumes:
  postgres_data:
```

Der Hostname `db` wird automatisch aufgelöst, weil beide Container im selben Docker-Netzwerk sind.

**Starten:**
```bash
docker compose up -d
```

**Logs der Datenbank anzeigen:**
```bash
docker compose logs db
```

## 8. Backups mit pg_dump

[`pg_dump`](../glossar.md#pg_dump) erstellt ein konsistentes Backup einer einzelnen Datenbank. Es wird direkt im Container ausgeführt.

**Einfaches Backup erstellen:**
```bash
docker exec postgres pg_dump -U meinapp meinapp_db > /backup/meinapp_db_$(date +%F).sql
```

**Backup als komprimiertes Archiv (empfohlen für große DBs):**
```bash
docker exec postgres pg_dump -U meinapp -Fc meinapp_db > /backup/meinapp_db_$(date +%F).dump
```

**Alle Datenbanken sichern:**
```bash
docker exec postgres pg_dumpall -U postgres > /backup/alle_datenbanken_$(date +%F).sql
```

### Backup zurückspielen mit pg_restore

**Aus SQL-Datei wiederherstellen:**
```bash
docker exec -i postgres psql -U meinapp -d meinapp_db < /backup/meinapp_db_2025-01-15.sql
```

**Aus komprimiertem Archiv wiederherstellen:**
```bash
docker exec -i postgres pg_restore -U meinapp -d meinapp_db < /backup/meinapp_db_2025-01-15.dump
```

**Vor der Wiederherstellung: Datenbank neu erstellen:**
```bash
docker exec postgres psql -U postgres -c "DROP DATABASE IF EXISTS meinapp_db;"
docker exec postgres psql -U postgres -c "CREATE DATABASE meinapp_db OWNER meinapp;"
docker exec -i postgres pg_restore -U meinapp -d meinapp_db < /backup/meinapp_db_2025-01-15.dump
```

## 9. Nützliche Befehle im Überblick

| Befehl | Bedeutung |
|---|---|
| `docker start postgres` | Container starten |
| `docker stop postgres` | Container stoppen |
| `docker restart postgres` | Container neu starten |
| `docker logs postgres` | Logs anzeigen |
| `docker exec -it postgres psql -U postgres` | Als Superuser einloggen |
| `docker exec -it postgres psql -U meinapp -d meinapp_db` | Als App-Benutzer einloggen |
| `docker exec postgres pg_dump -U meinapp meinapp_db > backup.sql` | Datenbank sichern |
| `docker exec -i postgres pg_restore -U meinapp -d meinapp_db < backup.dump` | Backup wiederherstellen |

## 10. Troubleshooting

**Container startet nicht:**
```bash
docker logs postgres
```

**Verbindung schlägt fehl: "could not connect to server"**
```bash
docker ps                  # läuft der Container?
docker logs postgres       # Fehlermeldung im Log?
```

**Verbindung schlägt fehl: "FATAL: password authentication failed"**
- Benutzername oder Passwort falsch
- Umgebungsvariablen beim `docker run` prüfen

**Daten nach Neustart weg?**
- Sicherstellen, dass ein [Volume](../glossar.md#volume-docker) (`-v postgres_data:/var/lib/postgresql/data`) angegeben ist
- Named Volumes (`postgres_data`) bleiben erhalten, anonyme Volumes nicht

**Welche PostgreSQL-Version läuft im Container?**
```bash
docker exec postgres psql -U postgres -c "SELECT version();"
```
