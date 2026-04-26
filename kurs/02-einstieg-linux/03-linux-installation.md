# Linux installieren

Es gibt mehrere Wege, Linux auszuprobieren oder dauerhaft einzusetzen. Du musst nicht sofort deinen Rechner umbauen. Fang einfach dort an, wo du dich wohlfühlst.

---

## Deine Optionen im Überblick

| Methode                           | Aufwand     | Empfohlen für            |
|-----------------------------------|-------------|--------------------------|
| WSL (Windows Subsystem for Linux) | Sehr gering | Einsteiger unter Windows |
| Virtuelle Maschine (VM)           | Gering      | Ausprobieren ohne Risiko |
| Dual-Boot                         | Mittel      | Neben Windows nutzen     |
| Nur Linux                         | Hoch        | Vollständiger Umstieg    |
| VPS in der Cloud                  | Gering      | Server-Setup üben        |

---

## Option 1: WSL - Linux in Windows

Das **Windows Subsystem for Linux** lässt dich Linux-Befehle direkt in Windows ausführen, ohne etwas zu installieren oder umzupartitionieren.

### Installation (Windows 10/11)

```powershell
# In PowerShell als Administrator ausführen:
wsl --install
```

Das installiert automatisch Ubuntu. Nach einem Neustart startest du WSL über das Startmenü
oder mit:

```powershell
wsl
```

### WSL starten und prüfen

```bash
# Zeigt die Linux-Version an
cat /etc/os-release

# Zeigt das Arbeitsverzeichnis
pwd
```

**Tipp:** WSL eignet sich hervorragend, um Linux-Befehle zu üben. Für echtes Server-Hosting ist jedoch eine VM oder ein VPS besser geeignet.

---

## Option 2: Virtuelle Maschine (VM)

Eine VM läuft als Programm in deinem normalen Betriebssystem. Du kannst sie jederzeit pausieren, löschen oder neu erstellen ohne dein System zu riskieren.

### Empfohlene VM-Software

- **VirtualBox** kostenlos, plattformübergreifend
- **VMware Workstation Player** kostenlos für privaten Gebrauch
- **UTM** für macOS (auch Apple Silicon)

### Ubuntu Server in VirtualBox installieren

1. **ISO herunterladen**: [ubuntu.com/download/server](https://ubuntu.com/download/server)
2. **VirtualBox öffnen** → „Neu" klicken
3. **Einstellungen:**
   - Name: z.B. `ubuntu-server`
   - Typ: Linux, Version: Ubuntu (64-bit)
   - RAM: mindestens **2048 MB** (2 GB)
   - Festplatte: mindestens **20 GB** (dynamisch)
4. **ISO einlegen**: Unter „Massenspeicher" → optisches Laufwerk → ISO auswählen
5. **VM starten** → Installer folgen

---

## Option 3: Ubuntu Server installieren (Bare Metal oder VM)

Egal ob auf echter Hardware oder in einer VM. der Ubuntu-Server-Installer läuft gleich ab.

### Schritt-für-Schritt

```
1. Sprache wählen: English (empfohlen für Server um Fehlermeldungen besser googlen zu können)
2. Tastaturlayout: German
3. Netzwerk: Standard übernehmen (DHCP)
4. Speicher: "Use entire disk" für eine einfache Partitionierung
5. Benutzerprofil:
   - Name:          dein Name
   - Servername:    z.B. myserver
   - Benutzername:  z.B. admin
   - Passwort:      sicheres Passwort wählen
6. OpenSSH installieren: JA (wichtig für späteren SSH-Zugriff)
7. Snaps: keine auswählen, einfach überspringen
8. Installation abwarten → Neustart
```

### Erster Login

Nach dem Neustart erscheint ein Login-Prompt im Terminal:

```
myserver login: admin
Password: ████████

Welcome to Ubuntu 24.04 LTS
```

```bash
# Immer zuerst System aktualisieren!
sudo apt update && sudo apt upgrade -y

# Aktuellen Benutzer anzeigen
whoami

# Systeminfo anzeigen
uname -a
```

---

## Option 4: VPS in der Cloud

Ein **[Virtual Private Server (VPS)](../../wiki/glossar.md#vps-virtual-private-server)** ist ein Linux-Server, der bei einem Anbieter läuft.
Du bezahlst monatlich und hast vollen Root-Zugriff.

### Anbieter

z.B.:
- Hetzner
- Ionos
- Netcup
- DigitalOcean
...

### VPS einrichten

```bash
# Vom lokalen Rechner per SSH verbinden
ssh root@<IP-ADRESSE>

# System sofort aktualisieren
apt update && apt upgrade -y

# Neuen Benutzer anlegen (Root direkt nutzen ist keine gute Praxis)
adduser admin
usermod -aG sudo admin
```

Mehr zum Thema VPS findest du unter [../../wiki/server-and-network/06-vps.md](../../wiki/server-and-network/06-vps.md).

---

## Nach der Installation: Erste Schritte

Unabhängig von der Methode solltest du nach der Installation Folgendes tun:

```bash
# 1. System aktualisieren
sudo apt update && sudo apt upgrade -y

# 2. Zeitzone setzen
sudo timedatectl set-timezone Europe/Berlin

# 3. Hostname prüfen
hostnamectl

# 4. Firewall aktivieren (UFW)
sudo ufw allow OpenSSH
sudo ufw enable
sudo ufw status
```

---

## Häufige Probleme

**"sudo: command not found"**
→ Du bist als root eingeloggt. `sudo` brauchst du nur als normaler Benutzer.

**Kein Internetzugang in der VM**
→ Netzwerkadapter in den VM-Einstellungen auf „NAT" oder „Netzwerkbrücke" setzen.

**SSH-Verbindung schlägt fehl**
→ Prüfe, ob OpenSSH installiert und der Port 22 in der [Firewall](../../wiki/glossar.md#firewall) freigegeben ist:

```bash
sudo systemctl status ssh
sudo ufw allow OpenSSH
```
