# Datenbanken im Überblick - Spickzettel

## 1. Warum brauche ich eine Datenbank?

Jede Anwendung, die Daten dauerhaft speichern muss, braucht eine Datenbank. Beim [Self-Hosting](../glossar.md#self-hosting) wählst du die Datenbank selbst und die Wahl hängt vom Anwendungsfall ab, nicht von Gewohnheit.

Die vier häufigsten Kandidaten beim Self-Hosting:
- **[PostgreSQL](../glossar.md#postgresql)**: robuste, vollwertige relationale Datenbank
- **[MariaDB](../glossar.md#mariadb) / [MySQL](../glossar.md#mysql)**: weit verbreitet, gut unterstützt
- **[SQLite](../glossar.md#sqlite)**: einfach, dateibasiert, kein Server nötig
- **[Redis](../glossar.md#redis)**: In-Memory-Speicher für schnelle Zugriffe

## 2. Schnell-Entscheidungshilfe

| Frage                                                | Empfehlung              |
|------------------------------------------------------|-------------------------|
| Ich brauche einfach eine SQL-Datenbank für meine App | PostgreSQL              |
| Die App-Dokumentation sagt "MySQL"                   | MariaDB (kompatibel)    |
| Kleine App, kein hoher Traffic, einfach halten       | SQLite                  |
| Ich brauche einen Cache oder Session-Speicher        | Redis                   |
| Die App hat viele gleichzeitige Schreibzugriffe      | PostgreSQL              |
| Ich will nichts konfigurieren                        | SQLite                  |
| Die App läuft in Docker und braucht eine eigene DB   | PostgreSQL oder MariaDB |

## 3. PostgreSQL - Wann verwenden?

PostgreSQL ist die sichere Standardwahl für die meisten Self-Hosting-Projekte.

**Verwende PostgreSQL wenn:**
- Du eine Hauptdatenbank für eine Webanwendung brauchst
- Die App explizit PostgreSQL unterstützt (z.B. Gitea, Nextcloud, Authentik)
- Du komplexe Abfragen oder viele gleichzeitige Nutzer erwartest
- Du JSON-Daten in einer SQL-Datenbank speichern willst

**Beispiele aus der Praxis:**
- Gitea, Forgejo (Git-Hosting)
- Nextcloud (optional, empfohlen für größere Instanzen)
- Authentik, Keycloak (Identity Management)
- Mastodon, Misskey (Social Media)

## 4. MariaDB - Wann verwenden?

MariaDB ist ein [Fork](../glossar.md#fork) von MySQL und zu großen Teilen kompatibel. Wenn eine App "MySQL" verlangt, funktioniert MariaDB meist genauso.

**Verwende MariaDB wenn:**
- Die App-Dokumentation MySQL/MariaDB vorschreibt
- Du aus der WordPress/LAMP-Welt kommst
- Die App explizit keine PostgreSQL-Unterstützung hat

**Beispiele aus der Praxis:**
- WordPress
- Viele PHP-Anwendungen
- Nextcloud (ebenfalls möglich)

## 5. SQLite - Wann verwenden?

SQLite speichert die gesamte Datenbank in einer einzigen Datei. Kein Datenbankserver, keine Konfiguration.

**Verwende SQLite wenn:**
- Die Anwendung wenig Traffic hat
- Du einfachstes Setup bevorzugst
- Du ein Tool oder einen kleinen Dienst für den Eigenbedarf betreibst
- Backups sollen einfach sein (einfach die .db-Datei kopieren)

**Verwende SQLite NICHT wenn:**
- Mehrere Prozesse gleichzeitig schreiben
- Hoher Traffic erwartet wird
- Die Datenbank über das Netzwerk erreichbar sein soll

**Beispiele aus der Praxis:**
- [Vaultwarden](../glossar.md#vaultwarden) (Bitwarden-kompatibles Password-Manager-Backend)
- Kleine Monitoring-Tools
- Heimautomatisierung (z.B. Home Assistant)

## 6. Redis - Wann verwenden?

Redis ist keine klassische Datenbank, sondern ein In-Memory-Speicher. Daten liegen im RAM. Zugriffe sind extrem schnell.

**Verwende Redis wenn:**
- Du einen Cache brauchst (z.B. Ergebnisse von DB-Abfragen zwischenspeichern)
- Sessions für eine Webanwendung gespeichert werden sollen
- Du eine Message Queue oder einen [Pub/Sub](../glossar.md#pubsub-publishsubscribe)-Mechanismus brauchst
- Daten nur kurzfristig gespeichert werden müssen

**Redis ist kein Ersatz für PostgreSQL oder MariaDB**. Es wird meist als Ergänzung daneben betrieben.

**Beispiele aus der Praxis:**
- [Mastodon](../glossar.md#mastodon) (Cache und Sidekiq-Queues)
- Nextcloud (Session-Cache)
- Gitea (Cache)

## 7. Typische Kombinationen beim Self-Hosting

```
Anwendung → PostgreSQL (Hauptdaten) + Redis (Cache/Sessions)
```

Das ist das häufigste Muster bei größeren Self-Hosting-Projekten.

```
Kleine Anwendung → SQLite (alles in einer Datei)
```

Ideal für Tools mit geringem Traffic oder Einzelpersonennutzung.

```
WordPress/PHP-App → MariaDB
```

Der klassische [LAMP-Stack](../glossar.md#lamp-stack).

## 8. Faustregel für Einsteiger

1. Schau in der Dokumentation deiner App nach, welche Datenbank empfohlen wird.
2. Wenn PostgreSQL und MariaDB beide möglich sind, wähle **PostgreSQL**.
3. Wenn die App SQLite unterstützt und der Traffic gering ist, nutze **SQLite** für weniger Aufwand.
4. Wenn die App Redis erwähnt, installiere es zusätzlich. Es ersetzt keine SQL-Datenbank.
