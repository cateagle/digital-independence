# 09 - Updates & Sicherheitsgrundlagen

Ein Server, der nicht aktualisiert wird, ist wie eine Haustür mit einem bekannten kaputten Schloss. Jemand irgendwo weiß, wie man es öffnet — es ist nur eine Frage der Zeit. Dieses Kapitel erklärt, warum Updates so wichtig sind, wie du sie automatisierst und wie du überprüfst, ob dein Server unnötig angreifbar ist.

---

## Warum Updates wichtig sind

Software hat Fehler. Manche dieser Fehler sind harmlos, aber manche öffnen Sicherheitslücken, durch die Angreifer in dein System eindringen können.

**Windows-Analogie:** Windows Update existiert aus genau demselben Grund. Der Unterschied: Unter Linux hast du viel mehr Kontrolle darüber, was wann aktualisiert wird — und du bist dafür verantwortlich, es zu tun.

Der typische Ablauf einer Sicherheitslücke:
1. Jemand entdeckt eine Schwachstelle in einem Programm.
2. Die Schwachstelle wird als [CVE](../glossar.md#cve-common-vulnerabilities-and-exposures) veröffentlicht (dazu gleich mehr).
3. Die Software-Entwickler veröffentlichen ein Update mit einer Korrektur.
4. Das Update landet in den Ubuntu-Paketquellen.
5. Du installierst das Update — oder du tust es nicht, und dein Server bleibt verwundbar.

**Grundregel:** Sicherheitsupdates immer zeitnah einspielen. Feature-Updates können warten.

---

## apt update und apt upgrade — der manuelle Weg

Bevor du alles automatisierst, solltest du verstehen, wie es manuell funktioniert.

```bash
sudo apt update
```

Dieser Befehl holt die aktuelle Liste der verfügbaren Pakete von den Paketquellen. Er installiert noch nichts — er fragt nur nach, was es Neues gibt.

```bash
sudo apt upgrade
```

Dieser Befehl installiert alle verfügbaren Updates für bereits installierte Pakete. Neue Pakete werden nicht hinzugefügt, keine alten werden entfernt.

```bash
sudo apt full-upgrade
```

Wie `upgrade`, aber erlaubt auch das Entfernen von veralteten Paketen, falls nötig. Für Sicherheitsupdates ist `upgrade` meistens ausreichend.

**Tipp:** `apt update` und `apt upgrade` immer zusammen ausführen. Nur `apt upgrade` ohne vorheriges `apt update` installiert veraltete Pakete aus dem lokalen Cache.

---

## Sicherheits-Updates vs. Feature-Updates

Nicht alle Updates sind gleich. Ubuntu unterscheidet:

| Art | Beschreibung | Dringlichkeit |
|-----|--------------|---------------|
| Sicherheitsupdates | Beheben bekannte Sicherheitslücken | Hoch — so schnell wie möglich |
| Bugfix-Updates | Beheben Programmfehler ohne Sicherheitsbezug | Mittel |
| Feature-Updates | Neue Funktionen, neue Versionen | Niedrig — erst testen |

**Warum dieser Unterschied wichtig ist:**

Feature-Updates können Dinge kaputt machen. Eine neue Hauptversion von nginx, PostgreSQL oder Python kann sich anders verhalten als die alte. Auf einem Produktivserver willst du nicht überraschend feststellen, dass deine Anwendung nach einem Update nicht mehr funktioniert.

Sicherheitsupdates hingegen sind in der Regel kleine, gezielte Korrekturen. Ubuntu backported Sicherheitsfixes, d.h. sie werden in die bestehende Version eingebaut, ohne die Gesamtfunktionalität zu ändern.

**Nur Sicherheitsupdates installieren:**

```bash
# Vorschau: was würde installiert werden?
sudo unattended-upgrade --dry-run

# Sicherheitsupdates tatsächlich einspielen:
sudo unattended-upgrade
```

Oder automatisch mit `unattended-upgrades` (siehe nächsten Abschnitt).

---

## CVEs — Was ist das?

CVE steht für **Common Vulnerabilities and Exposures** (Bekannte Schwachstellen und Anfälligkeiten). Es ist eine öffentliche Datenbank, in der jede bekannte Sicherheitslücke eine eindeutige ID bekommt.

**Beispiel einer CVE-ID:**

```
CVE-2024-3094
```

Das Format: `CVE-` gefolgt vom Jahr und einer laufenden Nummer.

**Was du damit machen kannst:**

- Auf [cve.mitre.org](https://cve.mitre.org) oder [nvd.nist.gov](https://nvd.nist.gov) nachschauen, was eine bestimmte Schwachstelle bedeutet.
- Prüfen, ob dein System betroffen ist.
- Auf [ubuntu.com/security/cves](https://ubuntu.com/security/cves) nachschauen, welche CVEs Ubuntu betreffen und ob bereits ein Fix verfügbar ist.

**Schweregrad (CVSS-Score):**

CVEs bekommen einen Schweregrad von 0 bis 10:
- 0–3.9: Niedrig
- 4.0–6.9: Mittel
- 7.0–8.9: Hoch
- 9.0–10.0: Kritisch

Ein Score von 9.8 bedeutet: Sofort patchen.

**Tipp:** Du musst CVEs nicht täglich verfolgen. Wenn du `unattended-upgrades` richtig einrichtest (nächster Abschnitt), werden Sicherheitsfixes automatisch eingespielt.

---

## unattended-upgrades — Automatische Sicherheitsupdates

`unattended-upgrades` ist ein Ubuntu-Paket, das automatisch Sicherheitsupdates im Hintergrund einspielt — ohne dass du jedes Mal manuell eingreifen musst.

**Windows-Analogie:** Wie "Updates automatisch installieren" in den Windows Update-Einstellungen, aber nur für Sicherheitsupdates.

### Installation

```bash
sudo apt install unattended-upgrades
```

Auf Ubuntu 24.04 ist es oft bereits vorinstalliert. Prüfen:

```bash
dpkg -l unattended-upgrades
```

### Aktivieren

```bash
sudo dpkg-reconfigure --priority=low unattended-upgrades
```

Es erscheinen zwei Fragen — beide mit "Ja" beantworten.

Alternativ direkt:

```bash
sudo systemctl enable unattended-upgrades
sudo systemctl start unattended-upgrades
```

### Konfiguration anschauen

Die Hauptkonfigurationsdatei:

```bash
sudo nano /etc/apt/apt.conf.d/50unattended-upgrades
```

Der wichtigste Teil ist die Liste der erlaubten Paketquellen:

```
Unattended-Upgrade::Allowed-Origins {
    "${distro_id}:${distro_codename}";
    "${distro_id}:${distro_codename}-security";
    // "${distro_id}:${distro_codename}-updates";
    // "${distro_id}:${distro_codename}-proposed";
    // "${distro_id}:${distro_codename}-backports";
};
```

- Die Zeile mit `-security` ist aktiv (kein `//` davor): Sicherheitsupdates werden automatisch eingespielt.
- Die Zeile mit `-updates` ist auskommentiert: Normale Feature-Updates werden nicht automatisch eingespielt.

**Empfehlung für Anfänger:** Standardeinstellungen beibehalten. Nur `-security` automatisieren, alles andere manuell prüfen.

### Automatischer Neustart (optional)

Manche Updates (z.B. Kernel-Updates) erfordern einen Neustart. Du kannst `unattended-upgrades` so konfigurieren, dass es nachts automatisch neu startet:

In `/etc/apt/apt.conf.d/50unattended-upgrades`:

```
Unattended-Upgrade::Automatic-Reboot "true";
Unattended-Upgrade::Automatic-Reboot-Time "03:00";
```

**Warnung:** Automatischen Neustart nur aktivieren, wenn du sicher bist, dass deine Dienste danach automatisch wieder starten (was mit `systemctl enable` sichergestellt wird).

### Prüfen ob es funktioniert

```bash
sudo unattended-upgrade --dry-run --debug
```

`--dry-run` führt nichts wirklich aus, zeigt dir aber, was es tun würde.

Logs anschauen:

```bash
cat /var/log/unattended-upgrades/unattended-upgrades.log
```

---

## Offene Ports prüfen mit ss -tulnp

Ein [Port](../glossar.md#port) ist wie eine Tür in dein System. Je weniger Türen offen sind, desto kleiner ist die Angriffsfläche. Mit `ss` kannst du sehen, welche Ports auf deinem Server lauschen.

**Windows-Analogie:** Wie `netstat -ano` in der Windows-Eingabeaufforderung, aber moderner und übersichtlicher.

### Der Befehl

```bash
sudo ss -tulnp
```

Was die Optionen bedeuten:
- `-t` — TCP-Verbindungen anzeigen
- `-u` — UDP-Verbindungen anzeigen
- `-l` — Nur lauschende (listening) Ports anzeigen
- `-n` — Keine Namensauflösung, zeige Zahlen (schneller)
- `-p` — Zeige den Prozess, der den Port benutzt

### Beispielausgabe

```
Netid  State   Recv-Q  Send-Q  Local Address:Port  Peer Address:Port  Process
tcp    LISTEN  0       128     0.0.0.0:22           0.0.0.0:*          users:(("sshd",pid=1234,fd=3))
tcp    LISTEN  0       511     0.0.0.0:80           0.0.0.0:*          users:(("nginx",pid=5678,fd=6))
tcp    LISTEN  0       511     127.0.0.1:5432       0.0.0.0:*          users:(("postgres",pid=9012,fd=7))
```

### Ausgabe lesen

| Spalte | Bedeutung |
|--------|-----------|
| `Netid` | Protokoll (tcp oder udp) |
| `State` | LISTEN = wartet auf Verbindungen |
| `Local Address:Port` | Auf welcher IP und welchem Port gelauscht wird |
| `Process` | Welcher Dienst den Port nutzt |

**Wichtig: Local Address verstehen**

- `0.0.0.0:22` — Port 22 (SSH) ist von überall erreichbar (alle Netzwerkinterfaces)
- `127.0.0.1:5432` — Port 5432 (PostgreSQL) ist nur lokal erreichbar, von außen nicht sichtbar
- `[::]:80` — Port 80 ist auch über IPv6 erreichbar

### Was du sehen solltest (und was nicht)

**Erwartete offene Ports auf einem frischen Server:**
- Port 22 ([SSH](../glossar.md#ssh-secure-shell)) — zum Einloggen
- Port 80/443 — nur wenn du einen Webserver betreibst

**Warnsignale:**
- Unbekannte Prozesse auf hohen Portnummern
- Dienste wie Datenbanken (Port 3306, 5432) auf `0.0.0.0` statt `127.0.0.1`
- Ports, die du nicht erkennst und die du nicht selbst installiert hast

**Datenbank-Ports niemals nach außen öffnen:**

```bash
# Schlecht — PostgreSQL von überall erreichbar:
0.0.0.0:5432

# Gut — PostgreSQL nur lokal erreichbar:
127.0.0.1:5432
```

Wenn eine Datenbank auf `0.0.0.0` lauscht, kann sie von jedem im Internet angesprochen werden. Viele automatisierte Angriffe suchen genau danach.

---

## Sicherheits-Checkliste für einen frischen Server

Gehe diese Liste durch, wenn du einen neuen Ubuntu-Server einrichtest:

```bash
# 1. System aktualisieren
sudo apt update && sudo apt upgrade -y

# 2. unattended-upgrades installieren und aktivieren
sudo apt install unattended-upgrades
sudo dpkg-reconfigure --priority=low unattended-upgrades

# 3. Offene Ports prüfen
sudo ss -tulnp

# 4. Nur SSH und benötigte Dienste sollten nach außen offen sein
# (Datenbanken, interne Dienste: nur auf 127.0.0.1)
```

---

## Troubleshooting

**F: `unattended-upgrades` läuft, aber ich sehe keine Logs.**

Der Dienst schreibt nur dann etwas ins Log, wenn er tatsächlich Updates gefunden und installiert hat. Prüfe:

```bash
systemctl status unattended-upgrades
```

**F: `ss -tulnp` zeigt einen Port, den ich nicht kenne. Was tun?**

Schau dir den Prozessnamen an und suche danach:

```bash
# Welcher Prozess benutzt Port 8080?
sudo ss -tulnp | grep :8080

# Mehr Infos über den Prozess
ps aux | grep <prozessname>
```

**F: Ich habe nach einem Update etwas kaputt gemacht. Wie finde ich heraus, was sich geändert hat?**

```bash
# Letzte installierte/geänderte Pakete anzeigen
grep " install\| upgrade" /var/log/dpkg.log | tail -20
```

---

## Nützliche Links

- [Ubuntu Security Notices](https://ubuntu.com/security/notices)
- [Ubuntu CVE-Übersicht](https://ubuntu.com/security/cves)
- [Unattended-Upgrades Dokumentation](https://help.ubuntu.com/community/AutomaticSecurityUpdates)
- [NVD — National Vulnerability Database](https://nvd.nist.gov)
