# Backups - Spickzettel

## 1. Warum Backups unbedingt notwendig sind

Festplatten sterben. Server werden gehackt. Dateien werden aus Versehen gelöscht. Ohne Backup sind alle Daten weg.

**Windows-Analogie:** Die Windows-Funktion "Dateiversionsverlauf" ist ein einfaches Backup-System. Auf einem Linux-Server bist du selbst dafür verantwortlich.

**Grundregel:** Ein Backup, das nicht getestet wurde, ist kein Backup.

## 2. Die 3-2-1-Regel

Die wichtigste Backup-Strategie überhaupt:

```
3 Kopien der Daten
2 verschiedene Speichermedien
1 Kopie an einem anderen Ort (off-site)
```

**Konkret für [Self-Hosting](../glossar.md#self-hosting):**
- Kopie 1: Produktionsdaten auf dem Server
- Kopie 2: Backup auf einer externen Festplatte oder zweitem Laufwerk
- Kopie 3: Backup in der Cloud (z.B. Hetzner Storage Box, Backblaze B2)

## 3. rsync für Datei-Backups

`rsync` synchronisiert Verzeichnisse effizient – nur geänderte Dateien werden übertragen.

**Lokales Backup:**
```bash
rsync -av /home/alice/ /mnt/backup/alice/
```

**Backup auf entfernten Server:**
```bash
rsync -av -e ssh /var/www/html/ backupuser@backup-server:/backups/html/
```

**Wichtige Optionen:**
| Option | Bedeutung |
|--------|-----------|
| `-a`   | Archiv-Modus (Berechtigungen, Zeitstempel erhalten) |
| `-v`   | Ausführliche Ausgabe |
| `-z`   | Komprimierung beim Transfer |
| `--delete` | Gelöschte Dateien auch im Backup entfernen |
| `--dry-run` | Simulation – nichts wird wirklich geändert |

**Empfehlung: Immer zuerst mit `--dry-run` testen:**
```bash
rsync -av --dry-run /var/data/ /mnt/backup/data/
```

## 4. tar für Archivierung

`tar` packt Verzeichnisse in eine einzige Archivdatei, optional komprimiert.

**Archiv erstellen:**
```bash
tar -czf backup-2024-01-15.tar.gz /var/www/html/
```

**Archiv entpacken:**
```bash
tar -xzf backup-2024-01-15.tar.gz -C /restore/
```

**Archivinhalt anzeigen (ohne zu entpacken):**
```bash
tar -tzf backup-2024-01-15.tar.gz
```

**Optionen erklärt:**
| Option | Bedeutung |
|--------|-----------|
| `-c`   | Erstelle Archiv |
| `-x`   | Entpacke Archiv |
| `-z`   | gzip-Komprimierung |
| `-f`   | Dateiname folgt |
| `-t`   | Archivinhalt auflisten |
| `-C`   | Zielverzeichnis beim Entpacken |

**Tipp:** Datum im Dateinamen hilft, Backups zu organisieren:
```bash
tar -czf "backup-$(date +%Y-%m-%d).tar.gz" /var/data/
```

## 5. PostgreSQL-Datenbank sichern (pg_dump)

Dateisystem-Backups einer laufenden Datenbank sind unzuverlässig – die Datenbankdateien können inkonsistent sein. Verwende stattdessen [`pg_dump`](../glossar.md#pg_dump).

**Einzelne Datenbank sichern:**
```bash
sudo -u postgres pg_dump meine_datenbank > backup-db-$(date +%Y-%m-%d).sql
```

**Alle Datenbanken sichern:**
```bash
sudo -u postgres pg_dumpall > backup-alle-dbs-$(date +%Y-%m-%d).sql
```

**Komprimiertes Backup:**
```bash
sudo -u postgres pg_dump meine_datenbank | gzip > backup-db-$(date +%Y-%m-%d).sql.gz
```

**Wiederherstellung:**
```bash
sudo -u postgres psql meine_datenbank < backup-db-2024-01-15.sql
```

**In Docker-Container:**
```bash
docker exec postgres-container pg_dump -U postgres meine_datenbank > backup.sql
```

## 6. Docker-Volume-Backups

[Docker](../glossar.md#docker)-Volumes enthalten die persistenten Daten deiner Container (Datenbanken, Uploads, Konfigurationen).

**Volume-Inhalt in tar-Archiv sichern:**
```bash
docker run --rm \
  -v mein_volume:/data \
  -v /mnt/backup:/backup \
  alpine tar -czf /backup/volume-backup-$(date +%Y-%m-%d).tar.gz -C /data .
```

**Was passiert hier:**
1. Ein temporärer Alpine-Container wird gestartet
2. Das Volume wird als `/data` eingehängt
3. Das Backup-Verzeichnis des Hosts wird als `/backup` eingehängt
4. `tar` erstellt das Archiv
5. Der Container wird danach automatisch gelöscht (`--rm`)

**Volume-Backup wiederherstellen:**
```bash
docker run --rm \
  -v mein_volume:/data \
  -v /mnt/backup:/backup \
  alpine tar -xzf /backup/volume-backup-2024-01-15.tar.gz -C /data
```

## 7. Backup-Skript mit cron automatisieren

**Backup-Skript erstellen (`/usr/local/bin/backup.sh`):**
```bash
#!/bin/bash
set -euo pipefail

DATUM=$(date +%Y-%m-%d)
BACKUP_DIR="/mnt/backup/${DATUM}"
LOG_FILE="/var/log/backup.log"

echo "[$(date)] Backup gestartet" >> "$LOG_FILE"

# Verzeichnis erstellen
mkdir -p "$BACKUP_DIR"

# Dateien sichern
rsync -av /var/www/html/ "$BACKUP_DIR/html/" >> "$LOG_FILE" 2>&1

# PostgreSQL sichern
sudo -u postgres pg_dump meine_datenbank | gzip > "$BACKUP_DIR/db.sql.gz"

# Alte Backups löschen (älter als 30 Tage)
find /mnt/backup/ -type d -mtime +30 -exec rm -rf {} + 2>/dev/null || true

echo "[$(date)] Backup abgeschlossen" >> "$LOG_FILE"
```

**Skript ausführbar machen:**
```bash
chmod +x /usr/local/bin/backup.sh
```

**Skript manuell testen:**
```bash
sudo /usr/local/bin/backup.sh
```

**Mit [Cron](../glossar.md#cron--crontab) täglich um 3 Uhr nachts ausführen:**
```bash
sudo crontab -e
```

Folgende Zeile hinzufügen:
```
0 3 * * * /usr/local/bin/backup.sh
```

**Cron-Syntax:**
```
┌───────────── Minute (0–59)
│ ┌─────────── Stunde (0–23)
│ │ ┌───────── Tag des Monats (1–31)
│ │ │ ┌─────── Monat (1–12)
│ │ │ │ ┌───── Wochentag (0–7, 0 und 7 = Sonntag)
│ │ │ │ │
0 3 * * * /usr/local/bin/backup.sh
```

## 8. Wiederherstellung testen

**Ein Backup ist wertlos, wenn die Wiederherstellung nicht funktioniert.**

**Checkliste für Restore-Tests:**
```bash
# 1. Backup-Datei prüfen (ist sie nicht leer/beschädigt?)
ls -lh /mnt/backup/2024-01-15/
tar -tzf /mnt/backup/2024-01-15/html.tar.gz | head -20

# 2. Auf einem Test-System wiederherstellen
rsync -av /mnt/backup/2024-01-15/html/ /tmp/restore-test/

# 3. Inhalt prüfen
ls -la /tmp/restore-test/

# 4. Datenbank-Restore testen
sudo -u postgres createdb test_restore
sudo -u postgres psql test_restore < /mnt/backup/2024-01-15/db.sql
sudo -u postgres psql test_restore -c "SELECT COUNT(*) FROM wichtige_tabelle;"

# 5. Test-Datenbank aufräumen
sudo -u postgres dropdb test_restore
```

**Empfehlung:** Führe einmal im Monat einen vollständigen Restore-Test durch und dokumentiere das Ergebnis.

## 9. Nützliche Befehle auf einen Blick

| Befehl | Was es macht |
|--------|--------------|
| `rsync -av /quelle/ /ziel/` | Dateien synchronisieren |
| `tar -czf archiv.tar.gz /verz/` | Archiv erstellen |
| `tar -xzf archiv.tar.gz -C /ziel/` | Archiv entpacken |
| `pg_dump db > backup.sql` | PostgreSQL-Datenbank sichern |
| `psql db < backup.sql` | Datenbank wiederherstellen |
| `find /backup/ -mtime +30 -delete` | Alte Backups löschen |
| `crontab -l` | Cron-Jobs anzeigen |
| `crontab -e` | Cron-Jobs bearbeiten |

## 10. Troubleshooting

**rsync schlägt fehl wegen Berechtigungen:**
```bash
# Mit sudo ausführen oder Berechtigungen prüfen
sudo rsync -av /var/data/ /mnt/backup/data/
```

**pg_dump: "Verbindung abgelehnt":**
```bash
# PostgreSQL läuft? Status prüfen
sudo systemctl status postgresql
```

**Backup-Verzeichnis voll:**
```bash
# Speicherplatz prüfen
df -h /mnt/backup/

# Größte Verzeichnisse finden
du -sh /mnt/backup/*/ | sort -rh | head -10
```

**Cron-Job läuft nicht:**
```bash
# Cron-Log prüfen
grep CRON /var/log/syslog | tail -20
```
