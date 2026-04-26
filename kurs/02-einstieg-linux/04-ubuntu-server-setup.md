# Ubuntu Server Setup

Vor dem Start:
- VM-Software installiert (VirtualBox, VMware, UTM, QUEMU …)
- Ubuntu Server 24.04 LTS ISO heruntergeladen: https://ubuntu.com/download/server

Achtet darauf, dass wir die richtige ISO für eure CPU habt. Vermutlich habt ihr einen amd64/x86 CPU. Für diesen funktioniert das "standard" Image. Besonders wenn ihr auf einem Mac arbeitet solltet ihr noch einmal schauen, ob ihr nicht doch einen arm64 CPU habt. Dann müsst ihr unter "Andere Architekturen" nach arm64 suchen und diese ISO verwenden.

---

## 1. VM einrichten

### Warum eine VM?

Wir nutzen eine virtuelle Maschine, um einen echten Server zu simulieren, ohne etwas am eigenen Rechner zu riskieren. Eine VM kannst du jederzeit löschen und neu aufsetzen. Das ist auch der Grund, warum wir **kein WSL** nutzen: WSL fühlt sich wie ein Linux-Terminal an, verhält sich aber nicht wie ein echter Server. Da wir [Docker](../../wiki/glossar.md#docker) installieren werden, verwenden wir auch keinen [Container](../../wiki/glossar.md#container) um das System auszuprobieren.

### VirtualBox-Einstellungen

| Einstellung | Wert                 |
|-------------|----------------------|
| Typ         | Linux, Ubuntu 64-bit |
| RAM         | mindestens 2048 MB   |
| Festplatte  | 20 GB, dynamisch     |
| Netzwerk    | NAT (Standard)       |

In VirtualBox machen wir 2 Port Forwarding Regeln:

| Name (z.B.) | Protokoll | Host-IP   | Host-Port | Gast-IP | Gast-Port |
|-------------|-----------|-----------|-----------|---------|-----------|
| SSH         | TCP       | 127.0.0.1 | 8022      | <leer>  | 22        |
| HTTP        | TCP       | 127.0.0.1 | 8080      | <leer>  | 80        |

### Ubuntu Server installieren

```
1. Sprache:      English  (Fehlermeldungen lassen sich so besser googlen)
2. Tastatur:     German
3. Netzwerk:     Standard übernehmen
4. Festplatte:   "Use entire disk"
5. Benutzerkonto anlegen
6. OpenSSH:      JA aktivieren
7. Snaps:        nichts auswählen, überspringen
```

Nach dem Neustart einloggen und direkt updaten:

```bash
sudo apt update && sudo apt upgrade -y
```

> **Warum zuerst updaten?**  
> Das frisch installierte System hat den Stand der ISO. Dieser kann Wochen oder Monate alt sein. Mit dem ersten Update schließt man bekannte Sicherheitslücken.

---

### SSH (Secure Shell) Einrichten

Noch ist der Server nicht über [SSH](../../wiki/glossar.md#ssh-secure-shell) (Secure Shell) erreichbar. Dafür müssen wir ssh zuerst aktivieren.

```bash
sudo systemctl enable --now ssh
```

> **Warnung**
> Wir können das so einfach machen, weil der Server nur in unserer virtuellen Maschine läuft. Im freien Internet dauert es nur wenige Minuten bis die ersten automatisierten Angriffe versuchen das Passwort über SSH versuchen zu knacken. In der Praxis würden wir daher vorher:
> - Eine [Firewall](../../wiki/glossar.md#firewall) vorschalten
> - [Rate Limiting](../../wiki/glossar.md#rate-limiting) einrichten
> - Root Login über SSH blockieren
> - Einen SSH Key statt einfachem Passwort einrichten und Passwort-Login verbieten
> - SSH Port vom Standardport 22 auf einen ungewöhnlichen Port verlegen
> - Logging/Monitoring einrichten
> Für diese Anleitung sparen wir uns diese Schritte.

---

### SSH verbinden

Jetzt verbinden wir uns mit der VM über unser Terminal von unserem Hauptsystem. Wir könnten das natürlich auch direkt über die VM machen, aber in der Praxis haben wir keinen Bildschirm am Server und der Server könnte überall auf der Welt stehen. Daher machen wir das so, wie wir es auch in der Praxis machen würden.

Wenn unser hauptsystem Linux ist, dann öffnen wir einfach eine [Shell](../../wiki/glossar.md#shell) und geben das hier ein:
```bash
ssh -p 8022 username@127.0.0.1
```

Dann werden wir nach dem Passwort gefragt.

Auf Windows ist ssh nicht einfach in die Konsole eingebaut. Es ist hier am einfachsten das Programm PuTTY zu installieren. Dieses gibt einem Eingabemasken für die gleichen Parameter und öffnet dann eine Shell.

## 2. Benutzerverwaltung

### Warum nicht alles als root?

Root darf alles. Ein Tippfehler wie `rm -rf /etc` lässt sich nicht rückgängig machen. Deshalb arbeiten wir als normaler Nutzer und verwenden `sudo` nur wenn nötig. `sudo` loggt außerdem jeden Befehl, den du damit ausführst.

```bash
# Wer bin ich gerade?
whoami

# Mit sudo: Befehl einmalig als root ausführen
sudo whoami

# Zu root wechseln (besser vermeiden)
sudo su -
exit   # zurück zum normalen User
```

### Gruppen

Gruppen steuern, wer auf was zugreifen darf. Später wird das bei Docker wichtig.

```bash
# Meine Gruppen anzeigen
groups

# Nutzer zu einer Gruppe hinzufügen
sudo usermod -aG gruppenname nutzername
```

> **Exkurs: Dateiberechtigungen**  
> Jede Datei in Linux gehört einem Benutzer und einer Gruppe.
> - `ls -la`: Hiermit sieht man diese Informationen. Die Spalte ganz links (`-rw-r--r--`) zeigt: Besitzer kann lesen/schreiben, Gruppe kann lesen, alle anderen können lesen. 
> - `chmod`: ändert diese Rechte
> - `chown`: ändert den Besitzer.

---

## 3. Basics in der Shell

### Orientierung

```bash
# Wo bin ich?
pwd

# Was ist hier?
ls
ls -la        # alle Dateien, auch versteckte, mit Details

# Navigieren
cd /etc       # absoluter Pfad von der Wurzel aus
cd ~          # Home-Verzeichnis des aktuellen Nutzers
cd ..         # ein Verzeichnis nach oben

# ~ ist eine Abkürzung für /home/deinnutzername
# / ist die Wurzel des gesamten Dateisystems
```

### `/` vs `~/`?

`/` ist der Anfang von allem. Jeder Pfad, der mit `/` beginnt, ist absolut und eindeutig, egal wo du gerade bist. `~/` ist eine Abkürzung für das Home-Verzeichnis des aktuellen Nutzers (`/home/admin`). Relative Pfade (ohne führendes `/`) gehen immer vom aktuellen Verzeichnis aus.

### Umgebungsvariablen

```bash
# Alle gesetzten Variablen anzeigen
env

# Eine Variable anzeigen
echo $HOME
echo $PATH

# Variable temporär setzen (nur in dieser Shell-Sitzung)
export MEIN_NAME="World"
echo "Hello $MEIN_NAME"
```

### `.bashrc`: Die Konfigurationsdatei der Shell

```bash
# Datei anschauen
cat ~/.bashrc

# Eigenes Alias hinzufügen
echo 'alias ll="ls -la"' >> ~/.bashrc

# Änderungen ohne Neustart laden
source ~/.bashrc

# Jetzt funktioniert:
ll
```

> **Warum `source` und nicht einfach die Datei ausführen?**  
> Ein normales `./skript.sh` startet eine neue Shell, macht die Änderungen dort und die neue Shell wird danach wieder geschlossen. 
> `source` führt die Befehle direkt in der aktuellen Shell aus, deshalb bleiben Variablen und [Aliases](../../wiki/glossar.md#alias-shell-alias) erhalten.

> **Exkurs: Weitere Config-Dateien**  
> `.bashrc` wird bei jeder interaktiven Shell geladen. `.bash_profile` oder `.profile` werden nur beim Login geladen. Systemweite Einstellungen liegen in `/etc/environment` und `/etc/bash.bashrc`. 
> Für systemweite PATH-Erweiterungen: `/etc/profile.d/`.

---

## 4. Pakete installieren

### apt update vs. apt upgrade

```bash
# Paketlisten aktualisieren (nur die Liste, noch nichts installieren)
sudo apt update

# Pakete auf neue Versionen bringen (-y überspringt die Abfrage mit "yes")
sudo apt upgrade -y

# Kurz: beides zusammen
sudo apt update && sudo apt upgrade -y
```

`apt update` ist wie das Anschauen des aktuellen Einkaufs-Prospekts. Du siehst, was es gibt.

`apt upgrade` ist die eigentliche Bestellung.

Wenn du `apt update` nicht vorher ausführst, dann kann es passieren, dass du glaubst, die neueste Software herunterzuladen, aber stattdessen eine veraltete Version bekommst.

> **Exkurs: unattended-upgrades**
> Auf Linux passiert grundsätzlich eigentlich nichts, wenn man es nicht aktiviert. Während Windows einen immer gleich erstmal mit regemäßigen Updates nervt, muss man das in Linux selbst aktivieren. Das ist keine ganz einfache Entscheidung. Dafür spricht, dass man immer die neuesten Versionen hat und damit bekannte Sicherheitslücken relativ schnell schließt. Dagegen spricht, dass jedes Upgrade auch Dinge kaputt machen kann. Je nach Use Case kann also das eine oder das andere Korrekt sein. 
> Empfehlung: Wenn dein Server auch mal crashen darf, dann sind die Risiken durch die automatischem Upgrades vermutlich geringer als das Risiko ein paar monate die Upgrades zu vergessen und dann mit bekannten Sicherheitslücken rumzulaufen.
> Hier geht es zur Installation: [wiki/server-maintenance/02-updates-upgrades.md](../../wiki/server-maintenance/02-updates-upgrades.md)

### Pakete installieren

```bash
sudo apt install htop
sudo apt install tree
sudo apt install curl
```

Jetzt ausprobieren:

```bash
htop        # Prozess-Monitor, beenden mit q
tree /etc   # Verzeichnisstruktur als Baum
curl https://example.com
```

### Warum `apt` und nicht `apt-get`?

`apt-get` ist das ältere, stabilere Tool. Besser für Skripte, weil die Ausgabe sich nicht ändert. `apt` ist die modernere Variante mit etwas benutzerfreundlicherer Ausgabe (Fortschrittsbalken, etc.). Für den täglichen Gebrauch: `apt`. In Skripten, die zuverlässig laufen sollen: `apt-get`.

> **Exkurs: Andere Paketmanager**  
> - `snap`: Canonical's eigenes Format, bringt alle Abhängigkeiten mit. Läuft isoliert (Sandbox). Oft etwas größer und langsamer. Manche Applikationen funktionieren in den Sandboxes nicht korrekt.
> - `dpkg`: Das unterliegende Werkzeug, auf dem `apt` aufbaut. Direkte `.deb`-Dateien installieren.
> - `aur`: Arch linux pakete. Extrem große Auswahl und sehr aktuell, aber weniger Stabil*
> - `nix`: Deterministischer Paketmanager mit Rollbackfunktion. Mächtig, aber komplett andere Denkweise.
> - `pip`/`uv`/`conda`: Python-Pakete. Komplett anderes Ökosystem.
> - `npm`/`cargo`/`go install`: Sprach-spezifische Package Manager. Diese installieren nichts systemweit.
> Pakete können auf verschiedenen Packetverzeichnissen zu finden sein. Daher gibt es meistens mehrere Möglichkeiten etwas zu installieren.
> Empfehlung: Auf einem Server möglichst wenig verschiedene Package Manager mischen:
> - Es wird sonst schwer nachvollziehbar, was woher kommt.
> - Pakete aus verschiedenen Quellen verhalten sich nicht immer 100% identisch
> - Es wird auch leicht vergessen einzelne Pakete zu updaten. Auf einem Server wollt ihr alles möglichst einfach und Übersichtlich haben.

---

## 5. Docker installieren

### Warum Docker nicht aus den Standard-Quellen?

Ubuntu liefert in seinem Standard-Repository eine oft veraltete Docker-Version. Für aktuelle Features und Sicherheitsupdates brauchen wir die offizielle Docker-Paketquelle. Dafür müssen wir Ubuntu erst beibringen, dieser Quelle zu vertrauen.

### Schritt 1: Abhängigkeiten und GPG-Key

```bash
sudo apt update
sudo apt install ca-certificates curl gnupg

sudo install -m 0755 -d /etc/apt/keyrings

curl -fsSL https://download.docker.com/linux/ubuntu/gpg | \
  sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
```

> **Warum ein GPG-Key?**  
> apt prüft bei jedem Download, ob das Paket wirklich von einem vertrauenswürdigen Anbieter kommt. Vergleichbar mit einer digitalen Unterschrift, die verifiziert dass die Pakete wirklich von Docker kommen. Ohne diesen Key würde apt die Docker-Pakete ablehnen.

### Schritt 2: Docker-Repository hinzufügen

```bash
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] \
  https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo $VERSION_CODENAME) stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
```

### Schritt 3: Docker installieren

```bash
sudo apt update
sudo apt install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

sudo systemctl enable --now docker
```

### Schritt 4: User in die docker-Gruppe

```bash
sudo usermod -aG docker $USER
```

**Wichtig:** Jetzt ausloggen und wieder einloggen, damit die Gruppenänderung wirksam wird.

```bash
exit
# neu einloggen, dann:
groups   # "docker" sollte jetzt auftauchen
```

> **Warum die Gruppe?**  
> Standardmäßig darf nur root mit Docker sprechen. Wer in der `docker`-Gruppe ist, darf das auch ohne `sudo`. Achtung: Docker-Zugriff ist fast gleichwertig mit root-Zugriff. Wer Docker steuern kann, kann Container starten, die das Host-Dateisystem einbinden.
> Tipp für einen echten Server: Richtet einen separaten User für docker ein. Wenn ihr etwas auf Docker ändern wollt, dann wechselt ihr zu diesem User. Damit baut ihr eine weitere Sicherheitsebene ein und ihr verhindert, dass ihr mit euren hohen Rechten ausversehen etwas kaputt macht. Wenn ihr mehrere Administratoren habt, dann wollt ihr auch generell, dass alle Admins einen persönlichen Account haben. Wenn jeder die Container von seinem persönlichen Account aus starten würde, dann könnte das löschen eines Accounts den Server crashen. Daher wird für solche Hintergrundprozesse gerne ein solcher Service-Account angelegt. Das erscheint am Anfang etwas umständlicher, aber es hilft später bei der Berechtigungsverwaltung und beim Monitoring.

### Installation prüfen

```bash
docker --version
docker compose version
docker run hello-world
```

---

## 6. Erste App mit Docker Compose

### Was ist Docker Compose?

[Docker Compose](../../wiki/glossar.md#docker-compose) erlaubt es, Container nicht mit langen `docker run`-Befehlen zu starten, sondern ihre Konfiguration in einer [YAML](../../wiki/glossar.md#yaml-yet-another-markup-language)-Datei zu speichern. Das ist reproduzierbar, versionierbar und lesbar.

Wir starten [Excalidraw](https://excalidraw.com/), ein kollaboratives Whiteboard-Tool. Sonst nur als [SaaS](../../wiki/glossar.md#saas-software-as-a-service) verfügbar, läuft hier komplett auf der eigenen VM. Mehrere Personen können gleichzeitig im selben Canvas zeichnen.

Das Setup besteht aus vier Containern, die zusammenarbeiten:
- **excalidraw**: die eigentliche Web-UI
- **excalidraw-storage-backend**: speichert Zeichnungen
- **excalidraw-room**: koordiniert die Echtzeit-Zusammenarbeit
- **[redis](../../wiki/glossar.md#redis)**: Zwischenspeicher für den Storage-Backend

### Arbeitsverzeichnis anlegen

```bash
mkdir ~/excalidraw
cd ~/excalidraw
```

### `docker-compose.yml` erstellen

```bash
nano docker-compose.yml
```

Inhalt einfügen:

```yaml
services:
  excalidraw:
    image: kiliandeca/excalidraw
    ports:
      - "80:80"
    environment:
      BACKEND_V2_GET_URL: http://excalidraw-storage-backend:8080/api/v2/scenes/
      BACKEND_V2_POST_URL: http://excalidraw-storage-backend:8080/api/v2/scenes/
      SOCKET_SERVER_URL: http://excalidraw-room:80/
      STORAGE_BACKEND: "http"
      HTTP_STORAGE_BACKEND_URL: "http://excalidraw-storage-backend:8080/api/v2"
  excalidraw-storage-backend:
    image: kiliandeca/excalidraw-storage-backend
    environment:
      STORAGE_URI: redis://redis:6379
  excalidraw-room:
    image: excalidraw/excalidraw-room
  redis:
    image: redis
```

Speichern: `Strg+O`, `Enter`, `Strg+X`

> **Warum keine `localhost`-URLs?**  
> Innerhalb eines Docker-Netzwerks sind Container nicht über `localhost` erreichbar. Das würde auf den Container selbst zeigen. Container erreichen sich gegenseitig über den **Service-Namen** aus der `docker-compose.yml`. Docker löst diese Namen automatisch auf.

> **Warum keine Ports für Redis und die anderen Services?**  
> Nur was nach außen erreichbar sein muss, bekommt ein Port-Mapping. Redis, Storage-Backend und Room werden nur von den anderen Containern intern genutzt. Kein Grund, sie dem Host-System (oder dem Netzwerk) zu exponieren.

### Starten

```bash
docker compose up -d
```

`-d` steht für "detached". Die Container laufen im Hintergrund.

> **Hinweis bei NAT:** Mit NAT-Netzwerk ist die VM nur vom Host aus erreichbar, wenn ein Port-Forwarding eingerichtet ist. VirtualBox: Einstellungen → Netzwerk → Erweitert → Port-Weiterleitung: Host-Port z.B. `8080` → Gast-Port `80`. Dann im Browser: `http://localhost:8080`

### Wichtige Docker-Befehle

```bash
# Laufende Container anzeigen
docker ps

# Alle Container (auch gestoppte)
docker ps -a

# Logs eines Containers anzeigen
docker compose logs -f

# Container stoppen
docker compose down

# Container wieder starten
docker compose up -d

# Nicht genutzte Images/Container aufräumen 
docker system prune
```

> **Exkurs: `restart: unless-stopped`**  
> Ohne `restart`-Policy laufen Container nach einem Serverneustart nicht automatisch wieder an. Mit `restart: unless-stopped` startet Docker sie automatisch, außer man hat sie bewusst mit `docker compose down` gestoppt. Für Produktions-Setups sinnvoll, hier bewusst weggelassen um das Compose-File einfach zu halten.

---