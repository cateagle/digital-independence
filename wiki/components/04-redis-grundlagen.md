# Redis Grundlagen - Spickzettel

## 1. Was ist Redis?

Redis steht für "Remote Dictionary Server". Es ist ein **[In-Memory Key-Value-Speicher](../glossar.md#key-value-speicher-key-value-store)** - alle Daten liegen im RAM, weshalb Zugriffe extrem schnell sind (typisch unter 1 Millisekunde).

Redis speichert Daten als Schlüssel-Wert-Paare:
```
Schlüssel: "user:42:session"
Wert:      "eyJhbGciOiJIUzI1NiJ9..."
```

**Redis ist keine Ersatz-Datenbank für [PostgreSQL](../glossar.md#postgresql).** Es ergänzt eine Hauptdatenbank als schneller Zwischenspeicher oder für spezielle Aufgaben wie Sessions und Queues.

**Typische Einsatzzwecke:**
- [Cache](../glossar.md#cache) (Ergebnisse teurer Datenbankabfragen zwischenspeichern)
- Session-Speicher (Wer ist gerade eingeloggt?)
- Job-Queues (Hintergrundaufgaben verwalten)
- [Rate Limiting](../glossar.md#rate-limiting) (Anfragen pro IP zählen)
- [Pub/Sub](../glossar.md#pubsub-publishsubscribe) (Nachrichten zwischen Prozessen verteilen)

## 2. Installation mit Docker

### Einfacher Start

```bash
docker run -d \
  --name redis \
  -p 127.0.0.1:6379:6379 \
  redis:7-alpine
```

- `-d` - Container läuft im Hintergrund
- `--name redis` - Container-Name für einfachen Zugriff
- `-p 127.0.0.1:6379:6379` - Port nur auf localhost binden (sicher)
- `redis:7-alpine` - offizielles Image, Alpine-Variante (klein)

**Status prüfen:**
```bash
docker ps
docker logs redis
```

### Mit Docker Compose (empfohlen)

Für den Einsatz zusammen mit anderen Services legt man Redis in einer `compose.yml` an:

```yaml
services:
  redis:
    image: redis:7-alpine
    restart: unless-stopped
    volumes:
      - redis_data:/data
    command: redis-server --appendonly yes --requirepass dein_sicheres_passwort

volumes:
  redis_data:
```

```bash
docker compose up -d
```

Redis hört standardmäßig auf Port **6379**. Da kein `ports:`-Abschnitt angegeben ist, ist der Port nur für andere Container im selben Compose-Netzwerk erreichbar - das ist für die meisten Anwendungsfälle richtig.

## 3. redis-cli - Das Kommandozeilentool

`redis-cli` wird direkt im laufenden Container ausgeführt:

**Interaktiver Modus:**
```bash
docker exec -it redis redis-cli
```

Du siehst den Prompt:
```
127.0.0.1:6379>
```

**Einzelnen Befehl ausführen:**
```bash
docker exec redis redis-cli PING
```
Ausgabe: `PONG` (Redis läuft und antwortet)

**Mit Passwort verbinden:**
```bash
docker exec -it redis redis-cli -a dein_sicheres_passwort
# oder innerhalb von redis-cli:
AUTH dein_sicheres_passwort
```

**redis-cli beenden:**
```
quit
```
oder `Strg + C`

## 4. Grundlegende Befehle

### SET und GET - Wert speichern und lesen

```bash
# Wert speichern
SET meinschluessel "Hallo Welt"

# Wert lesen
GET meinschluessel
```
Ausgabe: `"Hallo Welt"`

### Ablaufzeit setzen (TTL - Time To Live)

```bash
# Wert speichern, der nach 60 Sekunden automatisch gelöscht wird
SET session:user42 "daten" EX 60

# Verbleibende Lebensdauer in Sekunden abfragen
TTL session:user42
```
Ausgabe: `(integer) 58` (je nach vergangener Zeit)

- `TTL` gibt `-1` zurück, wenn kein Ablaufzeitpunkt gesetzt ist
- `TTL` gibt `-2` zurück, wenn der Schlüssel nicht existiert

**Ablaufzeit nachträglich setzen:**
```bash
EXPIRE meinschluessel 120   # läuft in 120 Sekunden ab
PERSIST meinschluessel      # Ablaufzeit entfernen (dauerhaft speichern)
```

### KEYS - Schlüssel suchen

```bash
# Alle Schlüssel anzeigen (nur auf Entwicklungsinstanzen!)
KEYS *

# Schlüssel nach Muster suchen
KEYS session:*
KEYS user:42:*
```

Ausgabe:
```
1) "session:user42"
2) "session:user99"
```

**Achtung:** `KEYS *` blockiert Redis kurz. Auf Produktionssystemen stattdessen `SCAN` verwenden.

### Weitere nützliche Befehle

```bash
# Existiert ein Schlüssel?
EXISTS meinschluessel
# 1 = ja, 0 = nein

# Schlüssel löschen
DEL meinschluessel

# Alle Schlüssel löschen (VORSICHT!)
FLUSHDB

# Wie viele Schlüssel gibt es?
DBSIZE

# Informationen über Redis anzeigen
INFO server
INFO memory
```

### Zahlen speichern und erhöhen (Zähler)

Redis kann Zahlen atomar erhöhen - ideal für Rate-Limiting oder Statistiken:

```bash
SET seitenaufrufe 0
INCR seitenaufrufe    # erhöht um 1 → 1
INCR seitenaufrufe    # erhöht um 1 → 2
INCRBY seitenaufrufe 10  # erhöht um 10 → 12

GET seitenaufrufe
# "12"
```

## 5. Cache vs. Session-Store

### Redis als Cache

Beim Caching speicherst du Ergebnisse teurer Operationen temporär, damit sie schnell wieder abrufbar sind.

**Typisches Muster (Pseudocode):**
```
1. Schlüssel in Redis prüfen (SCHNELL)
2. Wenn vorhanden → direkt zurückgeben
3. Wenn nicht vorhanden → Datenbankabfrage ausführen (LANGSAM)
4. Ergebnis in Redis speichern (mit TTL)
5. Ergebnis zurückgeben
```

**Beispiel in redis-cli:**
```bash
# Simuliertes Cache-Ergebnis speichern (läuft nach 5 Minuten ab)
SET cache:artikel:42 '{"titel":"Mein Artikel","inhalt":"..."}' EX 300

# Beim nächsten Aufruf aus dem Cache lesen
GET cache:artikel:42
```

**Wann ist Cache sinnvoll?**
- Datenbankabfragen, die sich selten ändern
- API-Antworten externer Dienste
- Aufwändige Berechnungen

### Redis als Session-Store

Statt Sessions in der Datenbank oder Dateien zu speichern, legt die Anwendung sie in Redis ab. Das ist schnell und funktioniert auch wenn mehrere Server-Instanzen laufen.

```bash
# Session anlegen (läuft nach 24 Stunden ab)
SET session:abc123 '{"user_id":42,"name":"Alice","role":"admin"}' EX 86400

# Session lesen (bei jedem Request)
GET session:abc123

# Session beim Logout löschen
DEL session:abc123
```

**Vorteil gegenüber Datenbank:** Kein SQL-Abfrage-Overhead. Sessions ablaufen lassen geht automatisch über TTL.

## 6. Persistenz-Optionen

Redis ist ein In-Memory-Speicher - bei einem Neustart gehen Daten verloren, sofern keine Persistenz konfiguriert ist. Es gibt zwei Methoden:

### RDB - Redis Database (Snapshots)

Redis speichert in regelmäßigen Abständen einen Snapshot aller Daten auf die Festplatte.

**Konfiguration als Docker-Command-Argument:**
```yaml
services:
  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    command: >
      redis-server
      --save 3600 1
      --save 300 100
      --save 60 10000
      --dir /data
      --dbfilename dump.rdb
```

**Vorteile:** Kompaktes Backup, schneller Neustart
**Nachteil:** Datenverlust zwischen zwei Snapshots möglich

**Snapshot manuell auslösen:**
```bash
docker exec redis redis-cli BGSAVE    # im Hintergrund speichern
docker exec redis redis-cli LASTSAVE  # Zeitstempel des letzten Snapshots
```

### AOF - Append Only File (Protokoll)

Redis schreibt jeden einzelnen Schreibbefehl in eine Logdatei. Beim Neustart wird diese Datei wiedergegeben.

**Konfiguration als Docker-Command-Argument:**
```yaml
services:
  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    command: >
      redis-server
      --appendonly yes
      --appendfilename appendonly.aof
      --appendfsync everysec
```

**Vorteile:** Minimaler Datenverlust (maximal 1 Sekunde)
**Nachteil:** Größere Datei, etwas langsamer als RDB

### Was verwenden?

| Anwendungsfall | Empfehlung |
|---|---|
| Nur Cache, Datenverlust egal | Keine Persistenz nötig |
| Sessions, Datenverlust von ~1s akzeptabel | [AOF](../glossar.md#aof-append-only-file) mit `everysec` |
| Maximale Datensicherheit | AOF mit `always` + [RDB](../glossar.md#rdb-redis-database-snapshot) als Backup |
| Schnelle Backups, selten Schreibzugriffe | Nur RDB |

**Persistenz deaktivieren (für reinen Cache):**
```yaml
command: redis-server --save "" --appendonly no
```

## 7. Konfiguration

Alle Redis-Optionen können als Command-Argumente in der `compose.yml` übergeben werden - ohne separate Konfigurationsdatei.

**Wichtige Einstellungen:**

```yaml
services:
  redis:
    image: redis:7-alpine
    restart: unless-stopped
    volumes:
      - redis_data:/data
    command: >
      redis-server
      --requirepass dein_sicheres_passwort
      --maxmemory 256mb
      --maxmemory-policy allkeys-lru
      --appendonly yes
      --appendfsync everysec

volumes:
  redis_data:
```

| Option | Bedeutung |
|---|---|
| `--requirepass` | Passwortschutz aktivieren |
| `--maxmemory 256mb` | Maximalen Speicher begrenzen |
| `--maxmemory-policy allkeys-lru` | Älteste Einträge verwerfen wenn Speicher voll (für Cache) |
| `--appendonly yes` | AOF-Persistenz aktivieren |
| `--save ""` | Alle RDB-Snapshots deaktivieren |

**Nach Konfigurationsänderungen neu starten:**
```bash
docker compose up -d --force-recreate redis
```

## 8. Verbindung aus einer Anwendung

Anwendungen verbinden sich mit Redis über eine Verbindungs-URL oder einzelne Parameter.

**Verbindungs-URL (Standard-Format):**
```
redis://localhost:6379
redis://:passwort@localhost:6379      # mit Passwort
redis://redis:6379                    # Docker-Servicename
redis://:passwort@redis:6379          # Docker mit Passwort
```

**Beispiele aus Self-Hosting-Apps:**
```bash
# Mastodon
REDIS_URL=redis://:passwort@redis:6379

# Nextcloud (in config.php)
'memcache.distributed' => '\OC\Memcache\Redis',
'redis' => ['host' => 'redis', 'port' => 6379, 'password' => 'passwort'],

# Gitea (in app.ini)
[cache]
ADAPTER = redis
HOST    = network=tcp,addr=redis:6379,db=0,password=passwort
```

**Vollständiges Compose-Beispiel mit App:**
```yaml
services:
  app:
    image: meine-app
    environment:
      REDIS_URL: redis://:dein_sicheres_passwort@redis:6379
    depends_on:
      - redis

  redis:
    image: redis:7-alpine
    restart: unless-stopped
    volumes:
      - redis_data:/data
    command: >
      redis-server
      --requirepass dein_sicheres_passwort
      --appendonly yes
      --appendfsync everysec

volumes:
  redis_data:
```

Der Hostname `redis` entspricht dem Service-Namen in der `compose.yml` - Docker Compose löst diesen automatisch auf.

## 9. Häufige Befehle im Überblick

| Befehl | Bedeutung |
|---|---|
| `docker exec redis redis-cli PING` | Redis-Verbindung testen |
| `docker exec redis redis-cli INFO server` | Server-Informationen |
| `docker exec redis redis-cli INFO memory` | Speichernutzung anzeigen |
| `docker exec redis redis-cli DBSIZE` | Anzahl der Schlüssel |
| `docker exec redis redis-cli FLUSHDB` | Alle Schlüssel löschen (Vorsicht!) |
| `docker exec redis redis-cli BGSAVE` | Snapshot im Hintergrund erstellen |
| `docker logs redis` | Container-Logs anzeigen |
| `docker compose restart redis` | Redis neu starten |

## 10. Troubleshooting

**Redis antwortet nicht:**
```bash
docker ps                          # läuft der Container?
docker logs redis                  # Fehlermeldungen anzeigen
docker exec redis redis-cli PING   # sollte PONG zurückgeben
```

**Verbindung aus anderem Container schlägt fehl:**
- Prüfen, ob beide Services in derselben `compose.yml` definiert sind
- Hostname muss dem Service-Namen entsprechen (`redis`, nicht `localhost`)
- Wenn Passwort gesetzt: `requirepass` und URL-Passwort müssen übereinstimmen

**Zu viel Speicher verbraucht:**
```bash
docker exec redis redis-cli INFO memory   # Speichernutzung prüfen
docker exec redis redis-cli DBSIZE        # Wie viele Schlüssel?
```
`--maxmemory` und `--maxmemory-policy` in der `compose.yml` setzen.

**Welche Redis-Version läuft?**
```bash
docker exec redis redis-server --version
docker exec redis redis-cli --version
```

**Daten persistent sichern (Backup):**
```bash
docker exec redis redis-cli BGSAVE
docker cp redis:/data/dump.rdb ./redis-backup.rdb
```
