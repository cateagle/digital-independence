# 06 - Cron & geplante Aufgaben

Auf einem Server müssen viele Aufgaben regelmäßig und automatisch erledigt werden: Backups, Protokoll-Bereinigungen, Sicherheitsupdates, Datenbankwartung. Dafür gibt es unter Linux [**Cron**](../glossar.md#cron--crontab) — den eingebauten Aufgabenplaner.

**Windows-Analogie:** Cron ist wie der Windows Task-Planer (`taskschd.msc`), aber komplett textbasiert und viel verbreiteter auf Servern.

---

## Was ist Cron?

Cron ist ein Hintergrunddienst ([Daemon](../glossar.md#daemon)), der kontinuierlich läuft und zu festgelegten Zeiten Befehle ausführt. Die Zeitpläne werden in einer Textdatei namens **Crontab** (Cron Table) definiert.

Cron läuft automatisch beim Systemstart und braucht keine manuelle Aktivierung:

```bash
systemctl status cron
```

---

## Die Crontab bearbeiten

Jeder Benutzer hat eine eigene Crontab. Bearbeiten mit:

```bash
crontab -e
```

Beim ersten Aufruf wirst du nach einem Editor gefragt. Wähle `nano` (Option 1) — das ist am anfängerfreundlichsten.

Weitere nützliche Crontab-Befehle:

```bash
crontab -l          # Aktuelle Crontab anzeigen
crontab -r          # Crontab löschen (Vorsicht!)
crontab -e -u alice # Crontab eines anderen Benutzers bearbeiten (nur root)
```

---

## Crontab-Syntax

Jede Zeile in der Crontab definiert einen Job und folgt diesem Muster:

```
* * * * * /pfad/zum/befehl
│ │ │ │ │
│ │ │ │ └── Wochentag  (0–7, wobei 0 und 7 = Sonntag)
│ │ │ └──── Monat      (1–12)
│ │ └────── Tag         (1–31)
│ └──────── Stunde      (0–23)
└────────── Minute      (0–59)
```

### Die 5 Felder erklärt

| Feld       | Wertebereich | Bedeutung                        |
|------------|--------------|----------------------------------|
| Minute     | 0–59         | In welcher Minute der Stunde     |
| Stunde     | 0–23         | In welcher Stunde des Tages      |
| Tag        | 1–31         | An welchem Tag des Monats        |
| Monat      | 1–12         | In welchem Monat                 |
| Wochentag  | 0–7          | An welchem Wochentag (0=Sonntag) |

### Sonderzeichen

| Zeichen | Bedeutung                              | Beispiel           |
|---------|----------------------------------------|--------------------|
| `*`     | Jeder Wert (immer)                     | `* * * * *`        |
| `,`     | Liste von Werten                       | `0 9,17 * * *`     |
| `-`     | Bereich von Werten                     | `0 9-17 * * *`     |
| `/`     | Schrittweite                           | `*/15 * * * *`     |

### Praktische Beispiele

```bash
# Jeden Tag um 02:30 Uhr ein Backup ausführen
30 2 * * * /home/alice/backup.sh

# Jeden Montag um 08:00 Uhr
0 8 * * 1 /usr/local/bin/wochenbericht.sh

# Alle 15 Minuten
*/15 * * * * /usr/local/bin/status-check.sh

# Jeden ersten Tag im Monat um Mitternacht
0 0 1 * * /usr/local/bin/monatsbereinigung.sh

# Werktags (Montag–Freitag) um 07:00 Uhr
0 7 * * 1-5 /home/alice/arbeitstag-init.sh

# Mehrmals täglich: um 06:00 und 18:00 Uhr
0 6,18 * * * /usr/local/bin/sync.sh
```

**Tipp:** Die Website [crontab.guru](https://crontab.guru) übersetzt Cron-Ausdrücke in lesbare Sprache — sehr nützlich beim Lernen.

### Vordefinierte Kürzel

Cron kennt auch einige bequeme Abkürzungen:

```bash
@reboot    # Einmal beim Systemstart
@hourly    # Jede Stunde (entspricht: 0 * * * *)
@daily     # Täglich um Mitternacht (entspricht: 0 0 * * *)
@weekly    # Wöchentlich (entspricht: 0 0 * * 0)
@monthly   # Monatlich (entspricht: 0 0 1 * *)
```

Beispiel:

```bash
# Skript beim Systemstart ausführen
@reboot /home/alice/start-dienst.sh
```

---

## System-Cron vs. Benutzer-Cron

Unter Linux gibt es zwei Arten von Cron-Jobs:

### Benutzer-Cron (`crontab -e`)

- Gehört einem bestimmten Benutzer
- Läuft mit den Rechten dieses Benutzers
- Gespeichert in `/var/spool/cron/crontabs/<benutzername>`
- Verwaltet mit `crontab -e`

Beispiel — als Benutzer `alice`:

```bash
crontab -e
```

```
30 2 * * * /home/alice/backup.sh
```

Dieser Job läuft als `alice` und hat nur deren Berechtigungen.

### System-Cron (`/etc/cron.d/` und `/etc/crontab`)

- Systemweite Jobs, meist als `root` oder ein bestimmter Systembenutzer
- Die Syntax hat ein **zusätzliches Feld** für den Benutzernamen:

```
# Minute Stunde Tag Monat Wochentag Benutzer Befehl
30 3 * * *   root   /usr/sbin/logrotate /etc/logrotate.conf
```

Systemweite Crontab-Dateien befinden sich hier:

```
/etc/crontab              # Haupt-System-Crontab
/etc/cron.d/              # Einzelne Dateien für Dienste
/etc/cron.hourly/         # Skripte, die stündlich laufen
/etc/cron.daily/          # Skripte, die täglich laufen
/etc/cron.weekly/         # Skripte, die wöchentlich laufen
/etc/cron.monthly/        # Skripte, die monatlich laufen
```

**Tipp:** Wenn du ein Skript täglich ausführen willst, kannst du es einfach in `/etc/cron.daily/` ablegen — Cron führt es automatisch aus (das Skript muss ausführbar sein):

```bash
sudo cp mein-skript.sh /etc/cron.daily/
sudo chmod +x /etc/cron.daily/mein-skript.sh
```

### Wann was verwenden?

| Situation                                    | Empfehlung                  |
|----------------------------------------------|-----------------------------|
| Persönliche Aufgaben eines Benutzers         | `crontab -e` als dieser Benutzer |
| Systemwartung (Backups, Updates)             | `/etc/cron.d/` als root     |
| Einfacher Tagesrhythmus ohne genaue Uhrzeit  | `/etc/cron.daily/`          |
| Dienst-spezifische Jobs                      | `/etc/cron.d/<dienstname>`  |

---

## Protokollierung von Cron-Jobs

### Cron-Aktivitäten im Systemlog prüfen

Cron schreibt seine Aktivitäten ins Systemprotokoll. Unter Ubuntu 24.04 LTS:

```bash
# Alle Cron-Einträge anzeigen
grep CRON /var/log/syslog

# Live mitverfolgen
sudo tail -f /var/log/syslog | grep CRON
```

Beispielausgabe:

```
Mar  1 02:30:01 server CRON[12345]: (alice) CMD (/home/alice/backup.sh)
Mar  1 02:30:02 server CRON[12345]: (CRON) info (No MTA installed, discarding output)
```

### Ausgabe eines Cron-Jobs in eine Datei umleiten

Standardmäßig wird die Ausgabe eines Cron-Jobs per E-Mail gesendet (was auf den meisten Servern nicht konfiguriert ist und stillschweigend verschwindet). Besser: direkt in eine Datei umleiten.

```bash
# Nur Ausgabe (stdout) speichern
30 2 * * * /home/alice/backup.sh >> /var/log/backup.log 2>&1

# Ausgabe mit Zeitstempel speichern
30 2 * * * echo "$(date): Start" >> /var/log/backup.log && /home/alice/backup.sh >> /var/log/backup.log 2>&1
```

Erklärung:
- `>>` — Ausgabe anhängen (nicht überschreiben)
- `2>&1` — Fehlermeldungen (stderr) ebenfalls in die Datei schreiben

### Ausgabe vollständig unterdrücken

Wenn ein Job keine Ausgabe erzeugen soll:

```bash
30 2 * * * /home/alice/backup.sh > /dev/null 2>&1
```

`/dev/null` ist das "schwarze Loch" unter Linux — alles, was dorthin gesendet wird, verschwindet.

### Mit journalctl prüfen

```bash
# Cron-Einträge über journalctl anzeigen
sudo journalctl -u cron --since "1 hour ago"

# Alle heutigen Cron-Einträge
sudo journalctl -u cron --since today
```

---

## Häufige Fehler

### 1. Falscher Pfad zum Befehl

Cron hat eine minimale Umgebung und kennt deinen `$PATH` nicht. Was im Terminal funktioniert, kann in Cron scheitern.

**Falsch:**
```
30 2 * * * backup.sh
```

**Richtig:** Immer absolute Pfade verwenden:
```
30 2 * * * /home/alice/backup.sh
```

Oder den vollen Pfad zum Programm angeben:
```
30 2 * * * /usr/bin/python3 /home/alice/backup.py
```

Den Pfad eines Programms herausfinden:
```bash
which python3
# Ausgabe: /usr/bin/python3
```

### 2. Skript nicht ausführbar

Das Skript muss ausführbar sein, sonst schlägt Cron still fehl:

```bash
chmod +x /home/alice/backup.sh
```

### 3. Umgebungsvariablen fehlen

Cron startet mit einer minimalen Umgebung. Variablen wie `HOME`, `USER` oder benutzerdefinierte `PATH`-Einträge sind möglicherweise nicht gesetzt.

**Lösung:** Variablen direkt in der Crontab definieren:

```
SHELL=/bin/bash
PATH=/usr/local/sbin:/usr/local/bin:/sbin:/bin:/usr/sbin:/usr/bin
HOME=/home/alice

30 2 * * * /home/alice/backup.sh
```

### 4. Ausgabe verschwindet spurlos

Wenn kein E-Mail-Server konfiguriert ist, verschwindet die Ausgabe. Deshalb immer in eine Datei umleiten (siehe Abschnitt Protokollierung).

### 5. Syntax-Fehler in der Crontab

Eine ungültige Crontab wird von Cron ignoriert. Nach dem Bearbeiten mit `crontab -e` gibt nano einen Hinweis bei Problemen. Zusätzlich prüfen:

```bash
crontab -l
```

Wenn der Job nicht auftaucht oder die Zeile fehlt, wurde sie nicht gespeichert.

### 6. Job läuft zur falschen Zeit

Cron verwendet die **Serverzeit**, nicht die lokale Zeit. Serverzeit prüfen:

```bash
date
timedatectl
```

Zeitzone anpassen falls nötig:

```bash
sudo timedatectl set-timezone Europe/Berlin
```

### 7. Relative Pfade im Skript

Auch innerhalb des Skripts selbst auf absolute Pfade achten. Das Arbeitsverzeichnis in Cron ist oft `/` oder das Home-Verzeichnis, nicht das Verzeichnis des Skripts.

**Im Skript am Anfang eintragen:**

```bash
#!/bin/bash
cd /home/alice/projekte || exit 1
```

---

## Schnelle Referenz

```bash
crontab -e              # Eigene Crontab bearbeiten
crontab -l              # Eigene Crontab anzeigen
crontab -r              # Eigene Crontab löschen
sudo crontab -e -u bob  # Crontab von Benutzer bob bearbeiten

grep CRON /var/log/syslog           # Cron-Aktivität im Log
sudo journalctl -u cron --since today  # Cron-Journal
```

---

## Trainingsübungen

1. Erstelle ein einfaches Skript `/home/<dein-name>/hallo.sh`, das die aktuelle Uhrzeit in `/tmp/hallo.log` schreibt.
2. Trage den Job in deine Crontab ein, sodass er jede Minute läuft.
3. Warte zwei Minuten und prüfe `/tmp/hallo.log` — stehen Einträge drin?
4. Prüfe die Cron-Aktivität mit `grep CRON /var/log/syslog`.
5. Ändere den Job so, dass er nur um 08:00 Uhr morgens läuft.
6. Lösche den Job wieder aus der Crontab.

---

## Weitere Ressourcen

- [crontab.guru](https://crontab.guru) — Cron-Ausdrücke interaktiv testen
- [Ubuntu Cron Dokumentation](https://help.ubuntu.com/community/CronHowto)
- `man 5 crontab` — Offizielle Manpage für die Crontab-Syntax
- `man 8 cron` — Manpage für den Cron-Daemon
