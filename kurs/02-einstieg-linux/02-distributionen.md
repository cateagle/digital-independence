# Linux-Distributionen

Linux selbst ist nur der [Kernel](../../wiki/glossar.md#kernel). Eine **[Distribution](../../wiki/glossar.md#distribution-linux)** (kurz: [Distro](../../wiki/glossar.md#distro)) packt den Kernel zusammen mit allem, was man zum Arbeiten braucht: Systemwerkzeuge, einen Paketmanager, vorinstallierte Software und oft eine grafische Oberfläche.

**Windows-Analogie:** Wenn Linux der Kernel ist, dann ist eine Distribution wie eine vollständige Windows-Edition, Windows Home, Pro oder Server. Alle basieren auf demselben Kern, unterscheiden sich aber in Ausstattung und Zielgruppe.

---

## Was steckt in einer Distribution?

```
Distribution (z.B. Ubuntu 24.04)
├── Linux-Kernel (z.B. 6.8)
├── Paketmanager (z.B. apt)
├── Systemdienste (systemd)
├── Standardwerkzeuge (bash, coreutils)
├── Optionale Desktop-Umgebung (GNOME, KDE, ...)
└── Vorinstallierte Anwendungen
```

Hier gehen wir primär auf Server Distributionen ein und übersehen auch für diesen Use Case bestimmt welche. Für Desktop Distributionen kannst zu z.B. hier mehr Infos bekommen:
- [distrochooser.de](https://distrochooser.de/de)

---

## Die wichtigsten Distributionen

### Ubuntu

Die beliebteste Einsteiger-Distribution. Basiert auf [Debian](../../wiki/glossar.md#debian), erscheint alle 2 Jahre als **[LTS](../../wiki/glossar.md#lts-long-term-support)-Version** (Long Term Support) mit 5 Jahren Support.

- **Paketmanager:** `apt`
- **Zielgruppe:** Einsteiger, Desktops, Server
- **Besonderheit:** Riesige Community, sehr viele Tutorials verfügbar
- **Server-Variante:** Ubuntu Server (ohne grafische Oberfläche)

```bash
# Pakete aktualisieren unter Ubuntu
sudo apt update && sudo apt upgrade
```

### Debian

Das Fundament, auf dem [Ubuntu](../../wiki/glossar.md#ubuntu) aufbaut. Gilt als besonders **stabil und konservativ**. Neue Software-Versionen kommen erst nach ausgiebigen Tests ins System.
- **Paketmanager:** `apt`
- **Zielgruppe:** Server, erfahrene Nutzer
- **Besonderheit:** Sehr langlebige Releases, minimale Installation möglich

### Fedora

Von Red Hat gesponsert und technisch oft vorn dabei. Nutzt neuere Software als Debian/Ubuntu. Red Hat bietet Enterprise Level Support.
- **Paketmanager:** `dnf`
- **Zielgruppe:** Entwickler, Desktop-Nutzer
- **Besonderheit:** Zeigt kommende RHEL-Features als Vorschau

```bash
# Pakete aktualisieren unter Fedora
sudo dnf upgrade
```

### Rocky Linux / AlmaLinux

Freie, community-gepflegte Nachfolger von CentOS. Kompatibel mit **Red Hat Enterprise Linux (RHEL)** dem Standard in Unternehmensumgebungen.

- **Paketmanager:** `dnf`
- **Zielgruppe:** Unternehmen, Server, Produktivsysteme
- **Besonderheit:** Binärkompatibel mit RHEL, sehr langer Support-Zeitraum

### Arch Linux

Ein minimalistisches System, das du vollständig selbst aufbaust. Kein Installer mit Standardpaketen. Du entscheidest über alles.

- **Paketmanager:** `pacman`
- **Zielgruppe:** Erfahrene Nutzer, die maximale Kontrolle wollen
- **Besonderheit:** Rolling Release (immer aktuellste Software), exzellente Dokumentation (Arch Wiki)

### Alpine Linux

Extrem klein und ressourcenschonend. Beliebt in **Docker-Containern**, weil Images damit winzig bleiben.

- **Paketmanager:** `apk`
- **Zielgruppe:** Container, eingebettete Systeme
- **Besonderheit:** Basis-Image nur ~5 MB groß

---

## Vergleichsübersicht

| Distribution | Paketmanager | Zielgruppe | Stabilität | Aktualisierungsmodell |
|---|---|---|---|---|
| Ubuntu LTS | apt | Einsteiger, Server | Hoch | Feste Releases |
| Debian | apt | Server, Experten | Sehr hoch | Feste Releases |
| Fedora | dnf | Entwickler | Mittel | Feste Releases |
| Rocky / Alma | dnf | Unternehmen | Sehr hoch | Feste Releases |
| Arch | pacman | Experten | Variabel | Rolling Release |
| Alpine | apk | Container | Hoch | Rolling Release |

---

## Welche Distro solltest du wählen?

Für den Einstieg ins Self-Hosting lautet die klare Empfehlung:

> **Ubuntu Server LTS** oder **Debian Stable**

Warum?

- Riesige Community → bei Problemen findet man sofort Hilfe
- Die meisten Tutorials im Internet beziehen sich auf `apt`-basierte Systeme
- Sehr guter Support-Zeitraum (5+ Jahre)
- Weit verbreitet bei [VPS](../../wiki/glossar.md#vps-virtual-private-server)-Anbietern (Hetzner, DigitalOcean, etc.)

---

## Tipp: Desktop vs. Server

Distributionen gibt es oft in zwei Varianten:

- **Desktop**: Mit grafischer Oberfläche (GNOME, KDE), für lokale Nutzung
- **Server**: Nur Terminal, kein Desktop -> spart Ressourcen und Angriffsfläche

Für Self-Hosting-Server wählst du immer die **Server-Variante**.
