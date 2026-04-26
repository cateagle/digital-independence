# 05 - Prozesse & Systemüberwachung

Wenn du weißt, wie du laufende Programme beobachten und steuern kannst, verlierst du nie die Kontrolle über deinen Server. Dieses Kapitel zeigt dir, wie du siehst, was auf deinem System passiert – und wie du eingreifst, wenn etwas aus dem Ruder läuft.

**Windows-Analogie:** Vieles davon entspricht dem Task-Manager (`Strg+Shift+Esc`), aber über die Kommandozeile – und deutlich mächtiger.

---

## Was ist ein Prozess?

Jedes Mal, wenn du ein Programm startest, erzeugt Linux einen [**Prozess**](../glossar.md#prozess). Jeder Prozess bekommt eine eindeutige Nummer, die [**PID**](../glossar.md#pid-process-id) (Process ID).

- Ein Webserver wie nginx ist ein Prozess.
- Deine [Shell](../glossar.md#shell) ([Bash](../glossar.md#bash)) ist ein Prozess.
- Der [SSH](../glossar.md#ssh-secure-shell)-[Daemon](../glossar.md#daemon), der deine Verbindung hält, ist ein Prozess.

Prozesse können weitere Prozesse starten – diese nennt man **Kindprozesse** (child processes). Der erste Prozess beim Systemstart heißt `systemd` und hat immer die PID 1.

```
$ ps --pid 1
  PID TTY          TIME CMD
    1 ?        00:00:03 systemd
```

**Windows-Analogie:** Wie im Task-Manager unter "Details" – jedes Programm hat eine PID. In Windows heißt das auch PID.

---

## Vordergrund vs. Hintergrund

Wenn du einen Befehl im Terminal eingibst, läuft er standardmäßig im **Vordergrund**: Das Terminal wartet, bis er fertig ist. Du kannst keine anderen Befehle eingeben, solange er läuft.

### Vordergrundprozess

```bash
# Das Terminal ist blockiert, bis ping gestoppt wird
ping google.com
```

Stoppe einen Vordergrundprozess mit `Strg+C`.

### Hintergrundprozess

Mit `&` am Ende startest du einen Befehl im **Hintergrund**. Das Terminal ist sofort wieder frei.

```bash
# Startet ping im Hintergrund
ping google.com > /dev/null &
# [1] 4821   ← Jobnummer und PID werden angezeigt
```

### Zwischen Vorder- und Hintergrund wechseln

```bash
# Laufenden Vordergrundprozess anhalten (pausieren)
Strg+Z

# Angehaltenen Job im Hintergrund weiterlaufen lassen
bg

# Hintergrundjob wieder in den Vordergrund holen
fg

# Alle aktuellen Jobs in dieser Shell anzeigen
jobs
```

Beispiel einer typischen Sitzung:

```
$ sleep 300
^Z
[1]+  Stopped                 sleep 300

$ bg
[1]+ sleep 300 &

$ jobs
[1]+  Running                 sleep 300 &

$ fg
sleep 300
^C
```

**Windows-Analogie:** Ein minimiertes Fenster läuft noch im Hintergrund – ähnlich wie ein Hintergrundprozess. `Strg+C` entspricht dem Klick auf "X" (Schließen).

---

## ps – Prozesse auflisten

`ps` (process status) zeigt eine Momentaufnahme der laufenden Prozesse.

### Nur deine eigenen Prozesse

```bash
ps
```

Ausgabe:
```
  PID TTY          TIME CMD
 1234 pts/0    00:00:00 bash
 5678 pts/0    00:00:00 ps
```

### Alle Prozesse des Systems anzeigen

```bash
ps aux
```

Ausgabe (gekürzt):
```
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  0.0  0.1 168568 11832 ?        Ss   08:00   0:03 /sbin/init
www-data  1042  0.1  0.5 123456 45678 ?        S    08:01   0:12 nginx: worker
alice     2391  0.0  0.0  10256  3456 pts/0    Ss   10:30   0:00 bash
```

Spalten erklärt:
- `USER` – Wer hat den Prozess gestartet?
- `PID` – Prozess-ID
- `%CPU` – CPU-Auslastung in Prozent
- `%MEM` – Arbeitsspeicher-Auslastung in Prozent
- `STAT` – Status (`S` = schlafend, `R` = läuft, `Z` = zombie)
- `COMMAND` – Der gestartete Befehl

### Nach einem bestimmten Prozess suchen

```bash
ps aux | grep nginx
```

```
www-data  1042  0.1  0.5 123456 45678 ?  S  08:01  0:12 nginx: worker process
```

**Tipp:** `pgrep` gibt nur die PID zurück – praktisch für Skripte:

```bash
pgrep nginx
# 1042
```

---

## top und htop – Echtzeit-Systemüberwachung

### top

`top` zeigt laufende Prozesse in Echtzeit, sortiert nach CPU-Auslastung. Es ist auf jedem Linux-System vorinstalliert.

```bash
top
```

Beispielausgabe:
```
top - 10:45:01 up 2 days, 3:12,  1 user,  load average: 0.05, 0.08, 0.06
Tasks: 112 total,   1 running, 111 sleeping,   0 stopped,   0 zombie
%Cpu(s):  1.2 us,  0.3 sy,  0.0 ni, 98.3 id,  0.1 wa
MiB Mem :   1987.5 total,    423.1 free,    891.2 used,    673.2 buff/cache
MiB Swap:   2048.0 total,   2047.9 free,      0.1 used.    942.1 avail Mem

  PID USER      PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TIME+ COMMAND
 1042 www-data  20   0  123456  45678   8765 S   0.7   2.2   0:12.34 nginx
 2391 alice     20   0   10256   3456   2345 S   0.0   0.2   0:00.01 bash
```

Wichtige Tastenkürzel in `top`:
- `q` – Beenden
- `k` – Prozess beenden (nach PID fragen)
- `M` – Nach Speicherverbrauch sortieren
- `P` – Nach CPU-Auslastung sortieren
- `1` – Alle CPU-Kerne einzeln anzeigen

### Die Kopfzeilen verstehen

```
load average: 0.05, 0.08, 0.06
```

Die **Load Average** zeigt die durchschnittliche Systemlast der letzten 1, 5 und 15 Minuten. Ein Wert unter der Anzahl deiner CPU-Kerne bedeutet: alles in Ordnung.

```bash
# Anzahl der CPU-Kerne anzeigen
nproc
# 2
```

Ein Load Average von 2.0 auf einem 2-Kern-System bedeutet volle Auslastung.

### htop – der komfortablere Task-Manager

`htop` ist wie `top`, aber mit Farben, Mausbedienung und übersichtlicherer Darstellung. Es muss zuerst installiert werden:

```bash
sudo apt install htop
```

```bash
htop
```

`htop` zeigt oben Balken für CPU, Arbeitsspeicher und Swap. Prozesse lassen sich direkt mit der Maus auswählen und mit `F9` beenden.

**Windows-Analogie:** `htop` ist der Linux-Task-Manager mit grafischer Darstellung – nur im Terminal.

---

## kill – Prozesse beenden

Wenn ein Programm eingefroren ist oder unerwünschtes Verhalten zeigt, kannst du es mit `kill` beenden.

### Prozess sanft beenden (SIGTERM)

```bash
kill 4821
```

Das schickt dem Prozess das Signal `SIGTERM` – eine freundliche Aufforderung, sich zu beenden. Der Prozess kann aufräumen und dann sauber schließen.

### Prozess sofort abwürgen (SIGKILL)

```bash
kill -9 4821
```

`SIGKILL` lässt dem Prozess keine Wahl – er wird sofort vom System beendet, ohne aufzuräumen. Nur verwenden, wenn der sanfte Weg nicht funktioniert.

### Prozess nach Name beenden

```bash
# Alle Prozesse mit diesem Namen beenden
pkill nginx

# Prozessnamen in der Ausgabe anzeigen (kein interaktives Bestätigen)
pkill -l nginx
```

### Typischer Ablauf

```bash
# 1. PID herausfinden
ps aux | grep firefox
# alice  7890  12.3  4.1  ...  firefox

# 2. Prozess sanft beenden
kill 7890

# 3. Falls er nicht reagiert: erzwingen
kill -9 7890
```

**Windows-Analogie:** Rechtsklick im Task-Manager -> "Task beenden" entspricht `kill`. "Task sofort beenden" entspricht `kill -9`.

---

## systemctl status – Dienste überwachen

Dienste (auch Daemons genannt) sind Hintergrundprozesse, die beim Systemstart automatisch gestartet werden – z.B. ein Webserver oder SSH.

`systemctl` ist das Werkzeug zur Verwaltung dieser Dienste unter Ubuntu 24.04.

### Status eines Dienstes prüfen

```bash
systemctl status nginx
```

Ausgabe:
```
● nginx.service - A high performance web server and a reverse proxy server
     Loaded: loaded (/lib/systemd/system/nginx.service; enabled; vendor preset: enabled)
     Active: active (running) since Mon 2026-03-01 08:01:23 UTC; 2h 44min ago
       Docs: man:nginx(8)
   Main PID: 1042 (nginx)
      Tasks: 2 (limit: 2237)
     Memory: 4.5M
        CPU: 12.345s
     CGroup: /system.slice/nginx.service
             ├─1042 "nginx: master process /usr/sbin/nginx -g daemon on; master_process on;"
             └─1043 "nginx: worker process"

Mär 01 08:01:23 server nginx[1042]: nginx: the configuration file /etc/nginx/nginx.conf syntax is ok
```

Die wichtigsten Zeilen:
- `Active: active (running)` – Der Dienst läuft.
- `Active: inactive (dead)` – Der Dienst ist gestoppt.
- `Active: failed` – Der Dienst ist abgestürzt.
- `enabled` – Der Dienst startet automatisch beim Booten.
- `disabled` – Der Dienst startet nicht automatisch.

### Weitere nützliche systemctl-Befehle

```bash
# Dienst starten
sudo systemctl start nginx

# Dienst stoppen
sudo systemctl stop nginx

# Dienst neu starten (z.B. nach Konfigurationsänderung)
sudo systemctl restart nginx

# Dienst neu laden ohne Unterbrechung (wenn unterstützt)
sudo systemctl reload nginx

# Dienst beim Booten automatisch starten
sudo systemctl enable nginx

# Automatischen Start deaktivieren
sudo systemctl disable nginx

# Alle laufenden Dienste auflisten
systemctl list-units --type=service --state=running
```

**Windows-Analogie:** `systemctl` entspricht dem Windows-Dienste-Manager (`services.msc`). `enable` ist wie "Starttyp: Automatisch", `disable` ist wie "Starttyp: Manuell".

---

## journalctl – Protokolle lesen

Unter Ubuntu 24.04 sammelt `systemd` alle Protokolle zentral in einem Journal. `journalctl` ist das Werkzeug, um diese Protokolle zu lesen.

**Windows-Analogie:** Wie die Windows-Ereignisanzeige (`eventvwr.msc`), aber als Kommandozeilenwerkzeug.

### Alle Protokolle anzeigen (neueste zuerst)

```bash
journalctl -r
```

`-r` steht für "reverse" – die neuesten Einträge zuerst.

### Protokolle eines bestimmten Dienstes anzeigen

```bash
journalctl -u nginx
```

```
Mär 01 08:01:23 server nginx[1042]: nginx: configuration file /etc/nginx/nginx.conf test is successful
Mär 01 08:01:23 server systemd[1]: Started A high performance web server.
```

### Protokolle live verfolgen (wie tail -f)

```bash
journalctl -u nginx -f
```

`-f` steht für "follow" – neue Einträge werden sofort angezeigt. Mit `Strg+C` beenden.

### Protokolle seit dem letzten Systemstart

```bash
journalctl -b
```

### Protokolle nach Zeitraum filtern

```bash
# Protokolle der letzten Stunde
journalctl --since "1 hour ago"

# Protokolle zwischen zwei Zeitpunkten
journalctl --since "2026-03-01 08:00" --until "2026-03-01 09:00"
```

### Nur Fehlermeldungen anzeigen

```bash
# Nur Fehler (priority: error und kritischer)
journalctl -p err

# Noch strenger: nur kritische Fehler
journalctl -p crit
```

### Protokolle eines bestimmten Dienstes mit Fehlerfilter kombinieren

```bash
journalctl -u nginx -p err -r
```

### Kompakte Zusammenfassung für schnellen Überblick

```bash
# Die letzten 50 Zeilen eines Dienstprotokolls
journalctl -u ssh -n 50
```

**Tipp:** Wenn ein Dienst nicht startet, ist `journalctl -u <dienstname> -r` oft der erste Schritt zur Fehlersuche.

---

## Systemressourcen auf einen Blick

Neben Prozessen ist es wichtig, den allgemeinen Systemzustand zu kennen:

```bash
# Arbeitsspeicher-Überblick
free -h
```

```
               total        used        free      shared  buff/cache   available
Mem:           1.9Gi       891Mi       423Mi        10Mi       673Mi       942Mi
Swap:          2.0Gi         0Mi       2.0Gi
```

```bash
# Festplattenbelegung
df -h
```

```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1        20G  8.1G   11G  43% /
tmpfs           994M     0  994M   0% /dev/shm
```

```bash
# Systemlaufzeit und Load Average
uptime
# 10:45:01 up 2 days, 3:12,  1 user,  load average: 0.05, 0.08, 0.06
```

---

## Schnellreferenz

| Aufgabe | Befehl |
|--------|--------|
| Alle Prozesse anzeigen | `ps aux` |
| Nach Prozess suchen | `ps aux \| grep nginx` |
| PID eines Prozesses finden | `pgrep nginx` |
| Echtzeit-Übersicht | `top` oder `htop` |
| Prozess sanft beenden | `kill <PID>` |
| Prozess sofort beenden | `kill -9 <PID>` |
| Prozess nach Name beenden | `pkill nginx` |
| Dienststatus prüfen | `systemctl status nginx` |
| Dienst starten/stoppen | `sudo systemctl start/stop nginx` |
| Dienst beim Booten aktivieren | `sudo systemctl enable nginx` |
| Protokolle eines Dienstes | `journalctl -u nginx` |
| Protokolle live verfolgen | `journalctl -u nginx -f` |
| Nur Fehler anzeigen | `journalctl -p err -r` |
| Arbeitsspeicher prüfen | `free -h` |
| Festplatte prüfen | `df -h` |

---

## Troubleshooting: Häufige Szenarien

**Ein Dienst startet nicht nach dem Neustart:**
```bash
systemctl status nginx
journalctl -u nginx -r -n 30
```

**Das System fühlt sich langsam an:**
```bash
htop          # Wer verbraucht CPU/RAM?
free -h       # Ist der Arbeitsspeicher voll?
df -h         # Ist die Festplatte voll?
```

**Ein Programm reagiert nicht mehr:**
```bash
pgrep programmname     # PID herausfinden
kill <PID>             # Sanft beenden
kill -9 <PID>          # Erzwingen, falls nötig
```

**Was ist gerade alles auf dem Server aktiv?**
```bash
systemctl list-units --type=service --state=running
```
