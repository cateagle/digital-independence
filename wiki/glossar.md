# Glossar

- [**A...**](#a)
- [**B...**](#b)
- [**C...**](#c)
- [**D...**](#d)
- [**E...**](#e)
- [**F...**](#f)
- [**G...**](#g)
- [**H...**](#h)
- [**I...**](#i)
- [**J...**](#j)
- [**K...**](#k)
- [**L...**](#l)
- [**M...**](#m)
- [**N...**](#n)
- [**O...**](#o)
- [**P...**](#p)
- [**Q...**](#q)
- [**R...**](#r)
- [**S...**](#s)
- [**T...**](#t)
- [**U...**](#u)
- [**V...**](#v)
- [**W...**](#w)
- [**X...**](#x)
- [**Y...**](#y)
- [**Z...**](#z)

## A

### Alias (Shell-Alias)

Ein benutzerdefinierter Kurzname für einen längeren Befehl oder eine Befehlskette. Aliase werden in der `.bashrc` definiert und erleichtern die tägliche Arbeit mit der Shell erheblich.

Siehe auch: [Die Linux Shell](./linux_basics/02-linux-shell.md)

### Arch Linux

Eine minimalistische Linux-Distribution, die vollständig manuell aufgebaut wird und immer die neueste Software liefert (Rolling Release). Arch richtet sich an erfahrene Nutzer, die maximale Kontrolle über ihr System wollen.

Siehe auch: [Linux-Distributionen](../kurs/02-einstieg-linux/02-distributionen.md)

### ACME (Automated Certificate Management Environment)

Ein offenes Protokoll für die automatisierte Verwaltung von SSL/TLS-Zertifikaten. Let's Encrypt nutzt ACME, um Zertifikate ohne manuelle Interaktion auszustellen und zu erneuern.

Siehe auch: [Nginx Grundlagen](./components/03-nginx-grundlagen.md)

### ACME-Challenge

Ein Verfahren zur Validierung der Domain-Inhaberschaft bei der Zertifikatausstellung. Certbot beweist gegenüber Let's Encrypt, dass man die Domain kontrolliert, indem man ein Token an einer bekannten URL (`.well-known/acme-challenge/`) bereitstellt.

Siehe auch: [SSL und Let's Encrypt](./server-and-network/04-ssl-letsencrypt.md)

### ACL (Access Control List)

Erweiterte Berechtigungsverwaltung für Dateien und Verzeichnisse, die granularere Kontrolle als Standard-Unix-Berechtigungen ermöglicht. ACLs erlauben es, spezifischen Benutzern und Gruppen unabhängig voneinander Zugriff zu gewähren oder zu verweigern.

Siehe auch: [Benutzer, Gruppen und Berechtigungen](./linux_basics/03-users-groups-permissions.md)

### AGPL (GNU Affero General Public License)
*Abkürzung: AGPLv3*

Die stärkste Variante der GPL-Lizenzen, die auch SaaS-Dienste in den Geltungsbereich einbezieht. Wer AGPL-Software über ein Netzwerk anbietet (z.B. als Web-App), muss den Quellcode ebenfalls veröffentlichen. Relevant für Self-Hosting, da AGPL verhindert, dass Cloud-Anbieter proprietäre Vorteile aus Open-Source-Software ziehen.

Siehe auch: [Open-Source-Lizenzen](../kurs/01-einstieg-open-source/03-lizenzen.md)

### AIO (All-in-One)

Ein von Nextcloud entwickeltes „Mastercontainer"-Image, das alle notwendigen Komponenten wie Nextcloud, Datenbank, Redis, Reverse Proxy und optionale Services selbst verwaltet und konfiguriert. AIO reduziert den operativen Aufwand beim Self-Hosting drastisch, da die Komponenten bereits aufeinander abgestimmt sind.

Siehe auch: [Nextcloud AIO Setup](../kurs/03-filesharing-und-groupware/02-nextcloud-aio-setup.md)

### Allowlist
*Synonym: Whitelist*

Ein Sicherheitskonzept, bei dem nur explizit erlaubte Verbindungen, Ports oder IP-Adressen akzeptiert werden und alles andere blockiert wird. Ist sicherer als eine Blocklist, da unerwartete Lücken verhindert werden.

Siehe auch: [Firewall & UFW](./server-hardening/01-firewall-ufw.md), [Whitelist](#whitelist)

### AOF (Append Only File)

Eine Redis-Persistenzmethode, die jeden einzelnen Schreibbefehl in eine Logdatei schreibt. Bei einem Neustart wird diese Datei wiedergegeben. AOF bietet minimalen Datenverlust, erzeugt aber größere Dateien als RDB.

Siehe auch: [Redis Grundlagen](./components/04-redis-grundlagen.md)

### Apache 2.0 (Apache License 2.0)

Permissive Open-Source-Lizenz, ähnlich wie MIT, aber mit explizitem Patent-Grant. Der Lizenzgeber gewährt Nutzern eine Lizenz für alle Patente, die in der Software stecken, was verhindert, dass Maintainer später Patente geltend machen.

Siehe auch: [Open-Source-Lizenzen](../kurs/01-einstieg-open-source/03-lizenzen.md)

### APT (Advanced Package Tool)

Der Standard-Paketmanager für Debian und Ubuntu, mit dem Softwarepakete installiert, aktualisiert und entfernt werden. APT verwaltet Abhängigkeiten automatisch und lädt Pakete aus vertrauenswürdigen Repositories.

Siehe auch: [Paketverwaltung](./linux_basics/04-paketverwaltung.md)

### Asymmetrische Verschlüsselung

Ein Verschlüsselungsverfahren, das zwei unterschiedliche Schlüssel verwendet: einen öffentlichen zum Verschlüsseln und einen privaten zum Entschlüsseln (oder umgekehrt). Ermöglicht sichere Kommunikation ohne vorherigen Austausch eines gemeinsamen Geheimnisses.

Siehe auch: [PGP Cheat Sheet](./tools/pgp-cheat-sheet.md), [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

## B

### Base Image

Das Ausgangs-Image, auf dem ein Docker Image aufbaut, angegeben in der `FROM`-Zeile eines Dockerfiles. Gängige Base Images sind `ubuntu`, `debian`, `alpine` oder `scratch`. Alpine-basierte Images sind besonders klein und daher im Self-Hosting beliebt.

Siehe auch: [Docker](./container/docker.md)

### Bootloader

Ein kleines Programm, das beim Start des Computers zuerst ausgeführt wird und den Betriebssystem-Kernel lädt. GRUB ist der am weitesten verbreitete Bootloader auf Linux-Systemen.

Siehe auch: [Was ist Linux?](../kurs/02-einstieg-linux/01-was-ist-linux.md)

### Bash

Die am weitesten verbreitete Shell auf Linux-Systemen, in der man Befehle eingibt und Shell-Skripte schreibt. Bash steht für „Bourne Again Shell" und ist der Standard auf Ubuntu und Debian.

Siehe auch: [Die Linux Shell](./linux_basics/02-linux-shell.md)

### Befehlszeilenschnittstelle
*→ Siehe [CLI (Command-Line Interface)](#cli-command-line-interface)*

### Blacklist
*→ Siehe [Blocklist](#blocklist)*

### Blocklist
*Synonym: Blacklist*

Ein Sicherheitskonzept, bei dem spezifische IPs, Ports oder Services blockiert werden, während alles andere erlaubt ist. Weniger sicher als eine Allowlist, da neue Bedrohungen nicht automatisch blockiert werden.

Siehe auch: [Firewall & UFW](./server-hardening/01-firewall-ufw.md), [Blacklist](#blacklist)

### Bridge-Netzwerk (Docker)

Das Standard-Netzwerkmodus für Docker-Container, bei dem Docker ein internes virtuelles Netzwerk erstellt. Container im selben Bridge-Netzwerk können sich gegenseitig über ihren Service-Namen ansprechen, sind aber vom Host isoliert.

Siehe auch: [Docker Compose](./container/docker-compose.md)

### Build-Kontext

Der Verzeichnis-Pfad oder der Inhalt, den Docker beim Bauen eines Images an den Docker-Daemon überträgt. Der Build-Kontext enthält alle Dateien, die per `COPY` oder `ADD` in das Image kopiert werden können.

Siehe auch: [Docker Compose](./container/docker-compose.md)

### Brute-Force-Angriff

Ein Angriff, bei dem ein Angreifer systematisch viele Passwortkombinationen durchprobiert, um Zugang zu einem Konto zu erlangen. Fail2ban sperrt IPs automatisch, wenn zu viele fehlgeschlagene Versuche erkannt werden.

Siehe auch: [Fail2Ban](./server-hardening/02-fail2ban.md), [SSH](./server-and-network/03-ssh.md)

### BSD (Berkeley Software Distribution)

Familie sehr permissiver Open-Source-Lizenzen (2-Clause und 3-Clause). Funktionieren ähnlich permissiv wie MIT und wurden von FreeBSD und OpenBSD geprägt.

Siehe auch: [Open-Source-Lizenzen](../kurs/01-einstieg-open-source/03-lizenzen.md)

## C

### CA (Certificate Authority)
*Deutsch: Zertifizierungsstelle*

Eine vertrauenswürdige Organisation, die digitale Zertifikate ausstellt und deren Echtheit mit ihrer Signatur bestätigt. Browser haben Root-Zertifikate von bekannten CAs vorinstalliert und vertrauen so automatisch deren ausgestellten Zertifikaten.

Siehe auch: [SSL und Let's Encrypt](./server-and-network/04-ssl-letsencrypt.md), [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md), [Zertifizierungsstelle](#zertifizierungsstelle)

### Cache

Ein schneller Zwischenspeicher, der häufig benötigte Daten im RAM ablegt. Redis wird oft als Cache eingesetzt, um teure Datenbankabfragen zu vermeiden und Antwortzeiten drastisch zu verkürzen.

Siehe auch: [Redis Grundlagen](./components/04-redis-grundlagen.md)

### Closed Source
*→ Siehe [Proprietäre Software](#proprietäre-software)*

### CalDAV

Ein offener Standard für die Synchronisierung von Kalendern über das Internet. Nextcloud Kalender nutzt CalDAV, um Termine zwischen verschiedenen Geräten und Anwendungen abzustimmen.

Siehe auch: [Nextcloud Apps & Plugins](../kurs/03-filesharing-und-groupware/04-plugins.md)

### CardDAV

Ein offener Standard für die Synchronisierung von Kontakten über das Internet. Nextcloud Kontakte implementiert CardDAV, um Adressbücher mit anderen Geräten zu synchronisieren.

Siehe auch: [Nextcloud Apps & Plugins](../kurs/03-filesharing-und-groupware/04-plugins.md)

### Certbot

Das offizielle Kommandozeilen-Werkzeug der EFF zum automatischen Beantragen, Installieren und Erneuern von Let's Encrypt-Zertifikaten. Kann die Konfiguration von Webservern wie Nginx und Apache automatisch anpassen.

Siehe auch: [SSL und Let's Encrypt](./server-and-network/04-ssl-letsencrypt.md), [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

### CIFS
*→ Siehe [SMB (Server Message Block)](#smb-server-message-block)*

### CLI (Command-Line Interface)
*Deutsch: Befehlszeilenschnittstelle*

Eine textbasierte Schnittstelle, über die man mit dem System kommuniziert, indem man Befehle tippt. Im Gegensatz zur GUI ist die CLI präziser, scriptbar und auf Servern die Regel.

Siehe auch: [Die Linux Shell](./linux_basics/02-linux-shell.md), [Befehlszeilenschnittstelle](#befehlszeilenschnittstelle)

### CNAME (Canonical Name)

Ein DNS-Eintrag, der eine Domain als Alias auf eine andere Domain verweist, statt direkt auf eine IP-Adresse. Nützlich, um mehrere Subdomains auf denselben Zieldienst zu leiten.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

### Compose File

Die YAML-Konfigurationsdatei (`docker-compose.yml` oder `compose.yml`) für Docker Compose. Sie beschreibt alle Services, Netzwerke und Volumes einer Anwendung in einem einzigen deklarativen Dokument.

Siehe auch: [Docker Compose](./container/docker-compose.md), [YAML](#yaml-yet-another-markup-language)

### Container Registry

Ein Dienst zum Speichern und Verteilen von Docker Images. Docker Hub ist die bekannteste öffentliche Registry; GitHub Container Registry (ghcr.io) und selbst gehostete Registries wie Harbor sind Alternativen. Nextcloud AIO wird z.B. von `ghcr.io` bezogen.

Siehe auch: [Docker](./container/docker.md)

### Collabora Online

Ein LibreOffice-basierter Office-Editor, der als Nextcloud-App in den Browser integriert wird. Collabora ermöglicht das Bearbeiten von Textdokumenten, Tabellen und Präsentationen direkt in Nextcloud.

Siehe auch: [Nextcloud Apps & Plugins](../kurs/03-filesharing-und-groupware/04-plugins.md)

### Container

Ein leichtgewichtiges, isoliertes Laufzeitumfeld, das eine Anwendung mit allen ihren Abhängigkeiten bündelt. Container laufen auf demselben Betriebssystem-Kernel, sind aber voneinander getrennt.

Siehe auch: [Docker](./container/docker.md)

### Copyleft

Designprinzip von Open-Source-Lizenzen (GPL, AGPL, LGPL), das verlangt, dass abgeleitete Werke unter der gleichen Lizenz veröffentlicht werden. Verhindert die Privatisierung von gemeinschaftlich entwickelter Software.

Siehe auch: [Open-Source-Lizenzen](../kurs/01-einstieg-open-source/03-lizenzen.md)

### Cron / Crontab

Ein zeitgesteuerter Aufgaben-Planer auf Linux, der Befehle automatisch zu definierten Zeiten ausführt. Die Konfiguration erfolgt in der Crontab-Datei mit einem eigenen Zeitformat (Minute, Stunde, Tag, Monat, Wochentag).

Siehe auch: [Cron – Geplante Aufgaben](./linux_basics/06-cron-geplante-aufgaben.md)

### CSR (Certificate Signing Request)

Eine Anfrage an eine CA, ein Zertifikat auszustellen. Der CSR enthält den öffentlichen Schlüssel und Informationen über den Antragsteller. Certbot generiert dies automatisch.

Siehe auch: [SSL und Let's Encrypt](./server-and-network/04-ssl-letsencrypt.md)

### CVE (Common Vulnerabilities and Exposures)

Ein standardisiertes System zur Identifizierung und Dokumentation von Sicherheitslücken in Software (z.B. CVE-2021-44228 für Log4Shell). CVE-Nummern ermöglichen schnelle, eindeutige Kommunikation über bekannte Schwachstellen.

Siehe auch: [Risiken bei der Nutzung von Open Source](../kurs/01-einstieg-open-source/06-open-source-risiken.md)

## D

### Dateirechte (chmod / chown)

Das Berechtigungssystem von Linux, mit dem festgelegt wird, wer eine Datei lesen, schreiben oder ausführen darf. `chmod` ändert die Berechtigungsbits (z.B. `755`), `chown` ändert Eigentümer und Gruppe einer Datei.

Siehe auch: [Benutzer, Gruppen und Berechtigungen](./linux_basics/03-users-groups-permissions.md)

### dpkg (Debian Package)

Das Low-Level-Paketverwaltungswerkzeug unter Debian und Ubuntu, auf dem APT aufsetzt. Während APT Abhängigkeiten automatisch auflöst und Pakete aus Repositories lädt, installiert `dpkg` lokale `.deb`-Dateien direkt.

Siehe auch: [Paketverwaltung](./linux_basics/04-paketverwaltung.md)

### Daemon

Ein Hintergrundprozess auf Linux, der ohne Benutzerinteraktion läuft und kontinuierlich einen Dienst erbringt. Der SSH-Server (`sshd`), Nginx und systemd sind Beispiele für Daemons.

Siehe auch: [Prozesse & Systemüberwachung](./linux_basics/05-prozesse-systemueberwachung.md)

### DDNS (Dynamic DNS)

Ein Dienst, der einen Domainnamen automatisch aktualisiert, wenn sich die IP-Adresse des Servers ändert. Relevant für Self-Hosting zu Hause, wo Internetanbieter dynamische IPs vergeben.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

### Datenbank-Migration

Eine versionierte Änderung am Datenbankschema (z.B. neue Tabellen oder Spalten), die automatisch beim Anwendungsupdate ausgeführt wird. Nextcloud führt Migrationen bei Updates automatisch aus; SQLite kann dabei Probleme bereiten.

Siehe auch: [Datenbanken bei Nextcloud](../kurs/03-filesharing-und-groupware/03-datenbanken.md)

### Data Sovereignty
*→ Siehe [Datensouveränität](#datensouveränität)*

### Datensouveränität
*Englisch: Data Sovereignty*

Das Recht und die Kontrolle über die eigenen Daten. Self-Hosting ermöglicht Datensouveränität, indem Daten auf dem eigenen Server verbleiben statt bei Cloud-Anbietern.

Siehe auch: [Was ist Nextcloud?](../kurs/03-filesharing-und-groupware/01-nextcloud.md), [Data Sovereignty](#data-sovereignty)

### Data Minimization
*→ Siehe [Datensparsamkeit](#datensparsamkeit)*

### Datensparsamkeit
*Englisch: Data Minimization*

Prinzip, nur die notwendigsten persönlichen Daten zu erheben und zu speichern. Ein wichtiger Teil bewusster digitaler Unabhängigkeit auf individueller Ebene.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md), [Data Minimization](#data-minimization)

### Data Protection
*→ Siehe [Datenschutz](#datenschutz)*

### Datenschutz
*Englisch: Data Protection*

Rechtliches und ethisches Konzept zum Schutz persönlicher Daten. Umfasst auch Datensparsamkeit und ist zentral für digitale Unabhängigkeit.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md), [Data Protection](#data-protection)

### Dateisystem

Die Struktur, nach der ein Betriebssystem Daten auf Speichermedien organisiert. Unter Linux ist alles ein Verzeichnis, beginnend beim Root-Verzeichnis `/`.

Siehe auch: [Dateisystem einbinden](./linux_basics/08-dateisystem-einhaengen.md)

### Debian

Eine Linux-Distribution und die Basis für Ubuntu. Viele Befehle in diesem Kurs (apt, systemctl) funktionieren gleich auf Debian und Ubuntu.

Siehe auch: [Linux-Distributionen](../kurs/02-einstieg-linux/02-distributionen.md)

### Decentralization
*→ Siehe [Dezentralisierung](#dezentralisierung)*

### Dezentralisierung
*Englisch: Decentralization*

Architekturprinzip, bei dem Kontrolle und Verarbeitung auf mehrere unabhängige Knoten verteilt werden, statt auf einen zentralen Punkt zu konzentrieren. Grundlage für dezentrale Dienste wie das Fediverse.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md), [Decentralization](#decentralization)

### Digital Independence
*→ Siehe [Digitale Unabhängigkeit](#digitale-unabhängigkeit)*

### Digitale Unabhängigkeit
*Englisch: Digital Independence*

Zustand, bei dem Individuen, Communities oder Organisationen bewusste Kontrolle über ihre digitale Infrastruktur, Daten und Software haben. Zentrales Thema dieses Kurses.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md), [Digital Independence](#digital-independence)

### Digital Sovereignty
*→ Siehe [Digitale Souveränität](#digitale-souveränität)*

### Digitale Souveränität
*Englisch: Digital Sovereignty*

Fähigkeit von Individuen, Organisationen oder Staaten, eigenständig über ihre digitale Infrastruktur und die damit verbundenen Entscheidungen zu bestimmen.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md), [Digital Sovereignty](#digital-sovereignty)

### Distribution (Linux)
*Kurzform: Distro*

Ein vollständiges Betriebssystem auf Basis des Linux-Kernels, gebündelt mit Software, Paketmanager und Konfiguration. Ubuntu, Debian, Arch Linux und Fedora sind bekannte Distributionen.

Siehe auch: [Linux-Distributionen](../kurs/02-einstieg-linux/02-distributionen.md), [Distro](#distro)

### Distro
*→ Siehe [Distribution (Linux)](#distribution-linux)*

### DNS (Domain Name System)

Das „Telefonbuch des Internets", das menschenlesbare Domainnamen (z.B. `meinserver.de`) in IP-Adressen übersetzt. Beim Self-Hosting konfiguriert man DNS-Einträge beim Domainregistrar, damit der eigene Domainname auf den Server zeigt.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

### DNS-Challenge

Ein Verfahren zur Domain-Validierung bei Let's Encrypt, bei dem Certbot einen TXT-Record in den DNS-Einträgen der Domain ablegt. Ermöglicht Zertifikate ohne laufenden Webserver.

Siehe auch: [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

### Docker

Eine Containerisierungsplattform, die es ermöglicht, Anwendungen in isolierten, portierbaren Containern zu verpacken. Docker ist das Standard-Werkzeug für Self-Hosting und wird von fast allen gängigen Open-Source-Diensten unterstützt.

Siehe auch: [Docker](./container/docker.md)

### Docker Compose

Ein Tool zum Definieren und Ausführen von Multi-Container-Docker-Anwendungen über eine einzige YAML-Datei. Mit Docker Compose werden Netzwerke, Volumes und Abhängigkeiten zwischen Containern deklarativ konfiguriert.

Siehe auch: [Docker Compose](./container/docker-compose.md)

### Docker Hub

Die zentrale, öffentliche Registry für Docker Images. Viele offizielle und Community-Images sind auf Docker Hub verfügbar und können mit `docker pull` heruntergeladen werden.

Siehe auch: [Docker](./container/docker.md)

### Dockerfile

Eine Textdatei, die Schritt-für-Schritt beschreibt, wie ein Docker Image gebaut wird. Ein Dockerfile enthält Befehle wie `FROM`, `RUN`, `COPY` und `EXPOSE`.

Siehe auch: [Docker](./container/docker.md)

### Domain

Ein lesbarer Name für eine Internetadresse, z.B. `example.com`. Domains bestehen aus einer Top-Level-Domain (.com, .de), einer Second-Level-Domain (example) und optional Subdomains.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

### Domainregistrar

Ein Dienstleister, bei dem man Domains registrieren kann. Der Registrar verwaltet die DNS-Einträge und die Autorität über die gemietete Domain.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

### DSGVO (Datenschutz-Grundverordnung)
*Englisch: GDPR (General Data Protection Regulation)*

Die europäische Datenschutzverordnung, die strenge Anforderungen an die Verarbeitung von Personendaten stellt. Nextcloud ist eine DSGVO-konforme Alternative zu Cloud-Diensten von US-Anbietern.

Siehe auch: [Was ist Nextcloud?](../kurs/03-filesharing-und-groupware/01-nextcloud.md), [GDPR](#gdpr-general-data-protection-regulation)

### Dual Licensing

Geschäftsmodell, bei dem die gleiche Software unter zwei Lizenzen angeboten wird: eine offene (z.B. GPL) für Community-Nutzung, eine kommerzielle für Unternehmen, die die Copyleft-Bedingungen nicht erfüllen können.

Siehe auch: [Geschäftsmodelle mit Open Source](../kurs/01-einstieg-open-source/04-geschaeftsmodelle.md)

### DV-Zertifikat (Domain Validation)

Ein Zertifikat, das nur prüft, ob man die Domain kontrolliert, nicht wer man als Person oder Organisation ist. Let's Encrypt stellt ausschließlich DV-Zertifikate aus, was für HTTPS ausreichend ist.

Siehe auch: [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

## E

### Entrypoint

Der Befehl oder das Skript, das als Haupt-Prozess eines Docker-Containers gestartet wird. In Docker Compose kann der Standard-Entrypoint des Images mit `entrypoint:` überschrieben werden.

Siehe auch: [Docker Compose](./container/docker-compose.md)

### Expose (Docker)

Eine Docker-Anweisung (`expose:` in Compose oder `EXPOSE` im Dockerfile), die dokumentiert, auf welchem Port ein Container intern lauscht – ohne den Port nach außen freizugeben. Im Gegensatz zu `ports:` ist `expose:` nur für die Container-zu-Container-Kommunikation im selben Netzwerk gedacht.

Siehe auch: [Docker Compose](./container/docker-compose.md), [Nginx Grundlagen](./components/03-nginx-grundlagen.md)

### ED25519

Ein modernes Public-Key-Kryptographieverfahren, das als Nachfolger von RSA gilt. Bietet bessere Sicherheit bei kürzeren Schlüsseln und wird von OpenSSH empfohlen (`ssh-keygen -t ed25519`).

Siehe auch: [SSH](./server-and-network/03-ssh.md)

### EFF (Electronic Frontier Foundation)

Eine gemeinnützige Bürgerrechtsorganisation, die sich für digitale Freiheit und Privatsphäre einsetzt. Die EFF ist Mitentwickler von Certbot und Unterstützer von Let's Encrypt.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md)

### Environment Variable (Umgebungsvariable)

Eine Konfigurationsvariable, die an einen Prozess oder Container übergeben wird. Ermöglicht flexible Konfiguration ohne Code-Änderungen und wird genutzt, um Passwörter und Einstellungen aus dem Code herauszuhalten.

Siehe auch: [Docker Compose](./container/docker-compose.md), [Geheimnisse schützen](./server-hardening/04-geheimnisse-schuetzen.md)

### Enterprise Edition

Eine kostenpflichtige Variante einer Software mit zusätzlichen Features, Support und Compliance-Funktionen. Nextcloud bietet eine Enterprise Edition mit dediziertem Support und SLA.

Siehe auch: [Was ist Nextcloud?](../kurs/03-filesharing-und-groupware/01-nextcloud.md)

### EV-Zertifikat (Extended Validation)

Ein Zertifikat, das zusätzlich zur Domain-Inhaberschaft auch die Existenz der Organisation validiert. Teurer als DV-Zertifikate und nur für große Unternehmen relevant.

Siehe auch: [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

## F

### Fulltext Search (Volltextsuche)

Eine Datenbankfunktion, die den Inhalt von Dokumenten oder Textspalten durchsucht, nicht nur exakte Treffer findet. Nextcloud's Suche funktioniert mit SQLite eingeschränkt; PostgreSQL und MariaDB unterstützen echte Volltextsuche.

Siehe auch: [Datenbanken bei Nextcloud](../kurs/03-filesharing-und-groupware/03-datenbanken.md), [PostgreSQL Grundlagen](./components/02-postgresql-grundlagen.md)

### Fedora

Eine Linux-Distribution, die von Red Hat gesponsert wird und technisch oft vorne dabei ist. Fedora nutzt neuere Software als Debian/Ubuntu und dient als Vorschau auf kommende Funktionen von Red Hat Enterprise Linux (RHEL).

Siehe auch: [Linux-Distributionen](../kurs/02-einstieg-linux/02-distributionen.md)

### Fail2Ban

Ein Sicherheitstool, das automatisch IP-Adressen blockiert, nachdem zu viele fehlgeschlagene Anmeldeversuche erkannt wurden. Fail2Ban arbeitet mit UFW zusammen, um Firewall-Regeln dynamisch zu verwalten.

Siehe auch: [Fail2Ban](./server-hardening/02-fail2ban.md)

### Fediverse

Ein föderiertes Netzwerk dezentraler Server, die über offene Protokolle (hauptsächlich ActivityPub) miteinander kommunizieren. Mastodon, PeerTube und Matrix sind populäre Fediverse-Dienste.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md)

### Firewall

Ein Sicherheitssystem, das eingehenden und ausgehenden Netzwerkverkehr filtert. Auf Linux-Servern wird UFW als benutzerfreundliche Schicht über das Kernel-Tool iptables eingesetzt.

Siehe auch: [Firewall & UFW](./server-hardening/01-firewall-ufw.md)

### Fork

Eine Kopie eines Open-Source-Projekts, bei der ein Entwickler oder Team das Projekt in eine andere Richtung weiterentwickelt. Forks schützen Communities vor Lizenzwechseln oder Projektabbrüchen.

Siehe auch: [Communities & Warum Open Source nutzen?](../kurs/01-einstieg-open-source/05-communities-und-gruende.md)

### Free Software / Freie Software

Software, die vier fundamentale Freiheiten bietet: ausführen, studieren, kopieren und verändern sowie veröffentlichen. Free Software betont ethische Freiheit und nicht nur Kostenlosigkeit.

Siehe auch: [Was ist Open Source?](../kurs/01-einstieg-open-source/02-was-ist-open-source.md)

### FSF (Free Software Foundation)

Von Richard Stallman gegründete gemeinnützige Organisation, die sich für Software-Freiheit einsetzt und gegen proprietäre Software. Die FSF hat die GPL-Lizenzen entwickelt.

Siehe auch: [Was ist Open Source?](../kurs/01-einstieg-open-source/02-was-ist-open-source.md)

### FTP (File Transfer Protocol)

Ein Netzwerkprotokoll zum unverschlüsselten Dateitransfer. Nextcloud kann externe FTP-Speicher als Storage-Backend einbinden; für neue Self-Hosting-Setups sollte SFTP (verschlüsselt) bevorzugt werden.

Siehe auch: [Nextcloud Apps & Plugins](../kurs/03-filesharing-und-groupware/04-plugins.md)

## G

### GRUB (Grand Unified Bootloader)

Der Standard-Bootloader auf den meisten Linux-Distributionen. GRUB wird beim Start des Computers ausgeführt, zeigt ein Auswahlmenü für installierte Betriebssysteme und lädt dann den Linux-Kernel.

Siehe auch: [Was ist Linux?](../kurs/02-einstieg-linux/01-was-ist-linux.md)

### Gruppe (Linux)

Eine Sammlung von Benutzern, denen gemeinsam Berechtigungen erteilt werden können. Gruppen vereinfachen die Zugriffsverwaltung, z.B. gibt die `sudo`-Gruppe allen Mitgliedern Admin-Rechte.

Siehe auch: [Benutzer, Gruppen und Berechtigungen](./linux_basics/03-users-groups-permissions.md)

### GDPR (General Data Protection Regulation)
*→ Siehe [DSGVO (Datenschutz-Grundverordnung)](#dsgvo-datenschutz-grundverordnung)*

### GNU / GNU/Linux

Das Projekt von Richard Stallman zur Schaffung eines vollständig freien Betriebssystems. GNU-Werkzeuge (Compiler, Shell, Utilities) bilden zusammen mit dem Linux-Kernel das Fundament des Betriebssystems, das korrekt GNU/Linux heißt.

Siehe auch: [Was ist Linux?](../kurs/02-einstieg-linux/01-was-ist-linux.md)

### GPL (GNU General Public License)

Die bekannteste Copyleft-Lizenz, entwickelt von Richard Stallman. Verlangt, dass abgeleitete Werke ebenfalls unter GPL veröffentlicht werden – der sogenannte „Copyleft-Effekt".

Siehe auch: [Open-Source-Lizenzen](../kurs/01-einstieg-open-source/03-lizenzen.md)

### GPG (GNU Privacy Guard)

Die freie Open-Source-Implementierung von PGP, die auf Linux-Systemen für Verschlüsselung und digitale Signaturen verwendet wird.

Siehe auch: [PGP Cheat Sheet](./tools/pgp-cheat-sheet.md)

### Groupware

Software für Teamzusammenarbeit mit Funktionen wie Dateispeicher, Kalender, Kontakte, Chat und Videokonferenzen. Nextcloud ist eine selbst hostbare Groupware-Lösung.

Siehe auch: [Was ist Nextcloud?](../kurs/03-filesharing-und-groupware/01-nextcloud.md)

### Grafische Benutzeroberfläche
*→ Siehe [GUI (Graphical User Interface)](#gui-graphical-user-interface)*

### GUI (Graphical User Interface)
*Deutsch: Grafische Benutzeroberfläche*

Eine visuelle Schnittstelle mit Fenstern, Schaltflächen und Maus-Interaktion. Auf Servern wird meist die CLI bevorzugt, da GUIs Ressourcen verbrauchen.

Siehe auch: [Was ist Linux?](../kurs/02-einstieg-linux/01-was-ist-linux.md), [Grafische Benutzeroberfläche](#grafische-benutzeroberfläche)

## H

### Health Check (Gesundheitsprüfung)

Ein Test, der regelmäßig prüft, ob ein Service antwortet und funktioniert. Health Checks werden in Docker Compose konfiguriert und helfen, fehlerhafte Container zu erkennen, bevor andere Dienste von ihnen abhängen.

Siehe auch: [Docker Compose](./container/docker-compose.md)

### HTTPS (HyperText Transfer Protocol Secure)

Die verschlüsselte Variante von HTTP mit TLS. HTTPS ist heute Pflicht für alle öffentlich erreichbaren Dienste und durch Let's Encrypt kostenlos erhältlich.

Siehe auch: [SSL und Let's Encrypt](./server-and-network/04-ssl-letsencrypt.md), [Nginx Grundlagen](./components/03-nginx-grundlagen.md)

### HTTP (HyperText Transfer Protocol)

Das unverschlüsselte Protokoll für die Datenübertragung im Web auf Port 80. HTTP sollte heute nur noch als Umleitung zu HTTPS verwendet werden.

Siehe auch: [Hosts, Ports und Protokolle](./server-and-network/02-host-port-protokolle.md)

## I

### init (Init-System)

Der erste Prozess, der beim Start eines Linux-Systems gestartet wird und immer die PID 1 hat. Das init-System startet alle weiteren Dienste und Prozesse. Auf modernen Systemen ist `systemd` das Standard-Init-System.

Siehe auch: [Prozesse & Systemüberwachung](./linux_basics/05-prozesse-systemueberwachung.md)

### Inode

Eine Datenstruktur im Linux-Dateisystem, die Metadaten einer Datei speichert: Eigentümer, Berechtigungen, Zeitstempel und Zeiger auf die Dateidaten. Jede Datei hat genau einen Inode; der Dateiname ist lediglich ein Verweis darauf.

Siehe auch: [Dateisystem einbinden](./linux_basics/08-dateisystem-einhaengen.md)

### Image (Docker)

Eine unveränderliche Vorlage mit allem, was nötig ist, um einen Container auszuführen: Betriebssystem, Bibliotheken, Code, Konfiguration. Images werden von Containern instanziiert und auf Registries wie Docker Hub verteilt.

Siehe auch: [Docker](./container/docker.md)

### Index (Datenbank)

Eine Datenstruktur, die die Suche in Datenbanktabellen beschleunigt. Indizes ermöglichen schnelle Lookups, kosten aber Speicherplatz und verlangsamen Schreiboperationen.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

### In-Memory-Speicher

Ein Speichersystem, das Daten ausschließlich im RAM vorhält statt auf der Festplatte. Dadurch sind Zugriffe extrem schnell (unter 1 Millisekunde). Redis ist der bekannteste In-Memory-Speicher im Self-Hosting-Kontext.

Siehe auch: [Redis Grundlagen](./components/04-redis-grundlagen.md)

### IPv4 (Internet Protocol Version 4)

Die meistgenutzte Version des Internet-Protokolls mit Adressen aus vier Zahlen (0–255), z.B. `192.168.1.10`. IPv4 hat nur ca. 4 Milliarden mögliche Adressen, was durch IPv6 gelöst werden soll.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

### IPv6 (Internet Protocol Version 6)

Die neuere Version des Internet-Protokolls mit deutlich längerer Adressschreibweise und praktisch unbegrenztem Adressraum. IPv6 soll IPv4 langfristig ablösen, ist aber noch nicht überall verbreitet.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

### IP-Adresse

Eine eindeutige Nummer, die jeden Computer oder Host im Netzwerk identifiziert. IPv4-Adressen bestehen aus vier Zahlen, IPv6-Adressen sind hexadezimal und deutlich länger.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

### ISO (Image)

Datenträgerabbild, dass man auf einen USB-Stick, eine CD/DVD, oder eine Festplattenpartition aufspeilen kann. Wird in diesem Kontext für Installationsumgebungen oder als Schnappschuss einer Festplatte verwendet.

### ISRG (Internet Security Research Group)

Die gemeinnützige Organisation, die Let's Encrypt betreibt und kostenlose TLS-Zertifikate für alle ermöglicht. Unterstützt von Mozilla, EFF und anderen.

Siehe auch: [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

## J

### JSON (JavaScript Object Notation)

Ein populäres, menschenlesbares Format für Datenserialisierung. PostgreSQL kann JSON nativ speichern und abfragen. Auch Docker-Logging und viele APIs verwenden JSON.

Siehe auch: [PostgreSQL Grundlagen](./components/02-postgresql-grundlagen.md)

### journald / journalctl

Der systemd-eigene Logging-Dienst, der Systemlogs strukturiert speichert. Mit `journalctl -u <service>` lassen sich Logs eines bestimmten Dienstes anzeigen.

Siehe auch: [Log-Management](./linux_basics/07-shell-scripting.md)

## K

### Kernel

Der Kern des Betriebssystems, der direkt mit der Hardware kommuniziert und Ressourcen für alle anderen Programme verwaltet. Der Linux-Kernel ist monolithisch und Open Source.

Siehe auch: [Was ist Linux?](../kurs/02-einstieg-linux/01-was-ist-linux.md)

### Key-Value-Speicher (Key-Value Store)

Ein einfaches Speichermodell, das Daten als Paare aus Schlüsseln und Werten organisiert. Redis ist ein In-Memory Key-Value-Speicher, der für Caching, Sessions und Queues eingesetzt wird.

Siehe auch: [Redis Grundlagen](./components/04-redis-grundlagen.md)

### Kanban-Board

Eine visuelle Methode zur Aufgabenverwaltung, bei der Aufgaben als Karten auf Spalten (z.B. „To Do", „In Progress", „Done") verschoben werden. Nextcloud Deck implementiert ein Kanban-Board ähnlich wie Trello.

Siehe auch: [Nextcloud Apps & Plugins](../kurs/03-filesharing-und-groupware/04-plugins.md)

### Kubernetes
*Abkürzung: K8s*

Eine Orchestrierungsplattform für Container-Verwaltung in großem Maßstab. Kubernetes ist deutlich komplexer als Docker Compose und wird erst bei vielen Containern und Hochverfügbarkeits-Anforderungen relevant.

## L

### LAMP-Stack

Eine klassische Kombination von Serversoftware: Linux, Apache, MySQL/MariaDB und PHP. LAMP ist die Basis für viele PHP-Anwendungen wie WordPress. Im modernen Self-Hosting wird Apache oft durch Nginx ersetzt.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

### Layer (Docker Image)

Eine Schicht in einem Docker Image, die einen Zustand des Dateisystems repräsentiert. Images bestehen aus mehreren Layers, was effizientes Caching und Wiederverwendung bei `docker pull` ermöglicht.

Siehe auch: [Docker](./container/docker.md)

### Let's Encrypt

Eine gemeinnützige, kostenlose Certificate Authority, die TLS-Zertifikate für jedermann ausstellt und HTTPS damit demokratisiert hat. Zertifikate laufen nach 90 Tagen ab und werden von Certbot automatisch erneuert.

Siehe auch: [SSL und Let's Encrypt](./server-and-network/04-ssl-letsencrypt.md), [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

### LGPL (GNU Lesser General Public License)

Eine schwächere Copyleft-Lizenz, die erlaubt, LGPL-Bibliotheken in proprietärer Software zu verwenden, ohne den gesamten Code zu veröffentlichen. Oft für Bibliotheken gewählt.

Siehe auch: [Open-Source-Lizenzen](../kurs/01-einstieg-open-source/03-lizenzen.md)

### LLM (Large Language Model)

Ein KI-Modell mit Milliarden von Parametern, das natürliche Sprache verarbeitet und generiert. Ollama ermöglicht den lokalen Betrieb von LLMs für Nextcloud-KI-Features.

Siehe auch: [KI-Features & KI-Backend](../kurs/03-filesharing-und-groupware/05-ki-features.md)

### Lock-in / Vendor Lock-in

Situation, in der ein Nutzer nicht einfach den Anbieter wechseln kann, weil technische oder finanzielle Hürden zu hoch sind. Self-Hosting und Open-Source-Software helfen, Lock-in zu vermeiden.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md)

### LTS (Long-Term Support)

Eine Software-Version mit verlängertem Sicherheits-Support, meist mehrere Jahre. Ubuntu LTS-Versionen (z.B. 24.04 LTS) sind die Empfehlung für Produktionsserver.

Siehe auch: [Linux-Distributionen](../kurs/02-einstieg-linux/02-distributionen.md)

## M

### Memcache

Ein Caching-Mechanismus für Webanwendungen, der häufig abgerufene Daten im RAM hält. Nextcloud unterstützt verschiedene Memcache-Backends (APCu für lokalen Cache, Redis für verteilten Cache). Redis ist im Self-Hosting die empfohlene Wahl.

Siehe auch: [Redis Grundlagen](./components/04-redis-grundlagen.md)

### Middleware

Software, die zwischen Anfrage und Antwort liegt und übergreifende Aufgaben wie Authentifizierung, Logging oder Rate-Limiting übernimmt. Nginx kann als Middleware vor Anwendungen eingesetzt werden.

Siehe auch: [Nginx Grundlagen](./components/03-nginx-grundlagen.md)

### Man-in-the-Middle-Angriff (MITM)

Ein Sicherheitsangriff, bei dem sich ein Angreifer zwischen Client und Server positioniert und beide täuscht. TLS-Zertifikate und deren Validierung verhindern MITM-Angriffe bei HTTPS.

Siehe auch: [SSL und Let's Encrypt](./server-and-network/04-ssl-letsencrypt.md), [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

### MariaDB

Ein kostenloser Fork von MySQL, entwickelt von den ursprünglichen MySQL-Entwicklern. MariaDB ist zu großen Teilen MySQL-kompatibel und wird oft mit Nextcloud und WordPress eingesetzt.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

### Mastodon

Eine dezentrale, selbst hostbare Social-Media-Plattform im Fediverse. Mastodon wird typischerweise mit PostgreSQL und Redis betrieben.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

### Merge Request
*→ Siehe [Pull Request (PR)](#pull-request-pr)*

### MIT-Lizenz

Die populärste und einfachste Open-Source-Lizenz. Extrem permissiv: Nutzung, Änderung und Weitergabe sind erlaubt, solange der Copyright-Hinweis beigelegt wird.

Siehe auch: [Open-Source-Lizenzen](../kurs/01-einstieg-open-source/03-lizenzen.md)

### Mount

Das Einbinden eines Dateisystems oder Verzeichnisses in einen Container oder an einen Mountpoint im Dateisystem. Docker Volumes und Bind Mounts ermöglichen Datenpersistenz zwischen Host und Container.

Siehe auch: [Docker Compose](./container/docker-compose.md), [Dateisystem einbinden](./linux_basics/08-dateisystem-einhaengen.md)

### MX-Record (Mail Exchange Record)

Ein DNS-Eintrag, der angibt, welcher Mail-Server für eine Domain zuständig ist. Notwendig, damit E-Mails an die eigene Domain zugestellt werden können.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

### MySQL

Eine beliebte relationale Datenbank, auf der MariaDB basiert. MySQL und MariaDB sind in der Praxis weitgehend austauschbar.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

## N

### nano

Ein einfacher, anfängerfreundlicher Texteditor für die Kommandozeile. nano zeigt die wichtigsten Tastenkürzel direkt am unteren Bildschirmrand an (Strg+O zum Speichern, Strg+X zum Beenden) und ist auf Ubuntu und Debian vorinstalliert.

Siehe auch: [Die Linux Shell](./linux_basics/02-linux-shell.md)

### Nameserver

Ein DNS-Server, der autoritativ für eine Domain verantwortlich ist. Nameserver halten die DNS-Einträge für Domains und antworten auf DNS-Anfragen weltweit.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

### Nextcloud

Eine selbst hostbare Collaboration-Plattform mit Dateispeicher, Kalender, Kontakten, Chat und mehr. Nextcloud ist unter AGPL lizenziert und die bekannteste Open-Source-Alternative zu Google Drive.

Siehe auch: [Was ist Nextcloud?](../kurs/03-filesharing-und-groupware/01-nextcloud.md)

### Nextcloud Deck

Eine Kanban-Board-App für Nextcloud zur Aufgabenverwaltung in Teams. Deck funktioniert ähnlich wie Trello und ermöglicht das Anlegen von Boards, Listen und Karten.

Siehe auch: [Nextcloud Apps & Plugins](../kurs/03-filesharing-und-groupware/04-plugins.md)

### Nextcloud Photos

Eine Fotogalerie-App in Nextcloud mit automatischer Sortierung nach Datum und Ort. Nextcloud Photos ist die eingebaute Alternative zu Google Fotos.

Siehe auch: [Was ist Nextcloud?](../kurs/03-filesharing-und-groupware/01-nextcloud.md)

### Nextcloud Talk

Eine integrierte Chat- und Videokonferenz-App in Nextcloud. Talk ist die selbst hostbare Alternative zu Microsoft Teams oder Google Meet und unterstützt verschlüsselte Sprach- und Videoanrufe.

Siehe auch: [Nextcloud Apps & Plugins](../kurs/03-filesharing-und-groupware/04-plugins.md)

### Nextcloud AIO (All-in-One)

Eine Docker-basierte Installation von Nextcloud, die alle Komponenten (Datenbank, Redis, Reverse Proxy, Backup) automatisch verwaltet. Empfohlen für einfaches Self-Hosting ohne viel Konfigurationsaufwand.

Siehe auch: [Nextcloud AIO Setup](../kurs/03-filesharing-und-groupware/02-nextcloud-aio-setup.md)

### nginx (Engine X)

Ein leichtgewichtiger, hochperformanter Webserver und Reverse Proxy. Nginx ist der Standard für Self-Hosting und wird vor vielen Anwendungen als Frontend-Server betrieben.

Siehe auch: [Nginx Grundlagen](./components/03-nginx-grundlagen.md)

### NoSQL
*Abkürzung: Not Only SQL*

Ein Datenbanktyp, der nicht auf relationalen Tabellen basiert. Beispiele sind Redis (Key-Value) und MongoDB (Dokumente). NoSQL ergänzt SQL, ersetzt es aber nicht vollständig.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

## O

### OIDC (OpenID Connect)

Ein modernes Authentifizierungsprotokoll für Single Sign-On (SSO), das auf OAuth 2.0 aufbaut. OIDC ermöglicht zentrale Authentifizierung über mehrere Dienste hinweg.

### OLAP (Online Analytical Processing)

Datenbankworkload für komplexe Analysen auf großen Datenmengen, typischerweise für Business Intelligence. Gegenpol zu OLTP für Echtzeit-Transaktionen.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

### OLTP (Online Transaction Processing)

Datenbankworkload für schnelle Echtzeit-Transaktionen mit vielen kleinen Abfragen. OLTP ist der Standard für Web-Anwendungen wie Nextcloud.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

### Ollama

Ein Tool zum lokalen Betrieb von Large Language Models. Ollama bietet eine OpenAI-kompatible API und ermöglicht selbst gehostete KI für Nextcloud und andere Dienste.

Siehe auch: [KI-Features & KI-Backend](../kurs/03-filesharing-und-groupware/05-ki-features.md)

### OnlyOffice

Ein Microsoft-Office-kompatibler Office-Editor, der als Nextcloud-App integriert wird. OnlyOffice bietet bessere Kompatibilität mit .docx/.xlsx-Dateien als Collabora Online.

Siehe auch: [Nextcloud Apps & Plugins](../kurs/03-filesharing-und-groupware/04-plugins.md)

### Open Core

Geschäftsmodell, bei dem der Kern Open Source ist, aber Enterprise-Features proprietär und kostenpflichtig sind. GitLab, Grafana und Nextcloud nutzen dieses Modell.

Siehe auch: [Geschäftsmodelle mit Open Source](../kurs/01-einstieg-open-source/04-geschaeftsmodelle.md)

### OSI (Open Source Initiative)

Gemeinnützige Organisation, die 10 Kriterien für die Definition von Open Source festgelegt hat und Lizenzen offiziell zertifiziert. 1998 gegründet, um Open Source als Entwicklungsmodell zu fördern.

Siehe auch: [Was ist Open Source?](../kurs/01-einstieg-open-source/02-was-ist-open-source.md)

### ORM (Object-Relational Mapping)

Eine Programmiertechnik, die Datenbankzeilen auf Objekte in der Programmiersprache abbildet. ORMs vereinfachen den Datenzugriff, können aber bei komplexen Abfragen ineffizient sein.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

### OV-Zertifikat (Organization Validation)

Ein Zertifikat, das zusätzlich zur Domain-Inhaberschaft auch die Existenz der Organisation validiert. Teurer als DV-Zertifikate, aber nicht für die meisten Self-Hosting-Setups nötig.

Siehe auch: [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

### ownCloud

Die Vorgänger-Software von Nextcloud, aus der Nextcloud 2016 als Fork hervorging. ownCloud existiert noch, liegt aber in Entwicklung und Verbreitung weit hinter Nextcloud zurück.

Siehe auch: [Was ist Nextcloud?](../kurs/03-filesharing-und-groupware/01-nextcloud.md)

## P

### PID (Process ID)

Eine eindeutige Nummer, die Linux jedem laufenden Prozess zuweist. Mit der PID kann man einen Prozess gezielt ansprechen, z.B. mit `kill <PID>` beenden. Der Init-Prozess hat immer PID 1.

Siehe auch: [Prozesse & Systemüberwachung](./linux_basics/05-prozesse-systemueberwachung.md)

### POSIX (Portable Operating System Interface)

Ein Satz von Standards, der eine einheitliche Schnittstelle zwischen Unix-artigen Betriebssystemen definiert. Linux ist weitgehend POSIX-kompatibel, was bedeutet, dass Skripte und Programme portabel zwischen Linux, macOS und anderen Unix-Systemen sind.

Siehe auch: [Was ist Linux?](../kurs/02-einstieg-linux/01-was-ist-linux.md)

### Prozess

Eine laufende Instanz eines Programms. Linux weist jedem Prozess eine eindeutige PID zu, verwaltet seine Ressourcen und trennt ihn von anderen Prozessen. Webserver, Datenbankdienste und die Shell sind allesamt Prozesse.

Siehe auch: [Prozesse & Systemüberwachung](./linux_basics/05-prozesse-systemueberwachung.md)

### Passphrase

Ein Passwort für einen privaten Schlüssel (bei SSH oder GPG). Schützt den Schlüssel zusätzlich, falls jemand Zugang zur Schlüsseldatei bekommt.

Siehe auch: [SSH](./server-and-network/03-ssh.md), [PGP Cheat Sheet](./tools/pgp-cheat-sheet.md)

### Permissive License (Permissive Lizenz)

Open-Source-Lizenz mit wenigen Einschränkungen, die auch kommerzielle und proprietäre Nutzung erlaubt. MIT, Apache 2.0 und BSD sind Beispiele.

Siehe auch: [Open-Source-Lizenzen](../kurs/01-einstieg-open-source/03-lizenzen.md)

### PGP (Pretty Good Privacy)

Ein System zur Verschlüsselung und digitalen Signierung von Daten und Nachrichten. GPG ist die freie Open-Source-Implementierung davon.

Siehe auch: [PGP Cheat Sheet](./tools/pgp-cheat-sheet.md)

### pgAdmin

Eine webbasierte Administrationsoberfläche für PostgreSQL. pgAdmin ermöglicht die Verwaltung von Datenbanken, Tabellen und Abfragen über ein grafisches Interface anstelle der Kommandozeile.

Siehe auch: [Datenbanken bei Nextcloud](../kurs/03-filesharing-und-groupware/03-datenbanken.md)

### pg_dump

Ein Tool zur Erstellung von PostgreSQL-Backups. Wird direkt im Container ausgeführt und erzeugt SQL- oder Binär-Backups der Datenbank.

Siehe auch: [PostgreSQL Grundlagen](./components/02-postgresql-grundlagen.md)

### PKI (Public Key Infrastructure)

Das gesamte System zur Ausgabe, Verwaltung und Validierung von Zertifikaten. Basiert auf vertrauenswürdigen Zertifizierungsstellen (CAs) und deren Signaturen.

Siehe auch: [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

### Port

Ein nummerierter „Kanal" auf einem Host, über den verschiedene Netzwerkdienste gleichzeitig laufen können. Ports sind Zahlen von 0 bis 65535; bekannte Standardports sind z.B. 22 (SSH), 80 (HTTP), 443 (HTTPS).

Siehe auch: [Hosts, Ports und Protokolle](./server-and-network/02-host-port-protokolle.md)

### Port Mapping

Die Zuordnung eines Ports auf dem Host zu einem Port im Docker-Container (`host:container`). Port Mapping macht Dienste von außen erreichbar und wird in Docker Compose mit `ports:` konfiguriert.

Siehe auch: [Docker Compose](./container/docker-compose.md)

### PostgreSQL
*Kurzform: Postgres*

Eine robuste, vollwertige relationale Datenbank mit erweiterten Features. PostgreSQL ist die Standard-Wahl für Self-Hosting und wird von Nextcloud AIO automatisch eingesetzt.

Siehe auch: [PostgreSQL Grundlagen](./components/02-postgresql-grundlagen.md), [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

### Private Key (Privater Schlüssel)

Der geheime Teil eines asymmetrischen Schlüsselpaars, der niemals weitergegeben werden darf. Wird zum Entschlüsseln von Nachrichten, zum Signieren und zur SSH-Authentifizierung verwendet.

Siehe auch: [SSH](./server-and-network/03-ssh.md), [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

### Proprietäre Software
*Synonym: Closed Source*

Software, deren Quellcode nicht öffentlich einsehbar ist. Nutzer müssen dem Hersteller vertrauen, was die Software wirklich tut.

Siehe auch: [Was ist Open Source?](../kurs/01-einstieg-open-source/02-was-ist-open-source.md), [Closed Source](#closed-source)

### Protokoll (Netzwerk)

Ein Regelwerk für die Kommunikation zwischen Computern. TCP und UDP sind Transportprotokolle; HTTP, HTTPS und SSH sind Anwendungsprotokolle.

Siehe auch: [Hosts, Ports und Protokolle](./server-and-network/02-host-port-protokolle.md)

### Public Key (Öffentlicher Schlüssel)

Der Teil eines asymmetrischen Schlüsselpaars, der öffentlich verteilt werden darf. Wird zum Verschlüsseln von Nachrichten oder zur Verifizierung von Signaturen verwendet.

Siehe auch: [SSH](./server-and-network/03-ssh.md), [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

### Public Money, Public Code

Forderung, dass mit öffentlichen Mitteln entwickelte Software auch als Open Source veröffentlicht werden soll. 2025 vom Europäischen Parlament unterstützt.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md)

### Pub/Sub (Publish/Subscribe)

Ein Messaging-Muster, bei dem Sender (Publisher) Nachrichten an Kanäle schicken und Empfänger (Subscriber) diese Kanäle abonnieren. Redis unterstützt Pub/Sub nativ und wird z.B. von Mastodon für Echtzeit-Benachrichtigungen eingesetzt.

Siehe auch: [Redis Grundlagen](./components/04-redis-grundlagen.md)

### Pull Request (PR)
*GitLab: Merge Request*

Mechanismus zur Einreichung von Code-Änderungen in ein Open-Source-Projekt. Maintainer reviewen den Code und entscheiden, ob er integriert wird.

Siehe auch: [Communities & Warum Open Source nutzen?](../kurs/01-einstieg-open-source/05-communities-und-gruende.md), [Merge Request](#merge-request)

## Q

### Quellcode (Source Code)

Maschinenlesbarer, aber für Menschen verständlicher Code, in dem Software geschrieben wird. Bei Open Source ist der Quellcode öffentlich einsehbar – das Kernmerkmal, das Open Source definiert.

Siehe auch: [Was ist Open Source?](../kurs/01-einstieg-open-source/02-was-ist-open-source.md)

### Query (Datenbankabfrage)

Eine Anfrage an eine Datenbank, um Daten zu lesen oder zu verändern. SQL-Queries werden mit SELECT, INSERT, UPDATE und DELETE formuliert.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

## R

### Repository (APT)
*Kurzform: Repo*

Ein Server, der signierte und geprüfte Softwarepakete für den Paketmanager bereitstellt. Ubuntu liefert offizielle Repositories (main, universe, restricted); zusätzlich können Drittanbieter-Repos (PPAs) hinzugefügt werden.

Siehe auch: [Paketverwaltung](./linux_basics/04-paketverwaltung.md)

### Root-Verzeichnis

Das oberste Verzeichnis im Linux-Dateisystem, dargestellt als `/`. Alle anderen Verzeichnisse und Dateien sind Unterverzeichnisse dieses Wurzelverzeichnisses — es gibt keine Laufwerksbuchstaben wie unter Windows.

Siehe auch: [Das Linux-Mentalmodell](./linux_basics/01-linux-mental-model.md)

### RPM (RPM Package Manager)
*Ursprünglich: Red Hat Package Manager*

Das Paketformat und -werkzeug von Red Hat-basierten Distributionen (Fedora, Rocky Linux, AlmaLinux). RPM ist das Pendant zu `.deb` unter Debian/Ubuntu; der High-Level-Paketmanager darüber ist `dnf`.

Siehe auch: [Linux-Distributionen](../kurs/02-einstieg-linux/02-distributionen.md)

### RDB (Redis Database Snapshot)

Eine Redis-Persistenzmethode, die in regelmäßigen Abständen Snapshots der Datenbank auf Festplatte speichert. RDB ist kompakt, aber Datenverlust zwischen Snapshots ist möglich.

Siehe auch: [Redis Grundlagen](./components/04-redis-grundlagen.md)

### Redis

Ein In-Memory Key-Value-Speicher, der für Caching, Sessions, Queues und Pub/Sub eingesetzt wird. Nextcloud nutzt Redis für Transactional File Locking und als Cache.

Siehe auch: [Redis Grundlagen](./components/04-redis-grundlagen.md)

### Rate Limiting

Ein Mechanismus, der die Anzahl von Anfragen eines Clients in einem Zeitraum begrenzt. Schützt vor Brute-Force-Angriffen und übermäßiger Last. Redis wird häufig für Rate-Limiting verwendet, da atomare Zähler-Operationen extrem schnell sind.

Siehe auch: [Redis Grundlagen](./components/04-redis-grundlagen.md)

### Replikation (Datenbank)

Das Kopieren von Datenbankänderungen auf ein sekundäres System für Hochverfügbarkeit und Lastverteilung. Wird für Produktionssysteme empfohlen, aber nicht für einfaches Self-Hosting benötigt.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

### REST / RESTful API

Ein Architekturstil für Web-APIs, der HTTP-Methoden (GET, POST, PUT, DELETE) auf Ressourcen anwendet. Nextcloud bietet umfangreiche REST-APIs für die Integration mit anderen Diensten.

Siehe auch: [KI-Features & KI-Backend](../kurs/03-filesharing-und-groupware/05-ki-features.md)

### Reverse Proxy

Ein Server, der zwischen dem Internet und internen Diensten sitzt, Anfragen entgegennimmt und an den richtigen Backend-Service weiterleitet. Nginx als Reverse Proxy ermöglicht mehrere Dienste auf Port 443 mit unterschiedlichen Domains.

Siehe auch: [Reverse Proxy](./server-and-network/05-reverse-proxy.md), [Nginx Grundlagen](./components/03-nginx-grundlagen.md)

### Root-Login

Die Anmeldung als `root`-Superuser, oft per SSH. Eine Sicherheitslücke, da `root` auf jedem System existiert und Angreifer den Benutzernamen kennen. Sollte per SSH deaktiviert werden.

Siehe auch: [Root-Login deaktivieren](./server-hardening/03-root-login-verhindern.md)

### RSA

Ein verbreitetes Public-Key-Kryptographieverfahren. Noch sicher, aber ED25519 ist moderner, effizienter und für neue SSH-Keys empfohlen.

Siehe auch: [SSH](./server-and-network/03-ssh.md), [PGP Cheat Sheet](./tools/pgp-cheat-sheet.md)

## S

### Skript (Shell-Skript)

Eine Textdatei mit einer Folge von Shell-Befehlen, die automatisch nacheinander ausgeführt werden. Skripte beginnen mit einer Shebang-Zeile (`#!/bin/bash`) und werden mit `chmod +x` ausführbar gemacht. Sie sind das Hauptwerkzeug zur Automatisierung auf Linux-Servern.

Siehe auch: [Die Linux Shell](./linux_basics/02-linux-shell.md)

### snap

Ein alternatives Paketsystem von Canonical, das Programme als in sich geschlossene, sandboxed Pakete installiert. Snaps sind unabhängig vom System-APT, werden aber seltener verwendet da sie langsamer starten und mehr Ressourcen verbrauchen.

Siehe auch: [Paketverwaltung](./linux_basics/04-paketverwaltung.md)

### stdin / stdout / stderr

Die drei Standarddatenströme jedes Linux-Prozesses: stdin (Standardeingabe, 0), stdout (Standardausgabe, 1) und stderr (Fehlerausgabe, 2). Mit Umleitungsoperatoren (`>`, `>>`, `2>&1`) kann man diese Ströme in Dateien oder andere Programme leiten.

Siehe auch: [Die Linux Shell](./linux_basics/02-linux-shell.md)

### S3 (Simple Storage Service)

Ein von Amazon Web Services eingeführtes objektbasiertes Speicherprotokoll. Nextcloud kann S3-kompatible Speicher als externes Storage-Backend einbinden. Viele selbst hostbare Dienste wie MinIO bieten eine S3-kompatible API.

Siehe auch: [Nextcloud Apps & Plugins](../kurs/03-filesharing-und-groupware/04-plugins.md)

### SaaS (Software as a Service)

Softwarebereitstellung als Cloud-Dienst, bei dem der Anbieter Betrieb und Wartung übernimmt. Nextcloud ist als SaaS bei Partnern oder selbst hostbar erhältlich.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md)

### Self-Hosting

Der Betrieb eigener Dienste auf selbst gewählter Hardware oder einem gemieteten Server, statt externe Cloud-Anbieter zu nutzen. Zentrales Thema dieses Kurses.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md)

### SFTP (SSH File Transfer Protocol)

Ein sicheres Dateitransfer-Protokoll über SSH. Nextcloud kann externe SFTP-Speicher als Storage-Backend einbinden.

Siehe auch: [Nextcloud Apps & Plugins](../kurs/03-filesharing-und-groupware/04-plugins.md)

### Shell

Eine Befehlszeilenumgebung, in der man mit dem Betriebssystem interagiert. Bash ist die Standard-Shell auf Linux. Im Docker-Kontext öffnet `docker exec -it <container> bash` eine Shell im laufenden Container.

Siehe auch: [Die Linux Shell](./linux_basics/02-linux-shell.md)

### Single Point of Failure (SPoF)

Ein kritischer Punkt in einem System, dessen Ausfall das gesamte System lahmlegt. Zentralisierung erzeugt SPoFs; Dezentralisierung und Redundanz reduzieren sie.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md)

### SLA (Service Level Agreement)

Eine Vereinbarung zwischen Anbieter und Kunde über garantierte Verfügbarkeit und Support-Zeiten. Nextcloud Enterprise bietet SLAs für Produktionsinstallationen.

Siehe auch: [Was ist Nextcloud?](../kurs/03-filesharing-und-groupware/01-nextcloud.md)

### SMB (Server Message Block)
*Synonym: CIFS*

Ein Netzwerkprotokoll für Datei- und Druckerfreigabe, das unter Windows Standard ist. Nextcloud kann externe SMB-Speicher als Storage-Backend einbinden.

Siehe auch: [Nextcloud Apps & Plugins](../kurs/03-filesharing-und-groupware/04-plugins.md), [CIFS](#cifs)

### SQL (Structured Query Language)

Die Standard-Abfragesprache für relationale Datenbanken. SQL wird für CREATE, SELECT, UPDATE und DELETE Operationen verwendet und ist Grundlage von PostgreSQL, MariaDB und SQLite.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

### SQLite

Eine dateibasierte SQL-Datenbank ohne separaten Server. SQLite ist einfach einzurichten, aber für kollaborative multi User Anwendungen mit vielen gleichzeitigen Schreibzugriffen wie z.B. Nextcloud geeignet.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md), [Datenbanken bei Nextcloud](../kurs/03-filesharing-und-groupware/03-datenbanken.md)

### SSH (Secure Shell)

Ein verschlüsseltes Protokoll für sichere Fernverwaltung von Servern auf Port 22. SSH ist das Standard-Werkzeug, um sich auf einem VPS anzumelden, Dateien zu übertragen und Tunnel aufzubauen.

Siehe auch: [SSH](./server-and-network/03-ssh.md)

### SSH-Key (SSH-Schlüsselpaar)

Ein asymmetrisches Schlüsselpaar für SSH-Authentifizierung. Der private Schlüssel bleibt lokal, der öffentliche wird auf dem Server hinterlegt. SSH-Keys sind deutlich sicherer als Passwörter.

Siehe auch: [SSH](./server-and-network/03-ssh.md)

### SSL (Secure Sockets Layer)

Das ursprüngliche Verschlüsselungsprotokoll für HTTP, entwickelt in den 1990ern. Gilt heute als veraltet und unsicher; TLS ist der Nachfolger. Der Begriff „SSL-Zertifikat" ist aber weiterhin gebräuchlich.

Siehe auch: [SSL und Let's Encrypt](./server-and-network/04-ssl-letsencrypt.md), [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

### SSO (Single Sign-On)

Ein System, das es Benutzern ermöglicht, sich einmal anzumelden und auf mehrere Dienste zuzugreifen. OIDC ist das moderne Protokoll für SSO.

### SSPL (Server Side Public License)

Eine von MongoDB entwickelte Lizenz, die verlangt, dass Dienste, die die Software nutzen, ebenfalls unter SSPL stehen. Von der OSI nicht als Open Source anerkannt.

Siehe auch: [Geschäftsmodelle mit Open Source](../kurs/01-einstieg-open-source/04-geschaeftsmodelle.md)

### Subdomain

Ein Teil eines Domainnamens, der vor der Second-Level-Domain steht, z.B. `cloud` in `cloud.example.com`. Subdomains werden separat per DNS-Eintrag konfiguriert.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

### Sudo (Substitute User Do)

Ein Linux-Befehl, der es erlaubt, ein Programm mit erhöhten Rechten (z.B. als Root) auszuführen, ohne das Root-Passwort zu kennen. Basis der Sicherheitsarchitektur auf Linux-Systemen.

Siehe auch: [Benutzer, Gruppen und Berechtigungen](./linux_basics/03-users-groups-permissions.md)

### Supply-Chain-Angriff

Ein Angriff auf eine Abhängigkeit oder einen Zulieferer statt auf das Zielprodukt direkt. Beispiele: die XZ-Utils-Backdoor 2024 oder das kompromittierte npm-Paket event-stream.

Siehe auch: [Risiken bei der Nutzung von Open Source](../kurs/01-einstieg-open-source/06-open-source-risiken.md)

### systemd

Der Init-System und Service-Manager der meisten modernen Linux-Distributionen. Mit `systemctl` werden Dienste gestartet, gestoppt und ihr Status abgefragt.

Siehe auch: [Prozesse & Systemüberwachung](./linux_basics/05-prozesse-systemueberwachung.md)

## T

### TCP (Transmission Control Protocol)

Ein zuverlässiges Transportprotokoll, das sicherstellt, dass Datenpakete in der richtigen Reihenfolge ankommen. TCP wird von HTTP, HTTPS, SSH und den meisten anderen Anwendungsprotokollen verwendet.

Siehe auch: [Hosts, Ports und Protokolle](./server-and-network/02-host-port-protokolle.md)

### Terminal

Ein Programm, das eine Shell-Sitzung öffnet und Texteingabe/-ausgabe bereitstellt. Auf Servern ist das Terminal die primäre Arbeitsumgebung.

Siehe auch: [Die Linux Shell](./linux_basics/02-linux-shell.md)

### TLD (Top-Level-Domain)

Der oberste Teil einer Domain, z.B. `.com`, `.de` oder `.org`. Die TLD gibt an, zu welcher Kategorie oder Region eine Domain gehört.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

### TLS (Transport Layer Security)

Das moderne Verschlüsselungsprotokoll für HTTPS, entwickelt als Nachfolger von SSL. TLS sichert die Verbindung zwischen Browser und Server und verhindert Abhören und Manipulation.

Siehe auch: [SSL und Let's Encrypt](./server-and-network/04-ssl-letsencrypt.md), [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

### TLS-Handshake

Der Prozess, bei dem Client und Server eine verschlüsselte TLS-Verbindung aushandeln: Versionsverhandlung, Cipher-Suite-Auswahl, Zertifikataustausch und Schlüsselerzeugung.

Siehe auch: [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

### TOTP (Time-based One-Time Password)

Ein zeitbasiertes Einmalpasswort für Zwei-Faktor-Authentifizierung. TOTP wird von Authenticator-Apps generiert und läuft nach 30 Sekunden ab.

Siehe auch: [Nextcloud Apps & Plugins](../kurs/03-filesharing-und-groupware/04-plugins.md)

### Transactional File Locking

Ein Mechanismus, um zu verhindern, dass zwei Clients dieselbe Datei gleichzeitig beschreiben. Nextcloud nutzt Redis für Transactional File Locking.

Siehe auch: [Datenbanken bei Nextcloud](../kurs/03-filesharing-und-groupware/03-datenbanken.md)

### TTL (Time to Live)

Bezeichnet zwei verschiedene Konzepte: (1) Im DNS: wie lange ein Eintrag im Cache gespeichert wird. (2) In Redis/Caches: wie lange ein Schlüssel gültig ist, bevor er automatisch gelöscht wird.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md), [Redis Grundlagen](./components/04-redis-grundlagen.md)

### TXT-Record

Ein DNS-Eintrag, der beliebige Textinformationen speichert. TXT-Records werden für Domain-Verifizierungen, SPF (E-Mail-Absendervalidierung) und DNS-Challenges bei Let's Encrypt genutzt.

Siehe auch: [IP-Adressen, Domains und DNS](./server-and-network/01-ip-domain-dns.md)

## U

### UID/GID (User ID / Group ID)

Numerische Kennungen für Benutzer und Gruppen unter Linux. Docker-Container laufen standardmäßig als Root (UID 0); für sicherere Setups sollte die `user:`-Option in Docker Compose genutzt werden, damit Datei-Berechtigungen zwischen Host und Container übereinstimmen.

Siehe auch: [Docker Compose](./container/docker-compose.md), [Benutzer, Gruppen und Berechtigungen](./linux_basics/03-users-groups-permissions.md)

### Ubuntu

Eine weit verbreitete Linux-Distribution auf Basis von Debian. Ubuntu LTS-Versionen (z.B. 24.04 LTS) sind die Empfehlung für Self-Hosting-Server.

Siehe auch: [Linux-Distributionen](../kurs/02-einstieg-linux/02-distributionen.md)

### UDP (User Datagram Protocol)

Ein schnelles, aber unzuverlässiges Transportprotokoll, bei dem Datenpakete verloren gehen können. UDP wird von DNS, Video-Streaming und Echtzeit-Anwendungen genutzt.

Siehe auch: [Hosts, Ports und Protokolle](./server-and-network/02-host-port-protokolle.md)

### UFW (Uncomplicated Firewall)

Ein benutzerfreundliches Firewall-Management-Tool für Linux, das die Verwaltung von iptables-Regeln vereinfacht. Mit UFW können Ports leicht erlaubt oder gesperrt werden.

Siehe auch: [Firewall & UFW](./server-hardening/01-firewall-ufw.md)

### Unix

Ein einflussreiches Betriebssystem aus den 1970ern, das das Design von Linux und macOS prägt. Linux ist kein direkter Abkömmling von Unix, folgt aber denselben Prinzipien (POSIX).

Siehe auch: [Was ist Linux?](../kurs/02-einstieg-linux/01-was-ist-linux.md)

### Upstream

(1) Im Open-Source-Kontext: das originale Projekt, von dem ein Fork stammt. (2) Im Reverse-Proxy-Kontext: der interne Backend-Server, an den der Proxy Anfragen weiterleitet.

Siehe auch: [Communities & Warum Open Source nutzen?](../kurs/01-einstieg-open-source/05-communities-und-gruende.md)

### User (Linux-Benutzer)

Ein Benutzerkonto auf einem Linux-System, unter dem Prozesse und Dienste mit eingeschränkten Berechtigungen laufen. Jede Person und jeder Dienst (z.B. `www-data` für Webserver) erhält ein eigenes Konto; nur `root` hat uneingeschränkte Rechte.

Siehe auch: [Benutzer, Gruppen und Berechtigungen](./linux_basics/03-users-groups-permissions.md)

## V

### vim (Vi IMproved)
*Synonym: vi*

Ein mächtiger, modaler Texteditor für die Kommandozeile, der auf nahezu jedem Linux-System vorinstalliert ist. vim unterscheidet zwischen Einfügemodus (Text tippen) und Normalmodus (Befehle geben); die Lernkurve ist steiler als bei nano, aber die Effizienz bei der Textbearbeitung deutlich höher.

Siehe auch: [Die Linux Shell](./linux_basics/02-linux-shell.md)

### Vaultwarden

Ein selbst gehosteter, Bitwarden-kompatibler Passwort-Manager. Vaultwarden ist ressourcenschonend und nutzt SQLite als Datenbank.

Siehe auch: [Datenbanken Überblick](./components/01-datenbanken-ueberblick.md)

### Volume (Docker)

Ein persistenter Speicherbereich, der die Lebenszeit eines Containers überlebt. Volumes ermöglichen Datenpersistenz bei Docker und sind der empfohlene Weg, Datenbank-Daten außerhalb von Containern zu speichern.

Siehe auch: [Docker Compose](./container/docker-compose.md)

### VPN (Virtual Private Network)

Ein verschlüsselter Netzwerktunnel, der sichere Verbindungen über öffentliche Netze ermöglicht. Im Self-Hosting-Kontext schützt ein VPN den Zugriff auf interne Dienste.

Siehe auch: [Firewall & UFW](./server-hardening/01-firewall-ufw.md)

### VPS (Virtual Private Server)

Eine gemietete virtuelle Maschine mit eigenem Betriebssystem und Root-Zugang. Ein VPS bietet mehr Kontrolle als Shared Hosting und ist der typische Einstieg ins Self-Hosting.

Siehe auch: [VPS und Virtualisierung](./server-and-network/06-vps.md)

## W

### Webroot

Das Verzeichnis im Dateisystem, aus dem ein Webserver statische Dateien ausliefert. In Nginx-Konfigurationen wird es mit `root` angegeben. Auch der Ablageort für ACME-Challenge-Dateien bei Let's Encrypt-Zertifikaten.

Siehe auch: [Nginx Grundlagen](./components/03-nginx-grundlagen.md)

### WebSocket

Ein Protokoll für bidirektionale Echtzeitkommunikation über Web-Verbindungen. WebSocket wird von modernen Web-Anwendungen verwendet und muss in Reverse-Proxy-Konfigurationen explizit unterstützt werden.

Siehe auch: [Reverse Proxy](./server-and-network/05-reverse-proxy.md)

### Webserver

Ein Programm, das HTTP/HTTPS-Anfragen entgegennimmt und Webinhalte ausliefert. Nginx und Apache sind die verbreitetsten Webserver im Self-Hosting.

Siehe auch: [Nginx Grundlagen](./components/03-nginx-grundlagen.md)

### Whitelist
*→ Siehe [Allowlist](#allowlist)*

### Wildcard-Zertifikat

Ein TLS-Zertifikat, das für alle Subdomains einer Domain gilt (z.B. `*.example.com`). Komplexer zu beantragen (erfordert DNS-Challenge), spart aber Zertifikatsverwaltung bei vielen Subdomains.

Siehe auch: [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

## X

### X.509

Der internationale Standard für digitale Zertifikate und die Public-Key-Infrastruktur. Alle TLS-Zertifikate sind X.509-Zertifikate.

Siehe auch: [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)

## Y

### YAML (Yet Another Markup Language)

Eine menschenfreundliche Konfigurationssprache, die leerzeichen-sensitiv ist. Docker Compose verwendet YAML für die Konfiguration von Multi-Container-Anwendungen.

Siehe auch: [Docker Compose](./container/docker-compose.md)

## Z

### zsh (Z Shell)

Eine moderne, erweiterbare Shell, die als Alternative zu Bash beliebt ist. zsh bietet verbesserte Tab-Vervollständigung, Plugins (z.B. Oh My Zsh) und Themes. Auf macOS ist zsh seit 2019 die Standard-Shell.

Siehe auch: [Die Linux Shell](./linux_basics/02-linux-shell.md)

### ZenDiS (Zentrum für Digitale Souveränität der Öffentlichen Verwaltung)

Eine deutsche Initiative zur Förderung digitaler Souveränität in der öffentlichen Verwaltung. Arbeitet an openDesk, einem Open-Source-Arbeitsplatz für Behörden.

Siehe auch: [Digitale Unabhängigkeit](../kurs/01-einstieg-open-source/01-digital-independence.md)

### Zertifizierungsstelle
*→ Siehe [CA (Certificate Authority)](#ca-certificate-authority)*

### Zertifikat (TLS-Zertifikat)

Ein digitales Dokument, das die Identität eines Servers beweist und seinen öffentlichen Schlüssel enthält. Ausgestellt von einer Certificate Authority (CA) und Grundlage für HTTPS.

Siehe auch: [SSL und Let's Encrypt](./server-and-network/04-ssl-letsencrypt.md), [TLS-Exkurs](../kurs/03-filesharing-und-groupware/06-tls-exkurs.md)
