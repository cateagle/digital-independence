# 04 - Paketverwaltung unter Ubuntu

Software unter Linux zu installieren funktioniert grundlegend anders als unter Windows. Statt Installer-Dateien von verschiedenen Webseiten herunterzuladen, gibt es einen zentralen Paketmanager, der sich um alles kümmert. Dieses Kapitel erklärt, wie das funktioniert und warum es die sicherere Methode ist.

**Windows-Analogie:** Stell dir vor, du hättest einen einzigen App-Store (wie den Microsoft Store), der alle Programme enthält – und du müsstest nie wieder auf irgendeine Webseite gehen, um eine `.exe` herunterzuladen.

---

## Was ist apt?

`apt` ist der Paketmanager von Ubuntu. Er installiert, aktualisiert und entfernt Software – und löst dabei automatisch Abhängigkeiten auf (d.h. er installiert alle weiteren Bibliotheken, die ein Programm braucht).

Fast alle Befehle zur Paketverwaltung brauchen Root-Rechte, deshalb werden sie mit `sudo` ausgeführt.

---

## apt update – Die Paketliste aktualisieren

Bevor du Software installierst oder aktualisierst, musst du die Paketliste aktualisieren. Diese Liste enthält Informationen darüber, welche Software in welcher Version verfügbar ist.

```bash
sudo apt update
```

Beispielausgabe:
```
Hit:1 http://archive.ubuntu.com/ubuntu noble InRelease
Get:2 http://security.ubuntu.com/ubuntu noble-security InRelease [126 kB]
Fetched 126 kB in 1s (98,7 kB/s)
Reading package lists... Done
```

**Wichtig:** `apt update` installiert noch nichts. Es aktualisiert nur die interne Liste, was verfügbar ist – vergleichbar mit "Im App-Store nach Updates suchen", bevor du auf "Aktualisieren" klickst.

---

## apt upgrade – Installierte Pakete aktualisieren

Nachdem die Paketliste aktualisiert wurde, kannst du alle installierten Programme auf die neueste Version bringen:

```bash
sudo apt upgrade
```

apt zeigt eine Zusammenfassung der geplanten Änderungen und fragt nach Bestätigung:

```
The following packages will be upgraded:
  curl libssl3 openssh-client
3 upgraded, 0 newly installed, 0 to remove and 0 not upgraded.
Do you want to continue? [Y/n]
```

Drücke `Y` und Enter zum Bestätigen (oder einfach Enter, da Y der Standard ist).

**Tipp:** `apt update` und `apt upgrade` werden oft direkt hintereinander ausgeführt:
```bash
sudo apt update && sudo apt upgrade
```

---

## apt install – Software installieren

```bash
sudo apt install <paketname>
```

Beispiel: `htop` (ein verbesserter Prozess-Monitor) installieren:
```bash
sudo apt install htop
```

Mehrere Pakete auf einmal installieren:
```bash
sudo apt install htop curl wget
```

Mit dem Flag `-y` wird die Nachfrage übersprungen (nützlich in Skripten):
```bash
sudo apt install -y htop
```

**Windows-Analogie:** Wie "Installieren" im Microsoft Store klicken – aber ohne den Umweg über eine Webseite.

---

## apt remove – Software entfernen

```bash
sudo apt remove <paketname>
```

Beispiel:
```bash
sudo apt remove htop
```

`apt remove` entfernt das Programm, lässt aber Konfigurationsdateien zurück. Um diese ebenfalls zu löschen:

```bash
sudo apt purge htop
```

**Windows-Analogie:** Wie "Programme und Features" in der Systemsteuerung, aber zuverlässiger – es bleiben keine verstreuten Dateien zurück.

---

## apt autoremove – Nicht mehr benötigte Pakete aufräumen

Wenn du ein Programm entfernst, können Abhängigkeiten (Bibliotheken, die nur für dieses Programm gebraucht wurden) übrig bleiben. `autoremove` räumt diese auf:

```bash
sudo apt autoremove
```

Beispielausgabe:
```
The following packages will be REMOVED:
  libfoo2 libbar1
0 upgraded, 0 newly installed, 2 to remove and 0 not upgraded.
Do you want to continue? [Y/n]
```

**Tipp:** Es empfiehlt sich, `autoremove` regelmäßig auszuführen, um das System sauber zu halten. Zusammen mit Update und Upgrade:
```bash
sudo apt update && sudo apt upgrade && sudo apt autoremove
```

---

## apt search – Nach Paketen suchen

Du weißt nicht genau, wie ein Paket heißt? Mit `apt search` kannst du die Paketliste durchsuchen:

```bash
apt search <suchbegriff>
```

Beispiel – nach einem Texteditor suchen:
```bash
apt search text editor
```

Für eine kompaktere Übersicht:
```bash
apt search neovim | head -20
```

Um mehr Informationen zu einem bestimmten Paket zu erhalten:
```bash
apt show htop
```

Ausgabe von `apt show htop`:
```
Package: htop
Version: 3.2.2-1
Description: interactive processes viewer
 htop is an interactive (text-mode) process viewer for Unix systems.
```

---

## Repositories verstehen

Woher kommen die Pakete eigentlich? Ubuntu nutzt sogenannte **[Repositories](../glossar.md#repository-apt)** (kurz: Repos) – das sind Server, auf denen geprüfte und signierte Software liegt.

**Windows-Analogie:** Ein Repository ist wie der Microsoft Store-Katalog: eine kuratierte Liste von vertrauenswürdiger Software.

Die Quellen sind in dieser Datei konfiguriert:
```
/etc/apt/sources.list
```

Und im Verzeichnis:
```
/etc/apt/sources.list.d/
```

Ubuntu liefert standardmäßig mehrere Repos:
- **main**: Offiziell von Canonical gepflegt, vollständig Open Source
- **universe**: Community-gepflegt, Open Source
- **restricted**: Proprietäre Treiber (z.B. Nvidia)
- **multiverse**: Software mit Lizenzeinschränkungen

### Wie werden Pakete auf Echtheit geprüft?

Jedes Repository ist mit einem kryptografischen Schlüssel signiert. `apt` prüft diese Signatur automatisch bei jedem Download. Wenn etwas nicht stimmt, verweigert apt die Installation mit einer Fehlermeldung.

Das ist ein wesentlicher Sicherheitsvorteil gegenüber dem manuellen Herunterladen von Installationsdateien.

### Drittanbieter-Repositories hinzufügen (PPAs)

Manchmal ist Software nicht in den Standard-Repos enthalten. Ubuntu unterstützt sogenannte **PPAs** (Personal Package Archives). Diese werden mit `add-apt-repository` hinzugefügt.

Beispiel:
```bash
sudo add-apt-repository ppa:ondrej/php
sudo apt update
sudo apt install php8.3
```

**Vorsicht:** Drittanbieter-Repos sind nicht von Canonical geprüft. Vertraue nur bekannten Quellen.

---

## snap – Ein zweites Paketsystem

Neben `apt` gibt es unter Ubuntu noch `snap`. Snaps sind in sich geschlossene Pakete, die unabhängig vom System-Paketverwaltung funktionieren.

```bash
# Snap-Paket suchen
snap find vlc

# Snap-Paket installieren
sudo snap install vlc

# Installierte Snaps auflisten
snap list

# Snap-Paket entfernen
sudo snap remove vlc
```

**Wann snap, wann apt?**
- `apt` ist die erste Wahl: schneller, besser integriert, kein Root-Container-Overhead
- `snap` kann sinnvoll sein, wenn eine neuere Version als im apt-Repo verfügbar ist (z.B. aktuelle VS Code-Version)
- Einige Programme werden offiziell nur als Snap angeboten

---

## Warum zufällige `curl | bash`-Skripte gefährlich sind

Im Internet findet man häufig Installationsanleitungen in dieser Form:

```bash
# NICHT einfach ausführen, ohne zu wissen, was es tut!
curl https://irgendeine-webseite.de/install.sh | bash
```

Dieser Befehl lädt ein Skript von einer URL herunter und führt es sofort aus – oft mit `sudo`, also als Root.

**Was kann dabei schiefgehen?**

- Das Skript kann beliebige Befehle ausführen: Dateien löschen, Passwörter auslesen, Hintertüren einrichten
- Die URL könnte kompromittiert sein (Angreifer übernehmen die Webseite)
- Du siehst nicht, was das Skript tut, bevor es ausgeführt wird
- Es gibt keine Integritätsprüfung wie bei signierten apt-Paketen

**Die sichere Alternative:**

```bash
# 1. Skript erst herunterladen
curl -o install.sh https://irgendeine-webseite.de/install.sh

# 2. Inhalt prüfen, bevor du es ausführst
cat install.sh
# oder mit einem Editor
nano install.sh

# 3. Nur ausführen, wenn du verstehst, was es tut
bash install.sh
```

**Faustregel:** Wenn ein Skript nicht von einem offiziellen Repository kommt und du nicht weißt, was es macht – führe es nicht aus. `apt` ist immer die sicherere Wahl, wenn das Programm dort verfügbar ist.

---

## Referenz: Häufige apt-Befehle

| Befehl | Was es macht |
|--------|--------------|
| `sudo apt update` | Paketliste aktualisieren |
| `sudo apt upgrade` | Alle Pakete aktualisieren |
| `sudo apt install <paket>` | Paket installieren |
| `sudo apt remove <paket>` | Paket entfernen (Konfig bleibt) |
| `sudo apt purge <paket>` | Paket und Konfig entfernen |
| `sudo apt autoremove` | Nicht mehr benötigte Pakete entfernen |
| `apt search <begriff>` | Nach Paketen suchen |
| `apt show <paket>` | Details zu einem Paket anzeigen |
| `apt list --installed` | Alle installierten Pakete anzeigen |

---

## Typischer Workflow

So sieht ein typischer Wartungs-Ablauf auf einem Ubuntu-Server aus:

```bash
# System aktuell halten
sudo apt update && sudo apt upgrade

# Aufräumen
sudo apt autoremove

# Ein neues Tool installieren
sudo apt install <paketname>
```

---

## Troubleshooting

**"E: Could not get lock /var/lib/dpkg/lock"**
Ein anderer apt-Prozess läuft noch (z.B. automatisches Update im Hintergrund). Warte kurz und versuche es erneut.

**"E: Unable to locate package xyz"**
- Zuerst `sudo apt update` ausführen
- Prüfen, ob der Paketname korrekt geschrieben ist: `apt search xyz`
- Das Paket könnte unter einem anderen Namen verfügbar sein

**"The following packages have been kept back"**
Manche Pakete werden von `apt upgrade` nicht automatisch aktualisiert, wenn neue Abhängigkeiten hinzukommen. Lösung:
```bash
sudo apt full-upgrade
```
