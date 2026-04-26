# Exkurs: Datenbanken bei Nextcloud

## Was Nextcloud unterstützt

Nextcloud unterstützt vier Datenbanken:

| Datenbank  | Empfohlen für                | Nextcloud-Support        |
|------------|------------------------------|--------------------------|
| [PostgreSQL](../../wiki/glossar.md#postgresql) | Mittlere bis große Instanzen | ✅ Gut unterstützt       |
| [MariaDB](../../wiki/glossar.md#mariadb)    | Mittlere Instanzen           | ✅ Gut unterstützt       |
| [MySQL](../../wiki/glossar.md#mysql)      | Wie MariaDB                  | ✅ Gut unterstützt       |
| [SQLite](../../wiki/glossar.md#sqlite)     | Einzelpersonen, Tests        | ⚠️ Nicht für Echtbetrieb |

Die [offizielle Nextcloud-Dokumentation](https://docs.nextcloud.com/server/latest/admin_manual/configuration_database/linux_database_configuration.html) empfiehlt **PostgreSQL oder MariaDB** für alle Installationen, die mehr als eine Person oder mehr als ein paar Dateien umfassen.

---

## Warum SQLite für Nextcloud keine gute Idee ist

SQLite speichert die gesamte Datenbank in einer Datei. Das ist für viele kleine Dienste eine vernünftige Wahl, aber Nextcloud hat spezifische Probleme damit:

- **Gleichzeitige Schreibzugriffe:** Wenn mehrere Tabs, der Sync-Client und ein Hintergrundjob gleichzeitig auf die Datenbank zugreifen, sperrt SQLite die Datei. Das führt zu Fehlern und Timeouts.
- **Keine [Volltextsuche](../../wiki/glossar.md#fulltext-search-volltextsuche):** Nextcloud's Suche funktioniert mit SQLite eingeschränkt.
- **Keine Online-Backup-Möglichkeit:** PostgreSQL und MariaDB lassen sich ohne Downtime sichern.
- **Migrations-Probleme:** Nextcloud-Updates, die [Datenbankmigrationen](../../wiki/glossar.md#datenbank-migration) ausführen, können mit SQLite länger dauern oder fehlschlagen.

SQLite ist für einen schnellen Test in Ordnung. Für alles, was lange stabil laufen soll, ist es das falsche Werkzeug.

---

## PostgreSQL vs. MariaDB: Was wählen?

Für Nextcloud ist beides in der Praxis gut. Die Entscheidungsgrundlage:

**PostgreSQL wählen, wenn:**
- Die Installation auf Dauer und mit mehreren Nutzern betrieben wird
- Andere Dienste auf demselben Server ebenfalls PostgreSQL verwenden
- Man eine starke, zukunftssichere Wahl will

**MariaDB wählen, wenn:**
- Die App-Dokumentation explizit MariaDB nennt
- Du aus dem WordPress/LAMP-Umfeld kommst und bereits MariaDB kennst
- Du mehrere PHP-Anwendungen betreibst, die alle MariaDB nutzen

---

## Was AIO entscheidet

Wenn du AIO nutzt, hast du keine Wahl bei der Datenbank. AIO startet immer PostgreSQL.

Das ist eine bewusste Entscheidung der Nextcloud-Entwickler:
- PostgreSQL ist die interne Empfehlung
- Weniger Optionen bedeuten weniger Fehlerkonfiguration
- AIO ist für einfaches Deployment gedacht, nicht für maximale Flexibilität

Wer eine andere Datenbank will, muss das manuelle Docker-Compose-Setup nehmen. Für diesen Kurs ist AIO der richtige Weg.

---

## Redis: Kein Ersatz, sondern Ergänzung

AIO startet auch einen **[Redis](../../wiki/glossar.md#redis)-Container**. Redis ist keine Alternative zur Datenbank. Es ist ein Ergänzungsdienst:

- **Dateisperren:** Nextcloud nutzt Redis, um zu verhindern, dass zwei Clients dieselbe Datei gleichzeitig beschreiben. Das nennt sich "[Transactional File Locking](../../wiki/glossar.md#transactional-file-locking)".
- **Session-Cache:** Angemeldete Nutzer-Sessions werden in Redis gecacht, nicht jedes Mal aus der Datenbank gelesen.
- **Hintergrundaufgaben:** Manche Nextcloud-Jobs nutzen Redis als Warteschlange.

Ohne Redis ist Nextcloud zwar funktionsfähig, aber langsamer und anfälliger für Dateikorruption bei gleichzeitigen Zugriffen.

---

## Datenbankadministration

AIO bietet keinen direkten Zugang zur PostgreSQL-Datenbank über eine Web-UI.
Falls du direkt in die Datenbank schauen willst:

```bash
# Shell im Datenbank-Container öffnen
docker exec -it nextcloud-aio-database bash

# PostgreSQL-Konsole starten
psql -U nextcloud -d nextcloud
```

Für den normalen Betrieb ist das nicht nötig. Nextcloud verwaltet seine Datenbankstruktur selbst.

Falls du ein bisschen schönere Optik für die Datenbank haben willst, dann kannst du auch [**pgAdmin**](https://www.pgadmin.org/) als Container dazu installieren. Damit musst du nicht in der Datenbank über die Konsole arbeiten, sondern hast eine modernes Webinterface.

---

## Weiter geht es

→ [Plugins & Apps](04-plugins.md)