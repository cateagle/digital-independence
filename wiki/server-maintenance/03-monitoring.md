# Monitoring - Spickzettel

## 1. Was überwacht man und warum?

Monitoring bedeutet: den Gesundheitszustand deines Servers im Blick behalten, bevor Probleme zu Ausfällen werden.

**Windows-Analogie:** Der Task-Manager zeigt CPU, RAM und laufende Prozesse. Auf einem Linux-Server gibt es ähnliche Tools – aber in der Kommandozeile und mit mehr Detail.

**Was interessiert uns:**
| Ressource | Warum wichtig |
|-----------|---------------|
| CPU-Auslastung | Hohe Last = System überlastet oder Prozess hängt |
| RAM-Verbrauch | Zu wenig = System beginnt zu swappen (langsamer) |
| Festplattenplatz | Voll = Dienste schlagen fehl, Logs können nicht geschrieben werden |
| Netzwerk | Ungewöhnlicher Traffic = möglicher Angriff |
| Dienste | Dienst gestoppt = Anwendung nicht erreichbar |
| Logs | Fehlermeldungen = Hinweise auf Probleme |

## 2. CPU, RAM und Systemlast mit htop

`htop` ist eine interaktive Prozessübersicht – wie der Task-Manager unter Windows, aber im Terminal.

**Installation:**
```bash
sudo apt install htop
```

**Starten:**
```bash
htop
```

**Wichtige Bereiche in htop:**
```
  CPU [|||||||||||          30%]    ← CPU-Auslastung pro Kern
  Mem [||||||||||||||||    1.2G/4G] ← RAM-Verbrauch
  Swp [                    0K/2G]   ← Swap-Nutzung (sollte 0 sein)

  Load average: 0.15 0.22 0.18      ← Systemlast (1/5/15 Minuten)
```

**htop-Tastenkürzel:**
| Taste | Funktion |
|-------|----------|
| `F6` oder `>` | Sortierung ändern (CPU, RAM, etc.) |
| `F4` | Prozesse filtern |
| `F9` | Prozess beenden (Signal senden) |
| `u` | Nach Benutzer filtern |
| `q` | Beenden |

**Ohne htop – schnelle CPU-Übersicht:**
```bash
top
```

**Systemlast ohne interaktives Tool:**
```bash
uptime
```
Ausgabe:
```
10:32:45 up 42 days, 3:12,  1 user,  load average: 0.08, 0.12, 0.10
```
- "up 42 days" – Server läuft seit 42 Tagen ohne Neustart
- Load average: Systemlast der letzten 1, 5 und 15 Minuten
- Faustregel: Load average sollte unter der Anzahl der CPU-Kerne liegen

**Anzahl CPU-Kerne prüfen:**
```bash
nproc
```

## 3. Festplattenplatz überwachen mit df und du

**Übersicht aller Laufwerke:**
```bash
df -h
```
Ausgabe:
```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1        50G   18G   30G  38% /
/dev/sdb1       200G  120G   72G  63% /mnt/data
tmpfs           2.0G     0  2.0G   0% /dev/shm
```

**Warnstufen:**
- Bis 70%: Alles gut
- 70–85%: Aufmerksamkeit – wann läuft es voll?
- Über 85%: Handlungsbedarf
- 100%: Kritisch – Dienste können abstürzen

**Welches Verzeichnis frisst den meisten Platz?**
```bash
du -sh /var/* | sort -rh | head -10
```

**Speicherplatz eines einzelnen Verzeichnisses:**
```bash
du -sh /var/log/
```

**[Docker](../glossar.md#docker)-spezifisch: Wie viel Platz verbraucht Docker?**
```bash
docker system df
```

## 4. RAM-Verbrauch mit free

```bash
free -h
```
Ausgabe:
```
               total        used        free      shared  buff/cache   available
Mem:           3.8Gi       1.2Gi       500Mi       120Mi       2.1Gi       2.3Gi
Swap:          2.0Gi          0B       2.0Gi
```

**Wichtige Spalten:**
- `total`: Gesamt-RAM
- `used`: Tatsächlich genutzter RAM
- `available`: Verfügbar für neue Prozesse (inklusive Cache)
- `Swap`: Festplattenbereich als RAM-Ersatz (langsam – sollte 0 sein)

**Tipp:** Die `available`-Spalte ist relevanter als `free`. Linux nutzt freien RAM als Cache – das ist gewollt und kein Problem.

## 5. Dienste überwachen mit systemctl

**Status eines Dienstes prüfen:**
```bash
sudo systemctl status nginx
```
Ausgabe:
```
● nginx.service - A high performance web server and a reverse proxy server
     Loaded: loaded (/lib/systemd/system/nginx.service; enabled; vendor preset: enabled)
     Active: active (running) since Mon 2024-01-15 10:00:00 UTC; 2 days ago
```

**Wichtige Status-Zeilen:**
| Status | Bedeutung |
|--------|-----------|
| `active (running)` | Dienst läuft |
| `inactive (dead)` | Dienst gestoppt |
| `failed` | Dienst abgestürzt |
| `enabled` | Wird beim Systemstart gestartet |
| `disabled` | Wird beim Systemstart nicht gestartet |

**Alle laufenden Dienste anzeigen:**
```bash
sudo systemctl list-units --type=service --state=running
```

**Fehlgeschlagene Dienste anzeigen:**
```bash
sudo systemctl list-units --type=service --state=failed
```

**Dienst neu starten:**
```bash
sudo systemctl restart nginx
```

**Dienst-Logs der letzten 50 Zeilen:**
```bash
sudo journalctl -u nginx -n 50
```

## 6. Logs mit journalctl lesen

[`journalctl`](../glossar.md#journald--journalctl) ist das zentrale Log-System unter Ubuntu ([systemd](../glossar.md#systemd)-basiert).

**Windows-Analogie:** Die Windows-Ereignisanzeige (Event Viewer) zeigt System- und Anwendungslogs. `journalctl` macht dasselbe in der Kommandozeile.

**Alle Logs anzeigen (neueste zuerst):**
```bash
sudo journalctl -r
```

**Logs eines bestimmten Dienstes:**
```bash
sudo journalctl -u nginx
sudo journalctl -u postgresql
sudo journalctl -u docker
```

**Logs der letzten Stunde:**
```bash
sudo journalctl --since "1 hour ago"
```

**Logs seit gestern:**
```bash
sudo journalctl --since yesterday
```

**Logs live verfolgen (wie tail -f):**
```bash
sudo journalctl -f
# Nur für einen bestimmten Dienst:
sudo journalctl -u nginx -f
```

**Nur Fehler anzeigen:**
```bash
sudo journalctl -p err
```

**Log-Level-Filter:**
| Level | Bedeutung |
|-------|-----------|
| `emerg` | System ist unbenutzbar |
| `alert` | Sofortige Aktion nötig |
| `crit` | Kritischer Zustand |
| `err` | Fehler |
| `warning` | Warnungen |
| `info` | Informationen |
| `debug` | Debug-Informationen |

**Wie viel Speicher verbrauchen die Logs?**
```bash
sudo journalctl --disk-usage
```

**Alte Logs aufräumen:**
```bash
sudo journalctl --vacuum-time=30d
```

## 7. Netzwerk-Übersicht

**Aktive Netzwerkverbindungen und offene Ports:**
```bash
ss -tulpn
```
Ausgabe (Beispiel):
```
Netid  State   Local Address:Port  Process
tcp    LISTEN  0.0.0.0:80          nginx
tcp    LISTEN  0.0.0.0:443         nginx
tcp    LISTEN  127.0.0.1:5432      postgres
```

**Offene Ports prüfen (nützlich für Sicherheits-Audit):**
```bash
sudo ss -tulpn | grep LISTEN
```

**Netzwerkauslastung live:**
```bash
sudo apt install nload
nload
```

## 8. Einfaches Monitoring-Skript

Ein einfaches Skript das täglich eine Übersicht per Log schreibt:

```bash
#!/bin/bash
# /usr/local/bin/system-check.sh

LOG="/var/log/system-check.log"
DATUM=$(date '+%Y-%m-%d %H:%M:%S')

echo "=== System-Check: $DATUM ===" >> "$LOG"

# CPU-Last
echo "Load average: $(uptime | awk -F'load average:' '{print $2}')" >> "$LOG"

# RAM
echo "RAM: $(free -h | awk '/^Mem:/ {print $3 " von " $2 " genutzt"}')" >> "$LOG"

# Festplatte
echo "Festplatte:" >> "$LOG"
df -h / /mnt/data 2>/dev/null >> "$LOG"

# Fehlgeschlagene Dienste
FAILED=$(systemctl list-units --state=failed --no-legend | wc -l)
echo "Fehlgeschlagene Dienste: $FAILED" >> "$LOG"

# Festplatte kritisch voll?
DISK_USE=$(df / | awk 'NR==2 {print $5}' | tr -d '%')
if [ "$DISK_USE" -gt 85 ]; then
    echo "WARNUNG: Festplatte zu ${DISK_USE}% voll!" >> "$LOG"
fi

echo "" >> "$LOG"
```

## 9. Prometheus & Grafana: Der nächste Schritt

Die bisher gezeigten Shell-Tools eignen sich gut für manuelle Checks und einfache Skripte. Für dauerhaftes, professionelles Monitoring gibt es bessere Lösungen:

**Prometheus** sammelt Metriken (CPU, RAM, Festplatte, Anfragen pro Sekunde, etc.) von Servern und Anwendungen und speichert sie in einer Zeitreihendatenbank.

**Grafana** visualisiert diese Metriken in übersichtlichen Dashboards mit Graphen und kann Alarme auslösen.

**Typischer Stack:**
```
Server → node_exporter → Prometheus → Grafana
         (Metriken        (Sammeln     (Anzeigen,
          bereitstellen)   + Speichern)  Alarme)
```

**Warum noch nicht jetzt?**
- Für einen einzelnen Server sind Shell-Tools ausreichend
- Prometheus/Grafana sind sinnvoll ab mehreren Servern oder wenn du auf Dashboards angewiesen bist
- Sie erfordern eigene Wartung und Ressourcen

**Wann Prometheus/Grafana einführen:**
- Du betreibst 3 oder mehr Server
- Du willst historische Daten und Trends sehen
- Du brauchst automatische Benachrichtigungen (z.B. "Festplatte zu 90% voll")
- Du hast Anwendungen die Metriken exportieren (Datenbankabfragen, API-Antwortzeiten)

## 10. Nützliche Befehle auf einen Blick

| Befehl | Was es macht |
|--------|--------------|
| `htop` | Interaktive Prozessübersicht |
| `uptime` | Systemlast und Laufzeit |
| `df -h` | Festplattenplatz aller Laufwerke |
| `du -sh /pfad/` | Platzbedarf eines Verzeichnisses |
| `free -h` | RAM-Verbrauch |
| `sudo systemctl status dienst` | Status eines Dienstes |
| `sudo systemctl list-units --state=failed` | Fehlgeschlagene Dienste |
| `sudo journalctl -u dienst -f` | Logs live verfolgen |
| `sudo journalctl -p err` | Nur Fehler anzeigen |
| `ss -tulpn` | Offene Ports und Verbindungen |
| `docker system df` | Docker-Speicherverbrauch |

## 11. Troubleshooting: "Was ist mit meinem Server los?"

**Vorgehen bei unklaren Problemen:**

```bash
# 1. Ist der Server erreichbar?
uptime

# 2. Gibt es fehlgeschlagene Dienste?
sudo systemctl list-units --state=failed

# 3. Ist die Festplatte voll?
df -h

# 4. Ist der RAM voll?
free -h

# 5. Was lief in der letzten Stunde schief?
sudo journalctl --since "1 hour ago" -p err

# 6. Welche Prozesse fressen Ressourcen?
htop
# oder ohne htop: nach CPU sortiert
ps aux --sort=-%cpu | head -10
# nach RAM sortiert
ps aux --sort=-%mem | head -10
```
